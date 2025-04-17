//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use gtk::{glib, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/commands_view.ui")]
    pub struct CommandsView {
        #[template_child]
        pub list_view: TemplateChild<gtk::ListView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CommandsView {
        const NAME: &'static str = "CommandsView";
        type Type = super::CommandsView;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CommandsView {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for CommandsView {}
    impl BoxImpl for CommandsView {}
}

glib::wrapper! {
    pub struct CommandsView(ObjectSubclass<imp::CommandsView>)
        @extends gtk::Widget, gtk::Box;
}

impl CommandsView {
    pub fn set_model(&self, model: Option<&impl glib::object::IsA<gtk::SelectionModel>>) {
        self.imp().list_view.set_model(model)
    }

    pub fn set_factory(&self, factory: Option<&impl glib::object::IsA<gtk::ListItemFactory>>) {
        self.imp().list_view.set_factory(factory)
    }
}
