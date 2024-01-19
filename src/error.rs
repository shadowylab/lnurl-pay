// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use std::fmt;

#[derive(Debug)]
pub enum Error {
    Bech32(bech32::Error),
    InvalidLnUrl,
    InvalidLightningAddress,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bech32(e) => write!(f, "Bech32: {e}"),
            Self::InvalidLnUrl => write!(f, "Invalid LNURL"),
            Self::InvalidLightningAddress => write!(f, "Invalid Lightning Address"),
        }
    }
}

impl From<bech32::Error> for Error {
    fn from(e: bech32::Error) -> Self {
        Self::Bech32(e)
    }
}
