//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use adw::{prelude::*, subclass::prelude::*};
use artifex_rpc::{upgrade_reply::Status, UpgradeReply, UpgradeRequest};
use futures_util::StreamExt;
use gettextrs::gettext;
use glib::clone;
use gtk::glib;
use std::cell::Cell;
use tracing::debug;

use crate::{
    client,
    widgets::{OperationPage, OperationPageImpl},
};

#[derive(Clone, Copy, Debug, glib::Enum, PartialEq, Default)]
#[enum_type(name = "UpgradeState")]
#[repr(u32)]
pub enum UpgradeState {
    Failed,
    Running,
    Succeed,
    #[default]
    Waiting,
}

mod imp {
    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/upgrade_page.ui")]
    #[properties(wrapper_type = super::UpgradePage)]
    pub struct UpgradePage {
        pub start_button: gtk::Button,
        #[template_child]
        pub upgrade_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub upgrade_progress_bar: TemplateChild<gtk::ProgressBar>,
        #[property(name = "upgrade-state", type = UpgradeState, get, set, builder(UpgradeState::default()))]
        pub upgrade_state: Cell<UpgradeState>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UpgradePage {
        const NAME: &'static str = "UpgradePage";
        type Type = super::UpgradePage;
        type ParentType = OperationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action_async("upgrade-page.start", None, |page, _, _| async move {
                debug!("UpgradePage::upgrade-page.start");
                page.start().await
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for UpgradePage {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj()
                .upcast_ref::<OperationPage>()
                .set_title(&gettext("Upgrade"));
            self.start_button
                .set_icon_name("media-playback-start-symbolic");
            self.start_button
                .set_tooltip_text(Some(&gettext("Start upgrade")));
            self.start_button
                .set_action_name(Some("upgrade-page.start"));
            let toolbar = gtk::Box::builder().build();
            toolbar.append(&self.start_button);
            let header_bar = self.obj().upcast_ref::<OperationPage>().get_header_bar();
            header_bar.pack_end(&toolbar);
            self.obj()
                .bind_property("upgrade-state", &*self.upgrade_label, "label")
                .transform_to(Self::upgrade_state_to_label)
                .build();
            self.obj()
                .bind_property("upgrade-state", &*self.upgrade_progress_bar, "visible")
                .transform_to(|_, s| Some(!matches!(s, UpgradeState::Waiting)))
                .build();
            self.obj().connect_notify_local(
                Some("upgrade-state"),
                clone!(
                    #[weak(rename_to = this)]
                    self,
                    move |_, _| {
                        match this.upgrade_state.get() {
                            UpgradeState::Failed => {
                                this.upgrade_progress_bar.add_css_class("failure");
                            }
                            UpgradeState::Succeed => {
                                this.upgrade_progress_bar.add_css_class("success");
                            }
                            UpgradeState::Running | UpgradeState::Waiting => {
                                this.upgrade_progress_bar.remove_css_class("failure");
                                this.upgrade_progress_bar.remove_css_class("success");
                            }
                        }
                    }
                ),
            );
        }
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for UpgradePage {}
    impl BinImpl for UpgradePage {}
    impl OperationPageImpl for UpgradePage {}
    impl UpgradePage {
        fn upgrade_state_to_label(_binding: &glib::Binding, state: UpgradeState) -> Option<String> {
            let label = match state {
                UpgradeState::Failed => gettext("Upgrade failed"),
                UpgradeState::Running => gettext("Upgrade in progress..."),
                UpgradeState::Succeed => gettext("Upgrade succeed"),
                UpgradeState::Waiting => gettext("Press \"start\" to start the upgrade"),
            };
            Some(label)
        }
    }
}

glib::wrapper! {
    pub struct UpgradePage(ObjectSubclass<imp::UpgradePage>)
        @extends gtk::Widget, adw::Bin, OperationPage;
}

impl UpgradePage {
    pub async fn start(&self) {
        let client = self.upcast_ref::<OperationPage>().imp().client.borrow();
        if let Some(client) = client.clone() {
            self.set_busy(true);
            self.imp().upgrade_progress_bar.set_fraction(0.0);
            let (sender, receiver) = async_channel::bounded::<
                Result<tonic::Response<tonic::Streaming<UpgradeReply>>, tonic::Status>,
            >(1);
            client::runtime().spawn(async move {
                let mut client = client.lock().await;
                let result = client.upgrade(UpgradeRequest {}).await;
                sender
                    .send(result)
                    .await
                    .expect("The channel needs to be open")
            });
            while let Ok(result) = receiver.recv().await {
                match result {
                    Ok(response) => {
                        self.set_upgrade_state(UpgradeState::Running);
                        let mut stream = response.into_inner();
                        while let Some(reply) = stream.next().await {
                            if let Ok(progress) = reply {
                                let progress_bar = &self.imp().upgrade_progress_bar;
                                match progress.status() {
                                    Status::Running => {
                                        progress_bar.set_fraction(progress.position as f64 / 100.0);
                                    }
                                    Status::Failure => {
                                        self.set_upgrade_state(UpgradeState::Failed);
                                    }
                                    Status::Success => {
                                        self.set_upgrade_state(UpgradeState::Succeed);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => self
                        .upcast_ref::<OperationPage>()
                        .show_error(&gettext("Upgrade failed"), &e.to_string()),
                }
            }
            self.set_busy(false);
        }
    }
    pub fn set_busy(&self, busy: bool) {
        self.imp().start_button.set_sensitive(!busy);
    }
}
