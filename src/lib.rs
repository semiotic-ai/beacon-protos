// Copyright 2024-, Semiotic AI, Inc.
// SPDX-License-Identifier: Apache-2.0

mod beacon_v1;
mod error;

pub use beacon_v1::{block::Body, Block, BlockRoot};
pub use error::BeaconProtosError;
