//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

mod config {
    #![allow(dead_code)]

    include!(concat!(env!("CODEGEN_BUILD_DIR"), "/config.rs"));
}

mod application;
mod client;
mod i18n;
mod widgets;
mod window;

use std::env;
use std::path::PathBuf;

use gettextrs::LocaleCategory;
use gtk::{gio, glib};

use self::application::Application;
use self::config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};

fn main() -> glib::ExitCode {
    tracing_subscriber::fmt::init();

    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name("artifex-client-gtk");

    let path = if env::var("MESON_DEVENV").is_ok() {
        let mut path = env::current_exe().expect("Couldn't find current executable name");
        path.pop();
        path.pop();
        path.push("data");
        path.push("resources");
        path.push("resources.gresource");
        path
    } else {
        PathBuf::from(RESOURCES_FILE)
    };
    let res = gio::Resource::load(path).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = Application::default();
    app.run()
}
