//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use adw::{prelude::*, subclass::prelude::*};
use artifex_rpc::{ExecuteReply, ExecuteRequest};
use gettextrs::gettext;
use gtk::{gio, glib};
use tracing::{debug, error};

use crate::{
    client,
    widgets::{OperationPage, OperationPageImpl},
};

use super::{
    command::CommandDirection, Command, CommandBar, CommandStatus, CommandsRow, CommandsView,
};

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/execution_page.ui")]
    pub struct ExecutionPage {
        pub menu_button: gtk::MenuButton,
        #[template_child]
        pub execution_menu: TemplateChild<gio::Menu>,
        #[template_child]
        pub commands_view: TemplateChild<CommandsView>,
        #[template_child]
        pub command_bar: TemplateChild<CommandBar>,
        pub(crate) commands: gio::ListStore,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExecutionPage {
        const NAME: &'static str = "ExecutionPage";
        type Type = super::ExecutionPage;
        type ParentType = OperationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action_async(
                "execution-page.execute-command",
                None,
                async move |page, _, _| {
                    debug!("execution-page.execute-command");
                    page.execute_command().await
                },
            );
            klass.install_action("execution-page.clear-commands", None, move |page, _, _| {
                debug!("execution-page.clear-commands");
                page.clear_commands();
            })
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExecutionPage {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj()
                .upcast_ref::<OperationPage>()
                .set_title(&gettext("Execution"));
            self.menu_button.set_icon_name("view-more-symbolic");
            self.menu_button
                .set_tooltip_text(Some(&gettext("Command execution menu")));
            let menu_popover =
                gtk::PopoverMenu::from_model(self.execution_menu.downcast_ref::<gio::Menu>());
            self.menu_button.set_popover(Some(&menu_popover));
            let header_bar = self.obj().upcast_ref::<OperationPage>().get_header_bar();
            header_bar.pack_end(&self.menu_button);
            self.obj().setup_store();
            self.obj().setup_factory();
        }
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl Default for ExecutionPage {
        fn default() -> Self {
            Self {
                menu_button: gtk::MenuButton::new(),
                execution_menu: TemplateChild::default(),
                commands: gio::ListStore::new::<Command>(),
                command_bar: gtk::TemplateChild::default(),
                commands_view: gtk::TemplateChild::default(),
            }
        }
    }

    impl WidgetImpl for ExecutionPage {}
    impl BinImpl for ExecutionPage {}
    impl OperationPageImpl for ExecutionPage {}
}

glib::wrapper! {
    pub struct ExecutionPage(ObjectSubclass<imp::ExecutionPage>)
        @extends gtk::Widget, adw::Bin, OperationPage;
}

impl ExecutionPage {
    fn setup_store(&self) {
        let selection_model = gtk::NoSelection::new(Some(self.imp().commands.clone()));
        self.imp().commands_view.set_model(Some(&selection_model));
    }

    fn setup_factory(&self) {
        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let row = CommandsRow::new();
            let list_item = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Must be a ListItem");
            list_item.set_child(Some(&row));
        });
        factory.connect_bind(move |_, list_item| {
            let list_item = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Must be a ListItem");
            if let Some(command) = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Must be to ListItem")
                .item()
                .and_downcast_ref::<Command>()
            {
                let child = list_item
                    .downcast_ref::<gtk::ListItem>()
                    .expect("Must be a ListItem")
                    .child();
                let row = child
                    .and_downcast_ref::<CommandsRow>()
                    .expect("Child must be a CommandsRow");
                row.set_command(Some(command));
            }
        });
        factory.connect_unbind(move |_, list_item| {
            let row = list_item
                .downcast_ref::<gtk::ListItem>()
                .expect("Must be a ListItem")
                .child()
                .and_downcast::<CommandsRow>()
                .expect("Child must be a CommandsRow");
            row.set_command(None::<Command>);
        });
        self.imp().commands_view.set_factory(Some(&factory));
    }

    pub fn set_busy(&self, busy: bool) {
        self.imp().command_bar.set_sensitive(!busy);
    }

    async fn execute_command(&self) {
        let client = self.upcast_ref::<OperationPage>().imp().client.borrow();
        if let Some(client) = client.clone() {
            self.set_busy(true);
            let cmd_text = self.imp().command_bar.command();
            let cmd_input = Command::new(&cmd_text);
            self.imp().commands.append(&cmd_input);
            let (sender, receiver) =
                async_channel::bounded::<Result<tonic::Response<ExecuteReply>, tonic::Status>>(1);
            client::runtime().spawn(async move {
                let mut client = client.lock().await;
                let result = client.execute(ExecuteRequest { command: cmd_text }).await;
                sender
                    .send(result)
                    .await
                    .expect("The channel needs to be open")
            });
            while let Ok(result) = receiver.recv().await {
                match result {
                    Ok(response) => {
                        self.set_current_command_input_status(CommandStatus::Ok);
                        let reply = response.into_inner();
                        let cmd_output = Command::new(&reply.stdout);
                        cmd_output.set_direction(CommandDirection::Output);
                        let status = if reply.code == 0 {
                            CommandStatus::Ok
                        } else {
                            CommandStatus::Ko
                        };
                        cmd_output.set_status(status);
                        self.imp().commands.append(&cmd_output);
                    }
                    Err(e) => {
                        error!("Command execution failed: {}", e.to_string());
                        self.set_current_command_input_status(CommandStatus::Ko)
                    }
                }
                self.set_busy(false);
            }
        }
    }

    fn set_current_command_input_status(&self, status: CommandStatus) {
        let n_items = self.imp().commands.n_items();
        if let Some(command) = self
            .imp()
            .commands
            .item(n_items - 1)
            .and_downcast_ref::<Command>()
        {
            command.set_status(status);
        }
    }

    fn clear_commands(&self) {
        self.imp().commands.remove_all();
    }
}
