//! The `device` module records data structures and methods related to wastewater devices in the
//! ESRI Utility Network.
use crate::import::utilities::entity::Entity;
use crate::import::utilities::wastewater::owner::Owner;
use crate::utils;
use geo::geometry;
use std::{fmt, path};

/// The `Device` struct represents a wastewater device from an ESRI Utility Network.
/// Domain field values in the UN are coded integers, so we will import the integers and recode
/// them with enums.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Device {
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

impl Device {
    /// Returns a reference to the value of the `asset_id` field, the unique identifier for the
    /// device.
    pub fn asset_id(&self) -> &String {
        &self.asset_id
    }

    /// Returns a reference to the value of the `historic_id` field, the historic identifier for the
    /// device.
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

    /// Creates a new `Device` struct from a shapefile geometry and record.
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
                let asset_type = AssetType::from_i16(asset_type as i16, &asset_group);
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
pub struct Devices(Vec<Device>);

impl Devices {
    /// The `from_shp` method converts from shapefiles of type [`shapefile::Polygon'].
    pub fn from_shp_z<P: AsRef<path::Path>>(path: P) -> aid::prelude::Clean<Self> {
        // the read_as method allows us to specify the spatial type, in this case PointZ
        // we also include the record field so we an read the field values.
        let shp = shapefile::read_as::<_, shapefile::PointZ, shapefile::dbase::Record>(path)?;
        // Iterate through the resulting vector, passing the shape to read_geo and then using
        // DeviceRaw::from_shp to read the associated record.
        let devices = shp
            .iter()
            .map(|(p, r)| {
                let geo = utils::read_geo_point(p);
                Device::from_shp(geo, r).unwrap()
            })
            .collect::<Vec<Device>>();
        Ok(Self(devices))
    }
}

/// The `AssetGroup` enum lists the valid asset groups for wastewater devices.
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
    /// Anode (not used by the city)
    Anode,
    /// Backflow Preventer
    BackflowPreventer,
    /// Cleanout
    Cleanout,
    /// Connection
    Connection,
    /// Controllable Valve
    ControllableValve,
    /// Flow valve
    FlowValve,
    /// Gate valve
    Gate,
    /// Grease Trap
    GreaseTrap,
    /// Groundbed
    Groundbed,
    /// Manhole Channel
    /// Used to represent manhole lid locations by the city.
    ManholeChannel,
    /// Meter
    Meter,
    /// Monitoring
    Monitoring,
    /// Outlet
    Outlet,
    /// Pump
    Pump,
    /// Rectifier
    Rectifier,
    /// Relief Valve
    ReliefValve,
    /// Service Connection
    ServiceConnection,
    /// Test Point
    TestPoint,
    /// Treatment
    Treatment,
    /// Weir
    Weir,
    /// Unknown
    #[default]
    Unknown,
}

impl fmt::Display for AssetGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Anode => write!(f, "Anode"),
            Self::BackflowPreventer => write!(f, "Backflow Preventer"),
            Self::Cleanout => write!(f, "Cleanout"),
            Self::Connection => write!(f, "Connection"),
            Self::ControllableValve => write!(f, "Controllable Valve"),
            Self::FlowValve => write!(f, "Flow Valve"),
            Self::Gate => write!(f, "Gate"),
            Self::GreaseTrap => write!(f, "Grease Trap"),
            Self::Groundbed => write!(f, "Groundbed"),
            Self::ManholeChannel => write!(f, "Manhole Channel"),
            Self::Meter => write!(f, "Meter"),
            Self::Monitoring => write!(f, "Monitoring"),
            Self::Outlet => write!(f, "Outlet"),
            Self::Pump => write!(f, "Pump"),
            Self::Rectifier => write!(f, "Rectifier"),
            Self::ReliefValve => write!(f, "Relief Valve"),
            Self::ServiceConnection => write!(f, "Service Connection"),
            Self::TestPoint => write!(f, "Test Point"),
            Self::Treatment => write!(f, "Treatment"),
            Self::Weir => write!(f, "Weir"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<i8> for AssetGroup {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::FlowValve,
            2 => Self::ControllableValve,
            3 => Self::ReliefValve,
            5 => Self::Pump,
            7 => Self::ServiceConnection,
            10 => Self::Meter,
            15 => Self::Treatment,
            21 => Self::Cleanout,
            22 => Self::Outlet,
            23 => Self::Connection,
            25 => Self::Monitoring,
            26 => Self::Weir,
            27 => Self::Gate,
            30 => Self::GreaseTrap,
            31 => Self::BackflowPreventer,
            32 => Self::ManholeChannel,
            50 => Self::Anode,
            51 => Self::Rectifier,
            52 => Self::TestPoint,
            53 => Self::Groundbed,
            _ => {
                tracing::warn!("Unrecognized asset group code: {}", value);
                Self::Unknown
            }
        }
    }
}

