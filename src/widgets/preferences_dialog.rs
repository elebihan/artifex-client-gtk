//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use crate::config::APP_ID;
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};

mod imp {
    use super::*;
    use crate::widgets::CertChooser;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/preferences_dialog.ui")]
    pub struct PreferencesDialog {
        #[template_child]
        pub root_cert_chooser: gtk::TemplateChild<CertChooser>,
        #[template_child]
        pub client_cert_chooser: gtk::TemplateChild<CertChooser>,
        #[template_child]
        pub client_key_chooser: gtk::TemplateChild<CertChooser>,
        pub settings: gio::Settings,
    }

    impl Default for PreferencesDialog {
        fn default() -> Self {
            Self {
                root_cert_chooser: gtk::TemplateChild::default(),
                client_cert_chooser: gtk::TemplateChild::default(),
                client_key_chooser: gtk::TemplateChild::default(),
                settings: gio::Settings::new(&format!("{APP_ID}.connection")),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesDialog {
        const NAME: &'static str = "PreferencesDialog";
        type Type = super::PreferencesDialog;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            CertChooser::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PreferencesDialog {
        fn constructed(&self) {
            self.parent_constructed();
            self.settings
                .bind("auth-root-cert-uri", &self.root_cert_chooser.get(), "uri")
                .build();
            self.settings
                .bind(
                    "auth-client-cert-uri",
                    &self.client_cert_chooser.get(),
                    "uri",
                )
                .build();
            self.settings
                .bind("auth-client-key-uri", &self.client_key_chooser.get(), "uri")
                .build();
        }
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for PreferencesDialog {}
    impl AdwDialogImpl for PreferencesDialog {}
    impl PreferencesDialogImpl for PreferencesDialog {}
}

glib::wrapper! {
    pub struct PreferencesDialog(ObjectSubclass<imp::PreferencesDialog>)
        @extends gtk::Widget, adw::Dialog, adw::PreferencesDialog;
}

impl PreferencesDialog {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
