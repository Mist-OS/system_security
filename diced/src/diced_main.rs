// Copyright 2021, The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Main entry point for diced, the friendly neighborhood DICE service.

use diced::dice::CDI_SIZE;
use diced::DiceNode;
use diced::ResidentNode;
use std::panic;
use std::sync::Arc;

static DICE_SERVICE_NAME: &str = "android.security.dice";

fn main() {
    android_logger::init_once(
        android_logger::Config::default().with_tag("diced").with_min_level(log::Level::Debug),
    );
    // Redirect panic messages to logcat.
    panic::set_hook(Box::new(|panic_info| {
        log::error!("{}", panic_info);
    }));

    // Saying hi.
    log::info!("Diced, your friendly neighborhood DICE service, is starting.");

    let node_impl = Arc::new(
        ResidentNode::new(&[0u8; CDI_SIZE], &[1u8; CDI_SIZE], vec![])
            .expect("Failed to construct a resident node."),
    );

    let node =
        DiceNode::new_as_binder(node_impl).expect("Failed to create IDiceNode service instance.");

    binder::add_service(DICE_SERVICE_NAME, node.as_binder())
        .expect("Failed to register IDiceNode Service");

    log::info!("Joining thread pool now.");
    binder::ProcessState::join_thread_pool();
}