/// The `AssetGroup` enum lists the valid asset groups for wastewater utilities.
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
    /// Valve with the primary purpose of allowing large volumes of air to be exhausted from or
    /// admitted into a sewer pipe as it is filled or drained.
    /// A sub-type of [`AssetGroup::ReliefValve`].
    AirAndVacuum,
    /// Valve that relieves system of trapped air or vacuums.
    /// A sub-type of [`AssetGroup::FlowValve`].
    AirGap,
    /// Valve with a primary role of releasing air pockets.
    /// A sub-type of [`AssetGroup::ReliefValve`].
    AirRelease,
    /// Valve that controls flow into a tower when the level drops below a threshold.
    /// A sub-type of [`AssetGroup::FlowValve`].
    Altitude,
    /// A point representing a groundbed.
    /// A sub-type of [`AssetGroup::Groundbed`].
    AnodeBed,
    /// A point representing a backflow preventer.
    /// A sub-type of [`AssetGroup::BackflowPreventer`].
    BackflowPreventer,
    /// A weir that typically spans the width of a bioswale, or channel, and operates with higher
    /// levels of downstream wastewater.
    /// A sub-type of [`AssetGroup::Weir`].
    BroadCrested,
    /// Meter for measuring bulk waste.
    /// A sub-type of [`AssetGroup::Meter`].
    Bulk,
    /// Valve that closes to prevent backward wastewater flow.
    /// A sub-type of [`AssetGroup::FlowValve`].
    Check,
    /// A location to clean a clogged pipe.
    /// A sub-type of [`AssetGroup::Cleanout`].
    Cleanout,
    /// A weir designed for areas of varying flows of wastewater.
    /// A sub-type of [`AssetGroup::Weir`].
    Combination,
    /// Valve with combined features of an air/vacuum valve and air release valve.
    /// A sub-type of [`AssetGroup::ReliefValve`].
    CombinationAir,
    /// A service connection for a commercial service.
    /// A sub-type of [`AssetGroup::Connection`].
    Commercial,
    /// A point representing a directional manhole channel.
    /// A sub-type of [`AssetGroup::ManholeChannel`].
    DirectionalManholeChannel,
    /// The point treated wastewater enters a body of water.
    /// A sub-type of [`AssetGroup::Outlet`].
    EffluentDischarge,
    /// A point representing a galvanic anode.
    /// A sub-type of [`AssetGroup::Anode`].
    Galvanic,
    /// A gate that can be raised and lowered to control flow.
    /// A sub-type of [`AssetGroup::Gate`].
    Gate,
    /// A point representing a grease interceptor.
    /// A sub-type of [`AssetGroup::GreaseTrap`].
    GreaseInterceptor,
    /// A point representing a grease recovery.
    /// A sub-type of [`AssetGroup::GreaseTrap`].
    GreaseRecovery,
    /// A point representing a grease trap.
    /// A sub-type of [`AssetGroup::GreaseTrap`].
    GreaseTrap,
    /// A point representing an impressed current cathodic protection anode.
    /// A sub-type of [`AssetGroup::Anode`].
    Iccp,
    /// A service connection for an industrial service.
    /// A sub-type of [`AssetGroup::Connection`].
    Industrial,
    /// A weir designed to control flow by an increased length in respect to the bioswale or
    /// channel's width.
    /// A sub-type of [`AssetGroup::Weir`].
    Labyrinth,
    /// A small opening allowing a light to be lowered to inspect the pipe.
    /// A sub-type of [`AssetGroup::Cleanout`].
    LampHole,
    /// A point representing a manhole channel.
    /// A sub-type of [`AssetGroup::ManholeChannel`].
    ManholeChannel,
    /// A weir designed for areas of heavy/torrential storms to minimize flooding upstream.
    /// A sub-type of [`AssetGroup::Weir`].
    MinimumEnergyLoss,
    /// A point representing an overflow.
    /// A sub-type of [`AssetGroup::Outlet`].
    Overflow,
    /// The connection of multiple pipes.
    /// A sub-type of [`AssetGroup::Connection`].
    PipeConnection,
    /// A device to measure pressure.
    /// A sub-type of [`AssetGroup::Monitoring`].
    PressureSensor,
    /// Pump that moves sewage.
    /// A sub-type of [`AssetGroup::Pump`].
    Pump,
    /// Grinds solids of sewage and then the sewage is pumped.
    /// A sub-type of [`AssetGroup::Pump`].
    PumpWithGrinder,
    /// A point representing a rectifier.
    /// A sub-type of [`AssetGroup::Rectifier`].
    Rectifier,
    /// A service connection for a residential service.
    /// A sub-type of [`AssetGroup::Connection`].
    Residential,
    /// A weir designed to take accurate measurements of flow and discharge.
    /// A sub-type of [`AssetGroup::Weir`].
    SharpCrested,
    /// Meter for measuring wastewater through a pump station.
    /// A sub-type of [`AssetGroup::Meter`].
    Station,
    /// A wall that can be inserted and removed to control.
    /// A sub-type of [`AssetGroup::Gate`].
    StopLog,
    /// Valve with a primary purpose of isolation areas of the sewer network.
    /// A sub-type of [`AssetGroup::ControllableValve`].
    System,
    /// A device to measure temperature.
    /// A sub-type of [`AssetGroup::Monitoring`].
    TemperatureSensor,
    /// A cleanout provided at the upstream end of a sewer main when a manhole is not provided.
    /// A sub-type of [`AssetGroup::Cleanout`].
    Terminal,
    /// A point representing a test point for cathodic protection.
    /// A sub-type of [`AssetGroup::TestPoint`].
    TestPoint,
    /// Facility to treat wastewater.
    /// A sub-type of [`AssetGroup::Treatment`].
    TreatmentPlant,
    /// Valve to control wastewater flow in a vault.
    /// A sub-type of [`AssetGroup::ControllableValve`].
    Vault,
    /// A point representing a vertical pipe.
    /// A sub-type of [`AssetGroup::Weir`].
    VNotch,
    /// Unknown
    #[default]
    Unknown,
}

