//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use adw::subclass::prelude::*;
use gtk::{glib, prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/command_bar.ui")]
    pub struct CommandBar {
        #[template_child]
        pub command_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub execute_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub command_popover: TemplateChild<gtk::Popover>,
        #[template_child]
        pub command_popover_label: TemplateChild<gtk::Label>,
    }

    impl Default for CommandBar {
        fn default() -> Self {
            Self {
                command_entry: TemplateChild::default(),
                execute_button: TemplateChild::default(),
                command_popover: TemplateChild::default(),
                command_popover_label: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CommandBar {
        const NAME: &'static str = "CommandBar";
        type Type = super::CommandBar;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CommandBar {
        fn constructed(&self) {
            self.parent_constructed();
            self.command_entry
                .bind_property("text", &*self.execute_button, "sensitive")
                .sync_create()
                .transform_to(|_, text: String| Some(!text.is_empty()))
                .build();
        }
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for CommandBar {}
    impl BinImpl for CommandBar {}
}

glib::wrapper! {
    pub struct CommandBar(ObjectSubclass<imp::CommandBar>)
        @extends gtk::Widget, adw::Bin;
}

impl CommandBar {
    /// Show COMMAND popover to display `message`.
    pub fn show_popover(&self, message: &str) {
        self.imp().command_popover.popup();
        self.imp().command_popover_label.set_text(message);
    }
    /// Get the command to execute.
    pub fn command(&self) -> String {
        self.imp().command_entry.buffer().text().to_string()
    }
}
