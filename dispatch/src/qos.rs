// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use dispatch_sys::{
    dispatch_qos_class_t, QOS_CLASS_BACKGROUND, QOS_CLASS_DEFAULT, QOS_CLASS_UNSPECIFIED,
    QOS_CLASS_USER_INITIATED, QOS_CLASS_USER_INTERACTIVE, QOS_CLASS_UTILITY,
};

#[derive(Debug, Default, PartialEq)]
#[repr(u8)]
pub enum Qos {
    UserInteractive,
    UserInitiated,
    Default,
    Utility,
    Background,
    #[default]
    Unspecified,
}

impl Qos {
    pub(crate) fn new(qos_class: dispatch_qos_class_t) -> Self {
        match qos_class {
            QOS_CLASS_USER_INTERACTIVE => Self::UserInteractive,
            QOS_CLASS_USER_INITIATED => Self::UserInitiated,
            QOS_CLASS_DEFAULT => Self::Default,
            QOS_CLASS_UTILITY => Self::Utility,
            QOS_CLASS_BACKGROUND => Self::Background,
            QOS_CLASS_UNSPECIFIED => Self::Unspecified,
            _ => panic!("unexpected qos class"),
        }
    }

    pub(crate) fn to_sys(&self) -> dispatch_qos_class_t {
        match self {
            Self::UserInteractive => QOS_CLASS_USER_INTERACTIVE,
            Self::UserInitiated => QOS_CLASS_USER_INITIATED,
            Self::Default => QOS_CLASS_DEFAULT,
            Self::Utility => QOS_CLASS_UTILITY,
            Self::Background => QOS_CLASS_BACKGROUND,
            Self::Unspecified => QOS_CLASS_UNSPECIFIED,
        }
    }
}
