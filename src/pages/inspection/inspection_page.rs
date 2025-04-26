//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use adw::{prelude::*, subclass::prelude::*};
use artifex_rpc::{InspectReply, InspectRequest};
use gettextrs::gettext;
use gtk::glib;
use humantime::format_duration;
use std::time::Duration;
use tracing::debug;

use crate::{
    client,
    widgets::{OperationPage, OperationPageImpl},
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/inspection_page.ui")]
    pub struct InspectionPage {
        pub refresh_button: gtk::Button,
        #[template_child]
        pub activity_spinner: TemplateChild<adw::Spinner>,
        #[template_child]
        pub kernel_version_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub system_uptime_row: TemplateChild<adw::ActionRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InspectionPage {
        const NAME: &'static str = "InspectionPage";
        type Type = super::InspectionPage;
        type ParentType = OperationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action_async("inspection-page.refresh", None, |page, _, _| async move {
                debug!("InspectionPage::inspection-page.refresh");
                page.refresh().await
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for InspectionPage {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj()
                .upcast_ref::<OperationPage>()
                .set_title(&gettext("Inspection"));
            self.refresh_button.set_icon_name("view-refresh-symbolic");
            self.refresh_button
                .set_tooltip_text(Some(&gettext("Refresh information")));
            self.refresh_button
                .set_action_name(Some("inspection-page.refresh"));
            let toolbar = gtk::Box::builder().build();
            toolbar.append(&self.refresh_button);
            let header_bar = self.obj().upcast_ref::<OperationPage>().get_header_bar();
            header_bar.pack_end(&toolbar);
        }
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for InspectionPage {}
    impl BinImpl for InspectionPage {}
    impl OperationPageImpl for InspectionPage {}
}

glib::wrapper! {
    pub struct InspectionPage(ObjectSubclass<imp::InspectionPage>)
        @extends gtk::Widget, adw::Bin, OperationPage;
}

impl InspectionPage {
    pub async fn refresh(&self) {
        let client = self.upcast_ref::<OperationPage>().imp().client.borrow();
        if let Some(client) = client.clone() {
            self.set_busy(true);
            let (sender, receiver) =
                async_channel::bounded::<Result<tonic::Response<InspectReply>, tonic::Status>>(1);
            client::runtime().spawn(async move {
                let mut client = client.lock().await;
                let result = client.inspect(InspectRequest {}).await;
                sender
                    .send(result)
                    .await
                    .expect("The channel needs to be open")
            });
            while let Ok(result) = receiver.recv().await {
                match result {
                    Ok(response) => {
                        let reply = response.into_inner();
                        self.imp()
                            .kernel_version_row
                            .set_subtitle(&reply.kernel_version);
                        self.imp().system_uptime_row.set_subtitle(
                            &format_duration(Duration::from_secs(reply.system_uptime)).to_string(),
                        );
                    }
                    Err(_) => {}
                }
            }
            self.set_busy(false);
        }
    }
    pub fn set_busy(&self, busy: bool) {
        self.upcast_ref::<OperationPage>().set_busy(busy);
        self.imp().activity_spinner.set_visible(busy);
        self.imp().refresh_button.set_sensitive(!busy);
    }
}
