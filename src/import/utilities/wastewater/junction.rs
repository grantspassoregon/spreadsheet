//! The `junction` module records data structures and methods related to wastewater junctions in the
//! ESRI Utility Network.
use crate::import::utilities::entity::Entity;
use crate::import::utilities::wastewater::owner::Owner;
use crate::utils;
use geo::geometry;
use std::path;

/// The `Junction` struct represents a wastewater junction from an ESRI Utility Network.
/// Domain field values in the UN are coded integers, so we will import the integers and recode
/// them with enums.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Junction {
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
    /// The `geometry` field provides the `geo` representation of the polygon geometry.
    pub geometry: geometry::Geometry,
}

impl Junction {
    /// Returns a reference to the value of the `asset_id` field, the unique identifier for the
    /// junction.
    pub fn asset_id(&self) -> &String {
        &self.asset_id
    }

    /// Returns a reference to the value of the `historic_id` field, the historic identifier for the
    /// junction.
    pub fn historic_id(&self) -> &Option<String> {
        &self.historic_id
    }

    /// Returns a string representation of the `asset_group` field.
    pub fn asset_group(&self) -> String {
        self.asset_group.to_string()
    }

    /// Returns a string representation of the `asset_type` field.
    pub fn asset_type(&self) -> String {
        self.asset_type.to_string()
    }

    /// Returns a string representation of the `owner` field.
    pub fn owner(&self) -> String {
        self.owner.to_string()
    }

    /// Creates a new `Junction` struct from a shapefile geometry and record.
    pub fn from_shp(
        geometry: geo::geometry::Geometry,
        record: &shapefile::dbase::Record,
    ) -> aid::prelude::Clean<Self> {
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

/// The `Junctions` struct is a wrapper around a vector of type [`Junction`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Junctions(Vec<Junction>);

impl Junctions {
    /// The `from_shp_z` method converts from shapefiles of type [`shapefile::PointZ'].
    pub fn from_shp_z<P: AsRef<path::Path>>(path: P) -> aid::prelude::Clean<Self> {
        // the read_as method allows us to specify the spatial type, in this case PointZ
        // we also include the record field so we an read the field values.
        let shp = shapefile::read_as::<_, shapefile::PointZ, shapefile::dbase::Record>(path)?;
        // Iterate through the resulting vector, passing the shape to read_geo_point and then using
        // Junction::from_shp to read the associated record.
        let junctions = shp
            .iter()
            .map(|(p, r)| {
                let geo = utils::read_geo_point(p);
                Junction::from_shp(geo, r).unwrap()
            })
            .collect::<Vec<Junction>>();
        Ok(Self(junctions))
    }
}

/// The `AssetGroup` enum lists the valid asset groups for wastewater junctions.
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::EnumIter,
)]
pub enum AssetGroup {
    /// The Sewer Fitting asset group in the SewerJunction feature class represents the junctions
    /// where pipes connect to other pipes.
    Fitting,
    /// The Wire Junction asset group in the SewerJunction feature class represents cathodic
    /// protection junction features that connect to pipes.
    Wire,
    /// The Insulation Junction asset group in the SewerJunction feature class represents cathodic
    /// protection junction features where the electric current stops.
    Insulation,
    /// Unknown
    #[default]
    Unknown,
}

impl From<i8> for AssetGroup {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::Unknown,
            20 => Self::Fitting,
            50 => Self::Wire,
            51 => Self::Insulation,
            _ => {
                tracing::warn!("Unrecognized asset group code: {}", value);
                Self::Unknown
            }
        }
    }
}

/// The `AssetGroup` enum lists the valid asset groups for wastewater junctions.
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::EnumIter,
)]
pub enum AssetType {
    /// Fitting with a socket at both ends connecting 2 pipes of the same size that is joined,
    /// welded, brazed or soldered.
    /// A sub-type of [`AssetGroup::Fitting`].
    Coupling,
    /// Fitting connecting 4 pipes of the same size.
    /// A sub-type of [`AssetGroup::Fitting`].
    Cross,
    /// Fitting connecting 2 pipes to allow a change in direction.
    /// A sub-type of [`AssetGroup::Fitting`].
    Elbow,
    /// Fitting to cover an open end of a pipe.
    /// A sub-type of [`AssetGroup::Fitting`].
    EndCap,
    /// Fitting connecting 2 pipes designed to absorb thermal expansion or movement.
    /// A sub-type of [`AssetGroup::Fitting`].
    ExpansionJoint,
    /// Fitting connecting pipes or valves with threaded bolts, wedges, clamps or other means of
    /// high compressive force.
    /// A sub-type of [`AssetGroup::Fitting`].
    Flange,
    /// Fitting inserted inside a pipe segment to stop flow.
    /// A sub-type of [`AssetGroup::Fitting`].
    Plug,
    /// Fitting connecting 2 pipes of different sizes.
    /// A sub-type of [`AssetGroup::Fitting`].
    Reducer,
    /// Fitting connecting 4 pipes of different sizes.
    /// A sub-type of [`AssetGroup::Fitting`].
    ReducingCross,
    /// Fitting connecting 3 pipes of different sizes.
    /// A sub-type of [`AssetGroup::Fitting`].
    ReducingTee,
    /// Fitting connecting 2 pipes by placing 2 different sides of the fitting around the pipes and
    /// securing in place with bolts.
    /// A sub-type of [`AssetGroup::Fitting`].
    Saddle,
    /// Fitting connection 2 pipes by screwing the pipes together.
    /// A sub-type of [`AssetGroup::Fitting`].
    Screw,
    /// Fitting connecting 2 pipes that by placing a sleeve over the pipe ends secured with clamps.
    /// A sub-type of [`AssetGroup::Fitting`].
    Sleeve,
    /// Fitting placed around a pipe and secured in place with bolts to enable a tap to be inserted in the pipe.
    /// A sub-type of [`AssetGroup::Fitting`].
    Tap,
    /// Fitting connection 2 different size pipes that is mounted around both sides of the larger
    /// pipe.
    /// A sub-type of [`AssetGroup::Fitting`].
    TappingSaddle,
    /// Fitting connection 3 pipes of the same size.
    /// A sub-type of [`AssetGroup::Fitting`].
    Tee,
    /// Fitting that transitions a pipe to another pipe of a different material.
    /// A sub-type of [`AssetGroup::Fitting`].
    Transition,
    /// Fitting that welds 2 pipes together.
    /// A sub-type of [`AssetGroup::Fitting`].
    Weld,
    /// Fitting connecting 3 pipes where a side inlet pipe joins at less than 90 degrees.
    /// A sub-type of [`AssetGroup::Fitting`].
    Wye,
    /// Unknown
    #[default]
    Unknown,
}

impl From<i16> for AssetType {
    fn from(value: i16) -> Self {
        match value {
            0 => Self::Unknown,
            41 => Self::Coupling,
            42 => Self::Cross,
            43 => Self::Elbow,
            44 => Self::EndCap,
            45 => Self::ExpansionJoint,
            46 => Self::Flange,
            47 => Self::Plug,
            48 => Self::Reducer,
            49 => Self::ReducingCross,
            50 => Self::ReducingTee,
            51 => Self::Saddle,
            52 => Self::Screw,
            53 => Self::Sleeve,
            54 => Self::Tap,
            55 => Self::TappingSaddle,
            56 => Self::Tee,
            57 => Self::Transition,
            58 => Self::Weld,
            59 => Self::Wye,
            _ => {
                tracing::warn!("Unrecognized asset type code: {}", value);
                Self::Unknown
            }
        }
    }
}
