// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

#[cfg(not(feature = "async"))]
pub mod blocking;
#[cfg(not(feature = "async"))]
pub use blocking::*;

#[cfg(feature = "async")]
pub mod asynchronous;
#[cfg(feature = "async")]
pub use asynchronous::*;

use snarkvm_console::program::Network;

use anyhow::{ensure, Result};
use std::marker::PhantomData;

/// Aleo API client for interacting with the Aleo Beacon API
#[derive(Clone, Debug)]
pub struct AleoAPIClient<N: Network> {
    #[cfg(feature = "async")]
    client: reqwest::Client,
    #[cfg(not(feature = "async"))]
    client: ureq::Agent,
    base_url: String,
    network_id: String,
    _network: PhantomData<N>,
}

impl<N: Network> AleoAPIClient<N> {
    pub fn new(base_url: &str, chain: &str) -> Result<Self> {
        #[cfg(feature = "async")]
        let client = reqwest::Client::new();
        #[cfg(not(feature = "async"))]
        let client = ureq::Agent::new();
        ensure!(
            base_url.starts_with("http://") || base_url.starts_with("https://"),
            "specified url {base_url} invalid, the base url must start with or https:// (or http:// if doing local development)"
        );
        Ok(AleoAPIClient {
            client,
            base_url: base_url.to_string(),
            network_id: chain.to_string(),
            _network: PhantomData,
        })
    }

    pub fn testnet3() -> Self {
        Self::new("https://vm.aleo.org/api", "testnet3").unwrap()
    }

    pub fn local_testnet3(port: &str) -> Self {
        Self::new(&format!("http://localhost:{}", port), "testnet3").unwrap()
    }

    /// Get base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get network ID being interacted with
    pub fn network_id(&self) -> &str {
        &self.network_id
    }
}