impl AssetType {
    /// The `from_i16` method converts integer coded asset types into an [`AssetType`] based upon
    /// the [`AssetGroup`].  Although most asset types have unique integer codes, there is overlap
    /// between the integer codes for [`AssetGroup::GreaseTrap`] and
    /// [`AssetGroup::ManholeChannel`].
    pub fn from_i16(value: i16, group: &AssetGroup) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Commercial,
            2 => Self::Industrial,
            3 => Self::Residential,
            41 => Self::AirGap,
            42 => Self::Altitude,
            43 => Self::Check,
            61 => Self::Pump,
            62 => Self::PumpWithGrinder,
            81 => Self::Cleanout,
            82 => Self::LampHole,
            83 => Self::Terminal,
            101 => Self::Gate,
            102 => Self::StopLog,
            121 => Self::AirAndVacuum,
            122 => Self::AirRelease,
            123 => Self::CombinationAir,
            141 => Self::System,
            142 => Self::Vault,
            161 => Self::Bulk,
            162 => Self::Station,
            181 => Self::PressureSensor,
            182 => Self::TemperatureSensor,
            201 => Self::TreatmentPlant,
            221 => Self::EffluentDischarge,
            222 => Self::Overflow,
            241 => Self::PipeConnection,
            281 => Self::BroadCrested,
            282 => Self::Combination,
            283 => Self::Labyrinth,
            284 => Self::MinimumEnergyLoss,
            285 => Self::SharpCrested,
            286 => Self::VNotch,
            301 => match *group {
                AssetGroup::GreaseTrap => Self::GreaseTrap,
                AssetGroup::ManholeChannel => Self::ManholeChannel,
                _ => {
                    tracing::warn!(
                        "Asset type {} does not match asset group {:?}",
                        value,
                        group
                    );
                    Self::Unknown
                }
            },
            302 => match *group {
                AssetGroup::GreaseTrap => Self::GreaseInterceptor,
                AssetGroup::ManholeChannel => Self::DirectionalManholeChannel,
                _ => {
                    tracing::warn!(
                        "Asset type {} does not match asset group {:?}",
                        value,
                        group
                    );
                    Self::Unknown
                }
            },
            303 => Self::GreaseRecovery,
            321 => Self::BackflowPreventer,
            901 => Self::Iccp,
            902 => Self::Galvanic,
            911 => Self::AnodeBed,
            941 => Self::Rectifier,
            981 => Self::TestPoint,
            _ => {
                tracing::warn!("Unrecognized asset type code: {}", value);
                Self::Unknown
            }
        }
    }
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AirAndVacuum => write!(f, "Air & Vacuum"),
            Self::AirGap => write!(f, "Air Gap"),
            Self::AirRelease => write!(f, "Air Release"),
            Self::Altitude => write!(f, "Altitude"),
            Self::AnodeBed => write!(f, "Anode Bed"),
            Self::BackflowPreventer => write!(f, "Backflow Preventer"),
            Self::BroadCrested => write!(f, "Broad-Crested"),
            Self::Bulk => write!(f, "Bulk"),
            Self::Check => write!(f, "Check"),
            Self::Cleanout => write!(f, "Cleanout"),
            Self::Combination => write!(f, "Combination"),
            Self::CombinationAir => write!(f, "Combination Air"),
            Self::Commercial => write!(f, "Commercial"),
            Self::DirectionalManholeChannel => write!(f, "Directional Manhole Channel"),
            Self::EffluentDischarge => write!(f, "Effluent Discharge"),
            Self::Galvanic => write!(f, "Galvanic"),
            Self::Gate => write!(f, "Gate"),
            Self::GreaseInterceptor => write!(f, "Grease Interceptor"),
            Self::GreaseRecovery => write!(f, "Grease Recovery"),
            Self::GreaseTrap => write!(f, "Grease Trap"),
            Self::Iccp => write!(f, "ICCP"),
            Self::Industrial => write!(f, "Industrial"),
            Self::Labyrinth => write!(f, "Labyrinth"),
            Self::LampHole => write!(f, "Lamp Hole"),
            Self::ManholeChannel => write!(f, "Manhole Channel"),
            Self::MinimumEnergyLoss => write!(f, "Minimum Energy Loss"),
            Self::Overflow => write!(f, "Overflow"),
            Self::PipeConnection => write!(f, "Pipe Connection"),
            Self::PressureSensor => write!(f, "Pressure Sensor"),
            Self::Pump => write!(f, "Pump"),
            Self::PumpWithGrinder => write!(f, "Pump with Grinder"),
            Self::Rectifier => write!(f, "Rectifier"),
            Self::Residential => write!(f, "Residential"),
            Self::SharpCrested => write!(f, "Sharp-Crested"),
            Self::Station => write!(f, "Station"),
            Self::StopLog => write!(f, "Stop Log"),
            Self::System => write!(f, "System"),
            Self::TemperatureSensor => write!(f, "Temperature Sensor"),
            Self::Terminal => write!(f, "Terminal"),
            Self::TestPoint => write!(f, "Test Point"),
            Self::TreatmentPlant => write!(f, "Treatment Plant"),
            Self::Vault => write!(f, "Vault"),
            Self::VNotch => write!(f, "V-Notch"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
