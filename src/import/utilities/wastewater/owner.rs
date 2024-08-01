//! The `owner` module defines the `Owner` enum that lists the valid owners for wastewater assets.
use crate::import::utilities::entity::Entity;

/// The `Owner` enum lists the valid owners for wastewater assets.
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::EnumIter,
)]
pub enum Owner {
    /// City of Grants Pass.
    #[default]
    City,
    /// Josephine County.
    County,
    /// State of Oregon.
    State,
    /// United States of America.
    Federal,
    /// Privately owned and maintained.
    Private,
    /// Unknown
    Unknown,
}

impl From<i8> for Owner {
    fn from(value: i8) -> Self {
        match value {
            1 => Self::City,
            // 2 => Self::Gpid,
            3 => Self::State,
            4 => Self::County,
            5 => Self::Federal,
            6 => Self::Private,
            7 => Self::Unknown,
            // 8 => Self::Parks,
            100 => Self::Unknown,
            _ => {
                tracing::warn!("Unrecognized owner code: {}", value);
                Self::Unknown
            }
        }
    }
}

impl From<&Entity> for Owner {
    fn from(value: &Entity) -> Self {
        match *value {
            Entity::City => Self::City,
            Entity::State => Self::State,
            Entity::County => Self::County,
            Entity::Federal => Self::Federal,
            Entity::Private => Self::Private,
            Entity::Unknown => Self::Unknown,
            Entity::Gpid => {
                tracing::warn!("GPID not a valid owner for wastewater assets.");
                Self::Unknown
            }
            Entity::Parks => {
                tracing::warn!("Parks Dept not a valid owner for wastewater assets.");
                Self::Unknown
            }
        }
    }
}
