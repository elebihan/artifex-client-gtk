//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use super::{Command, CommandStatus};
use gtk::{glib, prelude::*};
use std::cell::{Cell, RefCell};

mod imp {
    use gtk::subclass::prelude::*;
    use std::marker::PhantomData;

    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/commands_row.ui")]
    #[properties(wrapper_type = super::CommandsRow)]
    pub struct CommandsRow {
        #[template_child]
        pub timestamp_label: gtk::TemplateChild<gtk::Label>,
        #[template_child]
        pub status_stack: gtk::TemplateChild<gtk::Stack>,
        #[template_child]
        pub text_label: TemplateChild<gtk::Label>,
        #[property(
            type = String,
            set = Self::set_timestamp)]
        timestamp: PhantomData<String>,
        #[property(
            type = String,
            set = Self::set_text
            )]
        text: PhantomData<String>,
        #[property(type = CommandStatus, get, set = Self::set_status, builder(CommandStatus::default()))]
        pub status: Cell<CommandStatus>,
        #[property(get, set = Self::set_command, explicit_notify, nullable)]
        pub command: glib::WeakRef<Command>,
        pub bindings: RefCell<Vec<glib::Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CommandsRow {
        const NAME: &'static str = "CommandsRow";
        type Type = super::CommandsRow;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for CommandsRow {}
    impl WidgetImpl for CommandsRow {}
    impl ListBoxRowImpl for CommandsRow {}
    impl CommandsRow {
        fn set_status(&self, status: CommandStatus) {
            let page_name = match status {
                CommandStatus::Ko => "status-ko",
                CommandStatus::Ok => "status-ok",
                CommandStatus::Running => "status-running",
            };
            self.status_stack.set_visible_child_name(&page_name);
        }
        fn set_timestamp(&self, timestamp: &str) {
            self.timestamp_label.set_text(timestamp);
        }
        fn set_text(&self, text: &str) {
            self.text_label.set_text(text);
        }
        fn set_command(&self, command: Option<&Command>) {
            if self.command.upgrade().as_ref() == command {
                return;
            }
            for binding in self.bindings.take() {
                binding.unbind();
            }
            if let Some(command) = command {
                let timestamp_binding = command
                    .bind_property("timestamp", &*self.timestamp_label, "label")
                    .bidirectional()
                    .sync_create()
                    .build();
                let text_binding = command
                    .bind_property("text", &*self.text_label, "label")
                    .bidirectional()
                    .sync_create()
                    .build();
                let status_binding = command
                    .bind_property("status", &*self.obj(), "status")
                    .bidirectional()
                    .sync_create()
                    .build();
                self.bindings.borrow_mut().extend([
                    timestamp_binding,
                    text_binding,
                    status_binding,
                ]);
            }
            self.command.set(command);
            self.obj().notify_command();
        }
    }
}

glib::wrapper! {
    pub struct CommandsRow(ObjectSubclass<imp::CommandsRow>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl CommandsRow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
