//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::Properties;
use gtk::{glib, prelude::*};

use crate::widgets::{OperationPage, OperationPageImpl};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::ConnectionStatusPage)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/connection_status_page.ui")]
    pub struct ConnectionStatusPage {
        #[template_child]
        pub status_page: TemplateChild<adw::StatusPage>,
        #[property(get, set = Self::set_successful, construct, default = false)]
        pub successful: bool,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ConnectionStatusPage {
        const NAME: &'static str = "ConnectionStatusPage";
        type Type = super::ConnectionStatusPage;
        type ParentType = OperationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for ConnectionStatusPage {
        fn constructed(&self) {
            self.parent_constructed();
        }
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl ConnectionStatusPage {
        fn set_successful(&self, successful: bool) {
            let (icon_name, title, description) = if successful {
                (
                    "face-smile-symbolic",
                    "Connected",
                    "Connection to server established.",
                )
            } else {
                (
                    "face-sad-symbolic",
                    "Not connected",
                    "Enter server address and press \"Connect\".",
                )
            };
            self.status_page.set_icon_name(Some(icon_name));
            self.status_page.set_title(&gettext(title));
            self.status_page
                .set_description(Some(&gettext(description)));
        }
    }
    impl WidgetImpl for ConnectionStatusPage {}
    impl BinImpl for ConnectionStatusPage {}
    impl OperationPageImpl for ConnectionStatusPage {}
}

glib::wrapper! {
    pub struct ConnectionStatusPage(ObjectSubclass<imp::ConnectionStatusPage>)
        @extends gtk::Widget, adw::Bin, OperationPage;
}
