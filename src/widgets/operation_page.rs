//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use adw::subclass::prelude::*;
use gtk::{glib, prelude::*};
use std::cell::RefCell;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::client::Client;

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate, Default)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/operation_page.ui")]
    pub struct OperationPage {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub window_title: TemplateChild<adw::WindowTitle>,
        #[template_child]
        pub container: TemplateChild<gtk::Box>,
        pub client: RefCell<Option<Arc<Mutex<Client>>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for OperationPage {
        const NAME: &'static str = "OperationPage";
        type Type = super::OperationPage;
        type ParentType = adw::Bin;
        type Interfaces = (gtk::Buildable,);

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for OperationPage {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for OperationPage {}
    impl BinImpl for OperationPage {}
    impl BuildableImpl for OperationPage {
        fn add_child(&self, builder: &gtk::Builder, child: &glib::Object, type_: Option<&str>) {
            if self.obj().first_child().is_none() {
                self.parent_add_child(builder, child, type_);
            } else {
                self.container.append(
                    child
                        .downcast_ref::<gtk::Widget>()
                        .expect("Child is a 'Widget'"),
                );
            }
        }
    }
}

glib::wrapper! {
    pub struct OperationPage(ObjectSubclass<imp::OperationPage>)
        @extends gtk::Widget, adw::Bin,
    @implements gtk::Buildable;
}

pub trait OperationPageImpl: BinImpl {}

unsafe impl<T: OperationPageImpl> IsSubclassable<T> for OperationPage {}

impl OperationPage {
    pub fn set_title(&self, title: &str) {
        self.imp().window_title.set_title(title)
    }
    pub fn get_header_bar(&self) -> adw::HeaderBar {
        self.imp().header_bar.get()
    }
    pub fn set_client(&self, client: Option<Arc<Mutex<Client>>>) {
        *self.imp().client.borrow_mut() = client;
    }
    pub fn set_busy(&self, busy: bool) {
        self.imp().container.set_sensitive(!busy);
    }
}
