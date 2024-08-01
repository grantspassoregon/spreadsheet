//! The `line` module records data structures and methods related to wastewater lines in the
//! ESRI Utility Network.
use crate::import::utilities::entity::Entity;
use crate::import::utilities::wastewater::owner::Owner;
use crate::utils;
use geo::geometry;
use std::path;

/// The `Line` struct represents a wastewater line from an ESRI Utility Network..
/// Domain field values in the UN are coded integers, so we will import the integers and recode
/// them with enums.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Line {
    /// The asset group from the ESRI Utility Network.
    pub asset_group: AssetGroup,
    /// The asset type from the ESRI Utility Network.
    pub asset_type: AssetType,
    /// The asset id from the ESRI Utility Network.
    pub asset_id: String,
    /// The historic id used by the City for tracking the asset.
    pub historic_id: Option<String>,
    /// The asset owner.
    pub owner: Owner,
    /// The `geometry` field provides the `geo` representation of the line geometry.
    pub geometry: geometry::Geometry,
}

impl Line {
    /// Creates a new `Line` struct from a shapefile geometry and record.
    pub fn from_shp(
        geometry: geo::geometry::Geometry,
        record: &shapefile::dbase::Record,
    ) -> aid::prelude::Clean<Self> {
        // tracing::info!("{:#?}", record);
        let group = utils::read_num(record, "ASSETGROUP");
        let kind = utils::read_num(record, "ASSETTYPE");
        let id = utils::read_char(record, "assetid");
        let historic_id = utils::read_char(record, "historicid");
        let owner = utils::read_num(record, "ownedby");

        if let Some(asset_group) = group {
            let asset_group = AssetGroup::from(asset_group as i8);
            if let Some(asset_type) = kind {
                let asset_type = AssetType::from(asset_type as i16);
                if let Some(asset_id) = id {
                    if let Some(owner) = owner {
                        Ok(Self {
                            asset_group,
                            asset_type,
                            asset_id,
                            historic_id,
                            owner: Owner::from(&Entity::from(owner as i8)),
                            geometry,
                        })
                    } else {
                        tracing::warn!("Failed to read owner field.");
                        Err(aid::prelude::Bandage::Hint(
                            "Failed to read owner field.".to_string(),
                        ))
                    }
                } else {
                    tracing::warn!("Failed to read an asset id.");
                    Err(aid::prelude::Bandage::Hint(
                        "Failed to read an asset id.".to_string(),
                    ))
                }
            } else {
                tracing::warn!("Failed to read an asset type.");
                Err(aid::prelude::Bandage::Hint(
                    "Failed to read an asset type.".to_string(),
                ))
            }
        } else {
            tracing::warn!("Failed to read an asset group.");
            Err(aid::prelude::Bandage::Hint(
                "Failed to read an asset group.".to_string(),
            ))
        }
    }
}

/// The `Devices` struct is a wrapper around a vector of type [`Device`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Lines(Vec<Line>);

impl Lines {
    /// The `from_shp` method converts from shapefiles of type [`shapefile::Polygon'].
    pub fn from_shp_z<P: AsRef<path::Path>>(path: P) -> aid::prelude::Clean<Self> {
        // the read_as method allows us to specify the spatial type, in this case PointZ
        // we also include the record field so we an read the field values.
        let shp = shapefile::read_as::<_, shapefile::PolylineZ, shapefile::dbase::Record>(path)?;
        // Iterate through the resulting vector, passing the shape to read_geo and then using
        // DeviceRaw::from_shp to read the associated record.
        let lines = shp
            .iter()
            .map(|(p, r)| {
                let geo = utils::read_geo_line(p);
                Line::from_shp(geo, r).unwrap()
            })
            .collect::<Vec<Line>>();
        Ok(Self(lines))
    }
}

