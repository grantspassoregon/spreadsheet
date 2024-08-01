//! The `entity` module contains the `Entity` enum that lists the valid entities that can own and
//! maintain city utility assets.

/// The `Entity` enumerates the types of entity that can take ownernship or maintenance
/// responsibility for a utility asset.  The types of entities that can control an asset varies by
/// asset.  For instance, GPID may own stormwater assets, but not wastewater assets.  However,
/// stormwater assets and wastewater assets may both be owned by a common entity, City of Grants
/// Pass.  In this way, we can have multiple asset owner types that refer to a single common
/// entity, making invalid states of ownership expressible in the type system.
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
pub enum Entity {
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
    /// Grants Pass Irrigation District
    Gpid,
    /// COGP Parks Dept.
    Parks,
    /// Unknown
    Unknown,
}

impl From<i8> for Entity {
    fn from(value: i8) -> Self {
        match value {
            1 => Self::City,
            2 => Self::Gpid,
            3 => Self::State,
            4 => Self::County,
            5 => Self::Federal,
            6 => Self::Private,
            7 => Self::Unknown,
            8 => Self::Parks,
            100 => Self::Unknown,
            _ => {
                tracing::warn!("Unrecognized owner code: {}", value);
                Self::Unknown
            }
        }
    }
}
