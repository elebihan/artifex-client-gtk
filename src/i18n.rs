//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use gettextrs::gettext;

// Taken from https://gitlab.com/news-flash/news_flash_gtk/-/blob/master/src/i18n.rs
pub fn i18n(format: &str, args: &[&str]) -> String {
    let s = gettext(format);
    let mut parts = s.split("{}");
    let mut output = parts.next().unwrap_or_default().to_string();
    for (p, a) in parts.zip(args.iter()) {
        output += &(a.to_string() + p);
    }
    output
}
