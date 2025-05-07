//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use gettextrs::gettext;
use gtk::{
    gio,
    glib::{self, clone},
    prelude::*,
};
use tracing::{debug, error};

mod imp {
    use super::*;
    use gtk::subclass::prelude::*;
    use std::cell::RefCell;

    #[derive(Debug, glib::Properties)]
    #[properties(wrapper_type = super::CertChooser)]
    pub struct CertChooser {
        #[property(name = "uri", get, set)]
        pub uri: RefCell<String>,
        pub label: gtk::Label,
        pub button: gtk::Button,
    }

    impl Default for CertChooser {
        fn default() -> Self {
            Self {
                uri: RefCell::new(String::new()),
                label: gtk::Label::default(),
                button: gtk::Button::builder()
                    .icon_name("folder-open-symbolic")
                    .has_frame(false)
                    .tooltip_text(&gettext("Select PEM file"))
                    .build(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CertChooser {
        const NAME: &'static str = "CertChooser";
        type Type = super::CertChooser;
        type ParentType = gtk::Box;

        fn class_init(_klass: &mut Self::Class) {}

        fn instance_init(_obj: &glib::subclass::InitializingObject<Self>) {}
    }

    #[glib::derived_properties]
    impl ObjectImpl for CertChooser {
        fn constructed(&self) {
            self.parent_constructed();
            self.label.set_hexpand(true);
            self.label.set_halign(gtk::Align::End);
            self.label.set_has_tooltip(true);
            self.label.connect_query_tooltip(clone!(
                #[weak(rename_to = this)]
                self,
                #[upgrade_or]
                false,
                move |_, _, _, _, tooltip| {
                    let uri = this.obj().uri();
                    if uri.is_empty() {
                        false
                    } else {
                        tooltip.set_text(Some(&uri));
                        true
                    }
                }
            ));
            self.button.connect_clicked(clone!(
                #[weak(rename_to = this)]
                self,
                move |_| {
                    glib::spawn_future_local(async move {
                        this.select_from_file().await;
                    });
                }
            ));
            self.obj().set_hexpand(true);
            self.obj().set_spacing(6);
            self.obj().append(&self.label);
            self.obj()
                .bind_property("uri", &self.label, "label")
                .transform_to(|_, uri: String| {
                    glib::Uri::parse(&uri, glib::UriFlags::NONE)
                        .ok()
                        .and_then(|uri| {
                            let path = uri.path();
                            path.rsplit_once('/')
                                .map(|(_, file_name)| file_name)
                                .or(Some(&path))
                                .map(String::from)
                        })
                        .or(Some(gettext("None")))
                })
                .sync_create()
                .build();
            self.obj().append(&self.button);
        }
        fn dispose(&self) {}
    }

    impl WidgetImpl for CertChooser {}
    impl BoxImpl for CertChooser {}

    impl CertChooser {
        async fn select_from_file(&self) {
            let filter = gtk::FileFilter::new();
            filter.add_pattern("*.pem");
            filter.set_name(Some(&gettext("PEM files")));
            let dialog = gtk::FileDialog::builder()
                .modal(true)
                .title(&gettext("Select PEM file"))
                .default_filter(&filter)
                .build();
            match dialog
                .open_future(self.obj().root().and_downcast_ref::<gtk::Window>())
                .await
            {
                Ok(file) => {
                    debug!("Opening {:?}", file.path());
                    self.set_uri_from_file(file);
                }
                Err(e) => {
                    error!("Failed to open file ({})", e.to_string());
                }
            }
        }

        fn set_uri_from_file(&self, file: gio::File) {
            if let Err(e) = file
                .path()
                .ok_or_else(|| gettext("Invalid file"))
                .and_then(|path| {
                    glib::filename_to_uri(path, None).map_err(|e| e.message().to_string())
                })
                .map(|uri| self.obj().set_uri(uri))
            {
                error!("Failed to set URI ({})", e.to_string());
            }
        }
    }
}

glib::wrapper! {
    pub struct CertChooser(ObjectSubclass<imp::CertChooser>)
        @extends gtk::Widget, gtk::Box;
}

impl CertChooser {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
