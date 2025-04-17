//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

// gtk::ListStore and gtk::EntryCompletion are deprecated since 4.10.
// There is no replacement yet [1], Nautilus and Epiphany use a combination
// of an entry and a popover, so we could go in the same direction in the future.
// [1] https://gitlab.gnome.org/GNOME/gtk/-/issues/5689
#![allow(deprecated)]

use adw::subclass::prelude::*;
use gtk::{glib, prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/connection_bar.ui")]
    pub struct ConnectionBar {
        #[template_child]
        pub url_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub connect_button: TemplateChild<gtk::Button>,
        pub url_cache: gtk::ListStore,
        #[template_child]
        pub url_popover: TemplateChild<gtk::Popover>,
        #[template_child]
        pub url_popover_label: TemplateChild<gtk::Label>,
    }

    impl Default for ConnectionBar {
        fn default() -> Self {
            Self {
                url_entry: TemplateChild::default(),
                connect_button: TemplateChild::default(),
                url_cache: gtk::ListStore::new(&[glib::Type::STRING]),
                url_popover: TemplateChild::default(),
                url_popover_label: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ConnectionBar {
        const NAME: &'static str = "ConnectionBar";
        type Type = super::ConnectionBar;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ConnectionBar {
        fn constructed(&self) {
            self.parent_constructed();
            let completion = gtk::EntryCompletion::new();
            completion.set_model(Some(&self.url_cache));
            completion.set_text_column(0);
            completion.set_minimum_key_length(1);
            self.url_entry.set_completion(Some(&completion));
        }
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for ConnectionBar {}
    impl BinImpl for ConnectionBar {}
}

glib::wrapper! {
    pub struct ConnectionBar(ObjectSubclass<imp::ConnectionBar>)
        @extends gtk::Widget, adw::Bin;
}

impl ConnectionBar {
    /// Tell whether to allow connection or not.
    pub fn enable_connection(&self, enabled: bool) {
        let css_classes = ["suggested-action", "destructive-action"];
        let (icon_name, to_add, to_del) = if enabled {
            ("media-playback-start-symbolic", 0, 1)
        } else {
            ("media-playback-stop-symbolic", 1, 0)
        };
        self.imp().connect_button.set_icon_name(icon_name);
        self.imp()
            .connect_button
            .remove_css_class(css_classes[to_del]);
        self.imp().connect_button.add_css_class(css_classes[to_add]);
        self.imp().url_entry.set_sensitive(enabled);
    }
    /// Get the value of the URL entry.
    pub fn get_url(&self) -> String {
        self.imp().url_entry.buffer().text().to_string()
    }
    /// Set the list of recently used URLs.
    pub fn set_recent_urls(&self, urls: &[&str]) {
        let store = &self.imp().url_cache;
        for url in urls {
            store.set(&store.append(), &[(0, url)]);
        }
    }
    /// Get the list of recently used URLs.
    pub fn get_recent_urls(&self) -> Vec<String> {
        let mut urls = Vec::<String>::new();
        self.imp().url_cache.foreach(|model, _, iter| {
            if let Ok(url) = model.get_value(iter, 0).get::<String>() {
                urls.push(url);
            }
            false
        });
        urls
    }
    /// Add an URL to the list of recent URLS.
    pub fn add_recent_url(&self, url: &str) {
        let store = &self.imp().url_cache;
        let mut exists = false;
        self.imp().url_cache.foreach(|model, _, iter| {
            if let Ok(item) = model.get_value(iter, 0).get::<&str>() {
                if url == item {
                    exists = true;
                    return true;
                }
            }
            false
        });
        if !exists {
            store.set(&store.append(), &[(0, &url)]);
        }
    }
    /// Show URL popover to display `message`.
    pub fn show_popover(&self, message: &str) {
        self.imp().url_popover.popup();
        self.imp().url_popover_label.set_text(message);
    }
}
