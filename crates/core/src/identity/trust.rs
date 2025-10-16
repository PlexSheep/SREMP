use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Trust {
    Unknown,
    Trusted,
    Rejected,
}

impl Display for Trust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unknown => "Unknown",
                Self::Trusted => "Trusted",
                Self::Rejected => "Rejected",
            }
        )
    }
}
