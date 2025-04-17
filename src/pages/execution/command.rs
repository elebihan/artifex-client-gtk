//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use chrono::Local;
use gtk::{glib, glib::Properties, prelude::*, subclass::prelude::*};
use std::cell::RefCell;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, glib::Enum, PartialEq, Default)]
#[enum_type(name = "CommandDirection")]
#[repr(u32)]
pub enum CommandDirection {
    #[default]
    Input,
    Output,
}

impl Display for CommandDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input => write!(f, "input"),
            Self::Output => write!(f, "output"),
        }
    }
}

#[derive(Clone, Copy, Debug, glib::Enum, PartialEq, Default)]
#[enum_type(name = "CommandStatus")]
#[repr(u32)]
pub enum CommandStatus {
    Ko,
    Ok,
    #[default]
    Running,
}

impl Display for CommandStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ko => write!(f, "ko"),
            Self::Ok => write!(f, "ok"),
            Self::Running => write!(f, "running"),
        }
    }
}

#[derive(Debug, Default)]
pub struct CommandData {
    pub direction: CommandDirection,
    pub timestamp: String,
    pub status: CommandStatus,
    pub text: String,
}

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::Command)]
    pub struct Command {
        #[property(name = "direction", get, set, type = CommandDirection, member = direction, builder(CommandDirection::Input))]
        #[property(name = "timestamp", get, set, type = String, member = timestamp)]
        #[property(name = "status", get, set, type = CommandStatus, member = status, builder(CommandStatus::Running))]
        #[property(name = "text", get, set, type = String, member = text)]
        pub(crate) data: RefCell<CommandData>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Command {
        const NAME: &'static str = "Command";
        type Type = super::Command;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Command {}
}

glib::wrapper! {
    pub struct Command(ObjectSubclass<imp::Command>);
}

impl Command {
    pub fn new(text: &str) -> Self {
        glib::Object::builder()
            .property("text", text)
            .property(
                "timestamp",
                &format!("{}", Local::now().format("%d/%m/%Y %H:%M:%S")),
            )
            .build()
    }
}

impl From<CommandData> for Command {
    fn from(value: CommandData) -> Self {
        glib::Object::builder()
            .property("direction", value.direction)
            .property("timestamp", &value.timestamp)
            .property("status", value.status)
            .property("text", &value.text)
            .build()
    }
}
