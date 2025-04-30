//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use glib::clone;
use gtk::{glib, prelude::*, subclass::prelude::*};
use std::cell::Cell;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/commands_view.ui")]
    #[properties(wrapper_type = super::CommandsView)]
    pub struct CommandsView {
        #[template_child]
        pub list_view: TemplateChild<gtk::ListView>,
        #[template_child]
        pub search_bar: gtk::TemplateChild<gtk::SearchBar>,
        #[template_child]
        pub search_entry: gtk::TemplateChild<gtk::SearchEntry>,
        #[property(get, set)]
        search_mode_enabled: Cell<bool>,
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

    #[glib::derived_properties]
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

    pub fn scroll_to_last(&self) {
        let model = self
            .imp()
            .list_view
            .model()
            .expect("CommandsView must have a SelectionModel");
        let n_items = model.n_items();
        if n_items > 0 {
            // FIXME: This is an ugly hack.
            // One could think of using glib::idle_add_local_once(),
            // but as this function is called from an async function,
            // this won't work.
            // So use a timeout value "good enough" for small commamd
            // output.
            glib::timeout_add_local_once(
                std::time::Duration::from_millis(1000),
                clone!(
                    #[weak(rename_to = this)]
                    self,
                    move || {
                        this.imp().list_view.scroll_to(
                            n_items - 1,
                            gtk::ListScrollFlags::FOCUS,
                            None,
                        );
                    }
                ),
            );
        }
    }

    pub fn search_entry(&self) -> gtk::SearchEntry {
        self.imp().search_entry.get()
    }

    pub fn toggle_search_bar(&self) {
        let search_bar = self.imp().search_bar.get();
        search_bar.set_search_mode(!search_bar.is_search_mode());
    }
}
