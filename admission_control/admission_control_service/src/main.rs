// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use solana_libra_admission_control_service as admission_control_service;

use admission_control_service::admission_control_node;
use executable_helpers::helpers::{
    setup_executable, ARG_CONFIG_PATH, ARG_DISABLE_LOGGING, ARG_PEER_ID,
};

/// Run a Admission Control service in its own process.
/// It will also setup global logger and initialize config.
fn main() {
    let (config, _logger, _args) = setup_executable(
        "Libra AdmissionControl node".to_string(),
        vec![ARG_PEER_ID, ARG_CONFIG_PATH, ARG_DISABLE_LOGGING],
    );

    let admission_control_node = admission_control_node::AdmissionControlNode::new(config);

    admission_control_node
        .run()
        .expect("Unable to run AdmissionControl node");
}
