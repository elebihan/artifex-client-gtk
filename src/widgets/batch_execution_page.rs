//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use adw::{prelude::*, subclass::prelude::*};
use artifex_batch::{Batch, BatchReport, BatchRunner, MarkupKind, MarkupReportRenderer};
use gettextrs::gettext;
use gtk::{gio, glib};
use tracing::{debug, error};

use crate::{
    client,
    widgets::{OperationPage, OperationPageImpl},
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/elebihan/artifex-client-gtk/ui/batch_execution_page.ui")]
    pub struct BatchExecutionPage {
        pub open_button: gtk::Button,
        pub exec_button: gtk::Button,
        #[template_child]
        pub input_text_view: TemplateChild<gtk::TextView>,
        #[template_child]
        pub output_view: TemplateChild<gtk::Box>,
        #[template_child]
        pub output_text_view: TemplateChild<gtk::TextView>,
        pub activity_spinner: adw::Spinner,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BatchExecutionPage {
        const NAME: &'static str = "BatchExecutionPage";
        type Type = super::BatchExecutionPage;
        type ParentType = OperationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action_async(
                "batch-execution-page.open-batch",
                None,
                async move |page, _, _| {
                    debug!("batch-execution-page.open-batch");
                    page.open_batch().await
                },
            );
            klass.install_action_async(
                "batch-execution-page.exec-batch",
                None,
                async move |page, _, _| {
                    debug!("batch-execution-page.exec-batch");
                    page.exec_batch().await
                },
            );
            klass.install_action(
                "batch-execution-page.copy-output",
                None,
                move |page, _, _| {
                    debug!("batch-execution-page.copy-output");
                    page.copy_output()
                },
            )
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BatchExecutionPage {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj()
                .upcast_ref::<OperationPage>()
                .set_title(&gettext("Batch Execution"));
            self.open_button.set_icon_name("folder-open-symbolic");
            self.open_button
                .set_tooltip_text(Some(&gettext("Open batch file")));
            self.open_button
                .set_action_name(Some("batch-execution-page.open-batch"));
            let header_bar = self.obj().upcast_ref::<OperationPage>().get_header_bar();
            header_bar.pack_start(&self.open_button);
            self.activity_spinner.set_visible(false);
            self.exec_button
                .set_icon_name("media-playback-start-symbolic");
            self.exec_button
                .set_tooltip_text(Some(&gettext("Execute batch")));
            self.exec_button
                .set_action_name(Some("batch-execution-page.exec-batch"));
            let exec_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
            exec_box.append(&self.activity_spinner);
            exec_box.append(&self.exec_button);
            header_bar.pack_end(&exec_box);
            self.obj().enable_input_actions(false);
            self.obj().enable_output_actions(false);
        }
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for BatchExecutionPage {}
    impl BinImpl for BatchExecutionPage {}
    impl OperationPageImpl for BatchExecutionPage {}
}

glib::wrapper! {
    pub struct BatchExecutionPage(ObjectSubclass<imp::BatchExecutionPage>)
        @extends gtk::Widget, adw::Bin, OperationPage;
}

impl BatchExecutionPage {
    async fn open_batch(&self) {
        let filter = gtk::FileFilter::new();
        filter.add_pattern("*.txt");
        filter.set_name(Some(&gettext("Batch files")));
        let dialog = gtk::FileDialog::builder()
            .modal(true)
            .title(&gettext("Select batch file"))
            .default_filter(&filter)
            .build();
        match dialog
            .open_future(self.root().and_downcast_ref::<gtk::Window>())
            .await
        {
            Ok(file) => {
                debug!("Opening {:?}", file.path());
                self.load_batch(file)
            }
            Err(e) => {
                error!("Failed to open file: {e}")
            }
        }
    }

    fn load_batch(&self, file: gio::File) {
        match file.path().ok_or_else(|| gettext("Invalid file")) {
            Ok(path) => {
                match path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| gettext("Invalid file name"))
                {
                    Ok(file_name) => {
                        self.upcast_ref::<OperationPage>()
                            .imp()
                            .window_title
                            .set_subtitle(&file_name);
                        match std::fs::read_to_string(path) {
                            Ok(text) => {
                                self.imp().input_text_view.buffer().set_text(&text);
                                self.imp().output_text_view.buffer().set_text("");
                                self.enable_input_actions(true);
                                self.enable_output_actions(false);
                            }
                            Err(e) => self
                                .upcast_ref::<OperationPage>()
                                .show_error(&gettext("Failed to read file"), &e.to_string()),
                        }
                    }
                    Err(e) => self
                        .upcast_ref::<OperationPage>()
                        .show_error(&gettext("Failed to load batch"), &e.to_string()),
                }
            }
            Err(e) => self
                .upcast_ref::<OperationPage>()
                .show_error(&gettext("Failed to load batch"), &e.to_string()),
        }
    }

    fn enable_input_actions(&self, enabled: bool) {
        self.imp().exec_button.set_sensitive(enabled);
        self.imp().input_text_view.set_sensitive(enabled);
    }

    fn enable_output_actions(&self, enabled: bool) {
        self.imp().output_view.set_sensitive(enabled);
    }

    pub fn set_busy(&self, busy: bool) {
        self.upcast_ref::<OperationPage>().set_busy(busy);
        self.imp().activity_spinner.set_visible(busy);
        self.enable_input_actions(!busy);
    }

    async fn exec_batch(&self) {
        self.imp().output_text_view.buffer().set_text("");
        let buffer = self.imp().input_text_view.buffer();
        match Batch::from_reader(
            buffer
                .text(&buffer.start_iter(), &buffer.end_iter(), false)
                .as_str()
                .as_bytes(),
        ) {
            Ok(batch) => self.run_batch(batch).await,
            Err(e) => self
                .upcast_ref::<OperationPage>()
                .show_error(&gettext("Invalid batch"), &e.to_string()),
        }
    }

    async fn run_batch(&self, batch: Batch) {
        let client = self.upcast_ref::<OperationPage>().imp().client.borrow();
        if let Some(client) = client.clone() {
            self.set_busy(true);
            let (sender, receiver) =
                async_channel::bounded::<Result<BatchReport, artifex_batch::Error>>(1);
            client::runtime().spawn(async move {
                let mut client = client.lock().await;
                let mut runner = BatchRunner::new(&mut client);
                let result = runner.run(&batch).await;
                sender
                    .send(result)
                    .await
                    .expect("The channel needs to be open")
            });
            while let Ok(result) = receiver.recv().await {
                match result {
                    Ok(report) => {
                        let mut buffer = Vec::new();
                        let renderer = MarkupReportRenderer::new(MarkupKind::Yaml);
                        match renderer
                            .render(&mut buffer, &report)
                            .map_err(|e| e.to_string())
                            .and_then(|()| String::from_utf8(buffer).map_err(|e| e.to_string()))
                        {
                            Ok(text) => self.imp().output_text_view.buffer().set_text(&text),
                            Err(e) => self
                                .upcast_ref::<OperationPage>()
                                .show_error(&gettext("Failed to render report"), &e.to_string()),
                        }
                    }
                    Err(e) => self
                        .upcast_ref::<OperationPage>()
                        .show_error(&gettext("Failed to run batch"), &e.to_string()),
                }
            }
            self.set_busy(false);
            self.enable_output_actions(true);
        }
    }

    fn copy_output(&self) {
        let clipboard = self.clipboard();
        let buffer = self.imp().output_text_view.buffer();
        let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
        clipboard.set_text(&text);
        let toast = adw::Toast::builder()
            .title(&gettext("Batch output copied"))
            .build();
        self.upcast_ref::<OperationPage>()
            .imp()
            .toast_overlay
            .add_toast(toast);
    }
}
