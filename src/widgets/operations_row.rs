//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use gtk::{glib, prelude::*};

mod imp {
    use std::{cell::RefCell, marker::PhantomData};

    use gtk::subclass::prelude::*;

    use super::*;

    #[derive(Debug, glib::Properties, gtk::CompositeTemplate, Default)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/operations_row.ui")]
    #[properties(wrapper_type = super::OperationsRow)]
    pub struct OperationsRow {
        #[template_child]
        pub title_label: TemplateChild<gtk::Label>,
        #[property(
            type = String,
            get = |r: &Self| r.title_label.label().to_string(),
            set = Self::set_title,
            construct)]
        title: PhantomData<String>,
        #[property(name = "page-name", get, set, construct, default = "welcome")]
        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for OperationsRow {
        const NAME: &'static str = "OperationsRow";
        type Type = super::OperationsRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for OperationsRow {}
    impl WidgetImpl for OperationsRow {}
    impl ListBoxRowImpl for OperationsRow {}

    impl OperationsRow {
        fn set_title(&self, title: &str) {
            self.title_label.set_text(title);
        }
    }
}

glib::wrapper! {
    pub struct OperationsRow(ObjectSubclass<imp::OperationsRow>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl OperationsRow {
    pub fn new(title: &str, page_name: &str) -> Self {
        glib::Object::builder()
            .property("title", title)
            .property("page-name", page_name)
            .build()
    }
}