/// The `AssetGroup` enum lists the valid asset groups for wastewater lines.
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
pub enum AssetGroup {
    /// The Bonding Line asset group in the SewerLine feature class represents cathodic protection
    /// with a line feature connecting protected and non-protected features.
    BondingLine,
    /// The Sewer Force Main asset group in the SewerLine feature class represents pipes with a
    /// primary role lifting wastewater to a higher elevation.
    ForceMain,
    /// The Sewer Gravity Main asset group in the SewerLine feature class represents pipes using
    /// gravity to collect and transport wastewater.
    GravityMain,
    /// The Lateral asset group in the SewerLine feature class represents service lines with a
    /// primary role of transporting wastewater from the customer to the main.
    Lateral,
    /// The Rectifier Cable asset group in the SewerLine Feature class represents cathodic
    /// protection wire that connects the rectifier to the pipe.
    RectifierCable,
    /// The Test Lead Wire asset group in the SewerLine Feature class represents cathodic
    /// protection connecting from the test point to protected pipes.
    TestLeadWire,
    /// Unknown
    #[default]
    Unknown,
}

impl From<i8> for AssetGroup {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::GravityMain,
            2 => Self::ForceMain,
            3 => Self::Lateral,
            50 => Self::BondingLine,
            51 => Self::TestLeadWire,
            52 => Self::RectifierCable,
            _ => {
                tracing::warn!("Unrecognized owner code: {}", value);
                Self::Unknown
            }
        }
    }
}

/// The `AssetGroup` enum lists the valid asset groups for wastewater lines.
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
pub enum AssetType {
    /// Aqua test lead wire.
    /// A sub-type of [`AssetGroup::TestLeadWire`].
    Aqua,
    /// Blue test lead wire.
    /// A sub-type of [`AssetGroup::TestLeadWire`].
    Blue,
    /// Sewer Gravity Main to collect wastewater.
    /// A sub-type of [`AssetGroup::GravityMain`].
    Collector,
    /// A Commercial lateral service.
    /// A sub-type of [`AssetGroup::Lateral`].
    Commercial,
    /// Sewer Force main to collect wastewater.
    /// A sub-type of [`AssetGroup::ForceMain`].
    ForceMain,
    /// Forest test lead wire.
    /// A sub-type of [`AssetGroup::TestLeadWire`].
    Forest,
    /// Green test lead wire.
    /// A sub-type of [`AssetGroup::TestLeadWire`].
    Green,
    /// An Industrial lateral service.
    Industrial,
    /// Sewer Gravity Main to transport wastewater to the treatment plant.
    /// A sub-type of [`AssetGroup::GravityMain`].
    Interceptor,
    /// Lavender test lead wire.
    /// A sub-type of [`AssetGroup::TestLeadWire`].
    Lavender,
    /// Orange test lead wire.
    /// A sub-type of [`AssetGroup::TestLeadWire`].
    Orange,
    /// Pink test lead wire.
    /// A sub-type of [`AssetGroup::TestLeadWire`].
    Pink,
    /// Rectifier cable used for connecting rectifier to pipe for cathodic protection.
    /// A sub-type of [`AssetGroup::RectifierCable`].
    RectifierCable,
    /// Red test lead wire.
    /// A sub-type of [`AssetGroup::TestLeadWire`].
    Red,
    /// A residential lateral service.
    /// A sub-type of [`AssetGroup::Lateral`].
    Residential,
    /// Strap bonding line.
    /// A sub-type of [`AssetGroup::BondingLine`].
    Strap,
    /// Wire bonding line.
    /// A sub-type of [`AssetGroup::BondingLine`].
    Wire,
    /// Yellow test lead wire.
    /// A sub-type of [`AssetGroup::TestLeadWire`].
    Yellow,
    /// Unknown
    #[default]
    Unknown,
}

impl From<i16> for AssetType {
    fn from(value: i16) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Collector,
            2 => Self::Interceptor,
            41 => Self::ForceMain,
            121 => Self::Commercial,
            122 => Self::Industrial,
            123 => Self::Residential,
            901 => Self::Aqua,
            902 => Self::Blue,
            903 => Self::Forest,
            904 => Self::Green,
            905 => Self::Lavender,
            906 => Self::Orange,
            907 => Self::Pink,
            908 => Self::Red,
            909 => Self::Yellow,
            941 => Self::Wire,
            942 => Self::Strap,
            961 => Self::RectifierCable,
            _ => {
                tracing::warn!("Unrecognized owner code: {}", value);
                Self::Unknown
            }
        }
    }
}
