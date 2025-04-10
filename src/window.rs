//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::prelude::*;
use gtk::{
    gio,
    glib::{self, clone},
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

use crate::application::Application;
use crate::client::{self, Client};
use crate::config::{APP_ID, PROFILE};
use crate::i18n::i18n;
use crate::widgets::{
    ConnectionBar, ConnectionStatusPage, InspectionPage, OperationPage, OperationsRow,
};

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/window.ui")]
    pub struct Window {
        #[template_child]
        pub client_view: TemplateChild<adw::NavigationSplitView>,
        #[template_child]
        pub connection_bar: TemplateChild<ConnectionBar>,
        #[template_child]
        pub operations_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub connection_status_page: TemplateChild<ConnectionStatusPage>,
        #[template_child]
        pub inspection_page: TemplateChild<InspectionPage>,
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        pub settings: gio::Settings,
    }

    impl Default for Window {
        fn default() -> Self {
            Self {
                client_view: TemplateChild::default(),
                connection_bar: TemplateChild::default(),
                operations_list: TemplateChild::default(),
                connection_status_page: TemplateChild::default(),
                inspection_page: TemplateChild::default(),
                stack: TemplateChild::default(),
                settings: gio::Settings::new(APP_ID),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            OperationsRow::static_type();
            ConnectionBar::static_type();

            klass.install_action_async("win.toggle-connection", None, |win, _, _| async move {
                debug!("Window::win.toggle-connection");
                win.toggle_connection().await
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            let row = self
                .operations_list
                .row_at_index(0)
                .expect("Operations not empty");
            self.operations_list.unselect_row(&row);
            self.operations_list.connect_row_activated(clone!(
                #[weak(rename_to = win)]
                obj,
                move |_, row| {
                    win.operations_row_selected(row);
                }
            ));
            obj.enable_connection(true);
            obj.load_window_size();
            obj.load_urls();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for Window {}
    impl WindowImpl for Window {
        fn close_request(&self) -> glib::Propagation {
            if let Err(err) = self.obj().save_window_size() {
                warn!("Failed to save window state, {}", &err);
            }
            if let Err(err) = self.obj().save_urls() {
                warn!("Failed to save URL cache, {}", &err)
            }
            self.parent_close_request()
        }
    }

    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        let win = glib::Object::builder::<Window>()
            .property("application", app)
            .build();
        win.setup_pages();
        win
    }

    fn load_window_size(&self) {
        let imp = self.imp();
        let width = imp.settings.int("window-width");
        let height = imp.settings.int("window-height");
        let is_maximized = imp.settings.boolean("is-maximized");
        self.set_default_size(width, height);
        if is_maximized {
            self.maximize();
        }
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();
        let (width, height) = self.default_size();
        imp.settings.set_int("window-width", width)?;
        imp.settings.set_int("window-height", height)?;
        imp.settings
            .set_boolean("is-maximized", self.is_maximized())?;
        Ok(())
    }

    fn load_urls(&self) {
        let imp = self.imp();
        let urls = imp.settings.strv("recent-urls");
        let urls: Vec<&str> = urls.iter().map(|s| s.as_str()).collect();
        imp.connection_bar.set_recent_urls(&urls);
    }

    fn save_urls(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();
        let urls = imp.connection_bar.get_recent_urls();
        imp.settings.set_strv("recent-urls", urls)?;
        Ok(())
    }

    fn setup_pages(&self) {
        if let Some(client) = self.client() {
            self.imp()
                .inspection_page
                .upcast_ref::<OperationPage>()
                .set_client(client);
        }
    }

    fn operations_row_selected(&self, row: &gtk::ListBoxRow) {
        let imp = self.imp();
        let operations_row = row
            .downcast_ref::<OperationsRow>()
            .expect("Must be a 'OperationsRow'");
        imp.client_view.set_show_content(true);
        let page_name = operations_row.page_name();
        imp.stack.set_visible_child_name(&page_name);
    }

    pub fn select_page(&self, name: &str) {
        self.imp().stack.set_visible_child_name(name);
    }

    pub fn client(&self) -> Option<Arc<Mutex<Client>>> {
        self.application().map(|app| {
            app.downcast::<crate::application::Application>()
                .expect("Not an Application")
                .client()
        })
    }

    fn enable_connection(&self, enabled: bool) {
        let imp = self.imp();
        imp.connection_bar.enable_connection(enabled);
        imp.stack.set_visible_child_name("connection-status");
        imp.connection_status_page.set_successful(!enabled);
        imp.operations_list.set_sensitive(!enabled);
    }

    pub async fn toggle_connection(&self) {
        if let Some(client) = self.client() {
            let is_connected = client.lock().await.connected();
            let connection_bar = &self.imp().connection_bar;
            let url = connection_bar.get_url();
            if url.is_empty() || !url.starts_with("http://") {
                return;
            }
            let url = Arc::new(url);
            let (sender, receiver) = async_channel::bounded::<Result<(), client::Error>>(1);
            client::runtime().spawn(clone!(
                #[weak]
                url,
                #[weak]
                client,
                async move {
                    let client = client.lock().await;
                    let result = if is_connected {
                        client.disconnect().await
                    } else {
                        client.connect(&url).await
                    };
                    sender
                        .send(result)
                        .await
                        .expect("The channel needs to be open")
                }
            ));
            while let Ok(response) = receiver.recv().await {
                match response {
                    Err(e) => {
                        let (message, details) = if is_connected {
                            ("Disconnection failed", "Failed to disconnect from {}: {}")
                        } else {
                            ("Connection failed", "Failed to connect to {}: {}")
                        };
                        let details = i18n(details, &[&url, &e.to_string()]);
                        let dialog = gtk::AlertDialog::builder()
                            .modal(true)
                            .message(&gettext(message))
                            .detail(&details)
                            .buttons([glib::GString::from(gettext("Ok"))])
                            .build();
                        dialog.show(Some(&*self));
                        error!(details);
                    }
                    Ok(_) if is_connected => {
                        info!("Disconnected")
                    }
                    Ok(_) if !is_connected => {
                        info!("Connected to {}", url);
                        self.imp().connection_bar.add_recent_url(&url);
                    }
                    _ => unreachable!(),
                }
            }
            self.enable_connection(!client.lock().await.connected());
        }
    }
}
