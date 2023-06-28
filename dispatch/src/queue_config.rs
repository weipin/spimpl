// Copyright 2023 Developers of the Spimpl project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Qos, Queue, QueueAttributes};

#[derive(Default)]
pub struct QueueConfig<'a> {
    pub qos: Qos,
    pub attributes: QueueAttributes,
    pub target: Option<&'a Queue>,
}
