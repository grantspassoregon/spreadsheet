//! The `beehive` module contains data structures and methods related to the Beehive asset
//! management system.
//! The purpose of this module is to import the record of Beehive Events into a spatial layer for
//! querying and analysis.
use crate::utils;
use jiff::civil;
use std::str::FromStr;

/// The `EventRaw` struct contains fields for a Beehive event.  Although we could serialize
/// straight into the desired types, this is my first time working with `jiff` for datetime, so I
/// serialize all the data to Strings as an intermediary step and then convert to a strongly typed
/// `Event`.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct EventRaw {
    #[serde(rename = "Feature Name")]
    asset_id: String,
    #[serde(rename = "Feature Type")]
    asset_kind: String,
    #[serde(rename = "Assigned To")]
    assigned_to: String,
    #[serde(rename = "Created By")]
    created_by: String,
    #[serde(rename = "Create Date")]
    create_date: String,
    #[serde(rename = "Maintenance Type")]
    maintenance: Option<String>,
    #[serde(rename = "Modified Date")]
    modify_date: String,
    #[serde(rename = "Modified By")]
    modified_by: String,
    #[serde(rename = "Event Name")]
    name: String,
    #[serde(rename = "Note")]
    notes: Option<String>,
    #[serde(rename = "Event Type")]
    kind: String,
    #[serde(rename = "Planned Date")]
    plan_date: String,
    #[serde(rename = "Priority")]
    priority: String,
    #[serde(rename = "Schedule Time")]
    schedule_time: String,
    // "Complete" = true; null = false;
    #[serde(rename = "Event Status")]
    status: Option<String>,
}

/// The `EventsRaw` struct is a wrapper around a vector of type [`EventRaw`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct EventsRaw(Vec<EventRaw>);

impl EventsRaw {
    /// Reads the contents of the CSV file at the location specified in `path` into a `EventRaw`
    /// struct.
    /// Each element in the vector of type [`EventRaw`] maps to a row on the spreadsheet.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = utils::from_csv(path)?;
        Ok(EventsRaw(records))
    }
}

/// The `Event` struct is a Beehive Event that has been converted into domain-specific data structures.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Event {
    // Upgrade to strongly typed ID
    asset_id: String,
    asset_kind: AssetKind,
    assigned_to: String,
    created_by: String,
    create_date: civil::DateTime,
    maintenance: Option<Maintenance>,
    modify_date: civil::DateTime,
    modified_by: String,
    name: String,
    notes: Option<String>,
    kind: EventKind,
    plan_date: civil::Date,
    priority: Priority,
    schedule_time: civil::Time,
    status: Option<Status>,
}

impl TryFrom<EventRaw> for Event {
    type Error = aid::prelude::Bandage;

    /// Functions as a `new` constructor from a raw event.
    fn try_from(value: EventRaw) -> Result<Self, Self::Error> {
        // Convert fields to strongly-typed parameters.
        let (_, create_date) = utils::datetime(&value.create_date)?;
        let (_, modify_date) = utils::datetime(&value.modify_date)?;
        let asset_kind = AssetKind::from_str(&value.asset_kind)?;
        let kind = EventKind::from_str(&value.kind)?;
        let maintenance = Maintenance::from_raw(&value.maintenance);
        let (_, plan_date) = utils::mdy(&value.plan_date)?;
        let priority = Priority::from_str(&value.priority)?;
        // Schedule time may be blank, but we still want to load the event with a default.
        // Default value is midnight (0, 0, 0, 0).
        let mut schedule_time = civil::Time::default();
        if let Ok((_, result)) = utils::hm12(&value.schedule_time) {
            // Schedule time found, update value.
            schedule_time = result;
        }
        let status = Status::from_raw(&value.status);

        Ok(Self {
            asset_id: value.asset_id,
            asset_kind,
            assigned_to: value.assigned_to,
            create_date,
            created_by: value.created_by,
            maintenance,
            modified_by: value.modified_by,
            modify_date,
            name: value.name,
            notes: value.notes,
            kind,
            plan_date,
            priority,
            schedule_time,
            status,
        })
    }
}

impl TryFrom<&EventRaw> for Event {
    type Error = aid::prelude::Bandage;

    fn try_from(value: &EventRaw) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}

/// The `Events` struct is a wrapper around a vector of type [`Event`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Events(Vec<Event>);

impl From<EventsRaw> for Events {
    fn from(value: EventsRaw) -> Self {
        let mut events = Vec::new();
        let mut dropped = 0;
        value
            .iter()
            .enumerate()
            .map(|(i, v)| match Event::try_from(v) {
                Ok(event) => events.push(event),
                Err(e) => {
                    dropped += 1;
                    tracing::warn!("Dropping row {}, {}", i, e.to_string());
                }
            })
            .for_each(drop);
        if dropped > 0 {
            tracing::warn!("Dropped {dropped} events.");
        } else {
            tracing::trace!("Events read: {}", events.len());
        }
        Self(events)
    }
}

impl From<&EventsRaw> for Events {
    fn from(value: &EventsRaw) -> Self {
        Self::from(value.clone())
    }
}

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
/// The `EventKind` enumerates the types of Beehive events that can be generated by the user.
pub enum EventKind {
    /// Variant for Beehive Event "_Adjustment"
    Adjustment,
    /// Variant for Beehive Event "_Bypass Pumping"
    Bypass,
    /// Variant for Beehive Event "Wastewater Call-in"
    CallIn,
    /// A CCTV inspection from the Granite van.
    Cctv,
    /// A cleaning of a wastewater pipe segment using the vaccuum truck.
    /// Variant for Beehive Events "Cleaning", "Clean" and "_Cleaning - Emergency/Blockages"
    Cleaning,
    /// Variant for Beehive Event "Construction"
    Construction,
    /// Variant for Beehive Event "Contractor"
    Contractor,
    /// Variant for Beehive Event "_Inspection - Contractor"
    ContractorInspection,
    /// Variant for Beehive Events "Data Collection" and "_Data Collection"
    DataCollection,
    /// Variant for Beehive Event "_Debris/Grit Removal"
    DebrisRemoval,
    /// Variant for Beehive Events "Emergency Response/Callout" and "_Emergency Response/Callout"
    Emergency,
    /// Variant for Beehive Events "_Excavation"
    Excavation,
    /// Variant for Beehive Event "_TV Inspection - Final"
    FinalInspection,
    /// Variant for Beehive Event "Grease"
    Grease,
    /// Variant for Beehive Event "_Sealing/Grouting"
    Grouting,
    /// Variant for Beehive Event "Housekeeping"
    Housekeeping,
    /// Variant for Beehive Events "Inspection" and "_Inspection"
    Inspection,
    /// Variant for Beehive Events "Installation" and "_Installation"
    Installation,
    /// Variant for Beehive Events "Investigation" and "_Investigation"
    Investigation,
    /// Variant for Beehive Event "_Jetting"
    Jetting,
    /// Variant for Beehive Event "Line Parking (Force Main)"
    LineParking,
    /// Variant for Beehive Events "_Locates (Visual)" and "_Locates (GPS)"
    Locates,
    /// Variant for Beehive Events "Wastewater Maintenance", "Maintenance" and "_Maintenance"
    // Given an arbitrary Default value to enable default initialization of any parent struct.
    #[default]
    Maintenance,
    /// Variant for Beehive Event "Mapping"
    Mapping,
    /// Variant for Beehive Event "_Miscellaneous Tasks"
    Misc,
    /// Variant for Beehive Event "Notification"
    Notification,
    /// Variant for Beehive Events "Other" and "_Other"
    Other,
    /// Variant for Beehive Event "Owner Contacted"
    Owner,
    /// Variant for Beehive Event "_Cleaning - Planned"
    PlannedCleaning,
    /// Variant for Beehive Event "_TV Inspection - Planned"
    PlannedInspection,
    /// Variant for Beehive Event "_Record Keeping/Updating"
    Records,
    /// Variant for Beehive Events "_I&I Reduction" and "I&I Reduction"
    Reduction,
    /// Variant for Beehive Events "Replaced"
    Replaced,
    /// Variant for Beehive Events "Restoration" and "_Restoration"
    Restoration,
    /// Variant for Beehive Event "Root Control" and "_Root Control Mechanical"
    RootControl,
    /// Variant for Beehive Events "Testing" and "_Testing (dye)"
    Testing,
    /// Variant for Beehive Event "_TV Inspection - Unplanned"
    UnplannedInspection,
    /// Variant for Beehive Event "_Vacuum"
    Vacuum,
    /// Variant for Beehive Events "_Inspection - Visual" and "_Inspection - Unplanned Visual"
    VisualInspection,
    /// Variant for Beehive Events "_1Yr Warranty Clean" and "_1Yr Warranty TV Inspection"
    Warranty,
}

impl std::str::FromStr for EventKind {
    type Err = aid::prelude::Bandage;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "_Adjustment" => Ok(Self::Adjustment),
            "_Bypass Pumping" => Ok(Self::Bypass),
            "Wastewater Call-in" => Ok(Self::CallIn),
            "CCTV" => Ok(Self::Cctv),
            "Cleaning" => Ok(Self::Cleaning),
            "Clean" => Ok(Self::Cleaning),
            "_Cleaning - Emergency/Blockages" => Ok(Self::Cleaning),
            "Construction" => Ok(Self::Construction),
            "Contractor" => Ok(Self::Contractor),
            "_Inspection - Contractor" => Ok(Self::ContractorInspection),
            "Data Collection" => Ok(Self::DataCollection),
            "_Data Collection" => Ok(Self::DataCollection),
            "_Debris/Grit Removal" => Ok(Self::DebrisRemoval),
            "Emergency Response/Callout" => Ok(Self::Emergency),
            "_Emergency Response/Callout" => Ok(Self::Emergency),
            "_Excavation" => Ok(Self::Excavation),
            "_TV Inspection - Final" => Ok(Self::FinalInspection),
            "Grease" => Ok(Self::Grease),
            "_Sealing/Grouting" => Ok(Self::Grouting),
            "Housekeeping" => Ok(Self::Housekeeping),
            "Inspection" => Ok(Self::Inspection),
            "_Inspection" => Ok(Self::Inspection),
            "Installation" => Ok(Self::Installation),
            "_Installation" => Ok(Self::Installation),
            "Investigation" => Ok(Self::Investigation),
            "_Investigation" => Ok(Self::Investigation),
            "_Jetting" => Ok(Self::Jetting),
            "Line Parking (Force Main)" => Ok(Self::LineParking),
            "_Locates (Visual)" => Ok(Self::Locates),
            "_Locates (GPS)" => Ok(Self::Locates),
            "Wastewater Maintenance" => Ok(Self::Maintenance),
            "Maintenance" => Ok(Self::Maintenance),
            "_Maintenance" => Ok(Self::Maintenance),
            "Mapping" => Ok(Self::Mapping),
            "_Miscellaneous Tasks" => Ok(Self::Misc),
            "Notification" => Ok(Self::Notification),
            "Other" => Ok(Self::Other),
            "_Other" => Ok(Self::Other),
            "Owner Contacted" => Ok(Self::Owner),
            "_Cleaning - Planned" => Ok(Self::PlannedCleaning),
            "_TV Inspection - Planned" => Ok(Self::PlannedInspection),
            "_Record Keeping/Updating" => Ok(Self::Records),
            "I&I Reduction" => Ok(Self::Reduction),
            "_I&I Reduction" => Ok(Self::Reduction),
            "Replaced" => Ok(Self::Replaced),
            "Restoration" => Ok(Self::Restoration),
            "_Restoration" => Ok(Self::Restoration),
            "Root Control" => Ok(Self::RootControl),
            "_Root Control Mechanical" => Ok(Self::RootControl),
            "Testing" => Ok(Self::Testing),
            "_Testing (dye)" => Ok(Self::Testing),
            "_TV Inspection - Unplanned" => Ok(Self::UnplannedInspection),
            "_Vacuum" => Ok(Self::Vacuum),
            "_Inspection - Visual" => Ok(Self::VisualInspection),
            "_Inspection - Unplanned Visual" => Ok(Self::VisualInspection),
            "_1Yr Warranty Clean" => Ok(Self::Warranty),
            "_1yr Warranty TV Inspection" => Ok(Self::Warranty),
            _ => Err(aid::prelude::Bandage::Hint(format!(
                "Unrecognized EventKind: {s}"
            ))),
        }
    }
}

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
/// Types of wastewater assets used to classify Beehive events.
pub enum AssetKind {
    /// Wastewater basin
    Basin,
    /// Beehive feature name "Wastewater Call-In"
    CallIn,
    /// Cleanout
    Cleanout,
    /// Fitting
    Fitting,
    /// Force main
    Force,
    /// Lateral pipe
    Lateral,
    /// Beehive feature name "Wastewater Maintenance"
    // Given an arbitrary Default value to enable default initialization of any parent struct.
    #[default]
    Maintenance,
    /// Manhole
    Manhole,
    /// Gravity Main
    Pipe,
    /// Beehive feature name "Wastewater Pump"
    Pump,
    /// Beehive feature name "Wastewater Network Structure"
    Structure,
    /// Valve
    Valve,
}

impl std::str::FromStr for AssetKind {
    type Err = aid::prelude::Bandage;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Wastewater Sewer Basin" => Ok(Self::Basin),
            "Wastewater Cleanouts" => Ok(Self::Cleanout),
            "Wastewater Call-In" => Ok(Self::CallIn),
            "Wastewater Fitting" => Ok(Self::Fitting),
            "Wastewater Force Main" => Ok(Self::Force),
            "Wastewater Lateral" => Ok(Self::Lateral),
            "Wastewater Maintenance" => Ok(Self::Maintenance),
            "Wastewater Manhole" => Ok(Self::Manhole),
            "Wastewater Pipe" => Ok(Self::Pipe),
            "Wastewater Pump" => Ok(Self::Pump),
            "Wastewater Network Structure" => Ok(Self::Structure),
            "Wastewater Valve" => Ok(Self::Valve),
            _ => Err(aid::prelude::Bandage::Hint(format!(
                "Unrecognized EventKind: {s}"
            ))),
        }
    }
}

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
/// Maintenance type categories for Beehive.
pub enum Maintenance {
    /// Beehive maintenance type "Annual Maintenance"
    Annual,
    /// Beehive maintenance type "Maintenance (Capital)"
    Capital,
    /// Beehive maintenance type "Disposal"
    Disposal,
    /// Beehive maintenance types "Maintenance (Emergency)" and "Emergency Inspection"
    Emergency,
    /// Beehive maintenance type "Expansion"
    Expansion,
    /// Beehive maintenance type "Inspection", "Problem Inspection" and "Routine Inspection"
    Inspection,
    /// Beehive maintenance type "Monthly Maintenance"
    Monthly,
    /// Beehive maintenance type "Operate"
    Operate,
    /// Beehive maintenance type "Quarterly Maintenance"
    Quarterly,
    /// Beehive maintenance type "Maintenance (Regular)"
    // Given an arbitrary Default value to enable default initialization of any parent struct.
    #[default]
    Regular,
    /// Beehive maintenance type "Replace"
    Replace,
    /// Beehive maintenance type "Semi-Annual Maintenance"
    SemiAnnual,
    /// Beehive maintenance type "Upgrade"
    Upgrade,
    /// Beehive maintenance type "Weekly Maintenance"
    Weekly,
}

impl std::str::FromStr for Maintenance {
    type Err = aid::prelude::Bandage;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Annual Maintenance" => Ok(Self::Annual),
            "Maintenance (Capital) " => Ok(Self::Capital),
            "Disposal" => Ok(Self::Disposal),
            "Maintenance (Emergency)" => Ok(Self::Emergency),
            "Emergency Inspection" => Ok(Self::Emergency),
            "Expansion" => Ok(Self::Expansion),
            "Inspection" => Ok(Self::Inspection),
            "Problem Inspection" => Ok(Self::Inspection),
            "Routine Inspection" => Ok(Self::Inspection),
            "Monthly Maintenance" => Ok(Self::Monthly),
            "Operate" => Ok(Self::Operate),
            "Quarterly Maintenance" => Ok(Self::Quarterly),
            "Maintenance (Regular)" => Ok(Self::Regular),
            "Replace" => Ok(Self::Replace),
            "Semi-Annual Maintenance" => Ok(Self::SemiAnnual),
            "Upgrade" => Ok(Self::Upgrade),
            "Weekly Maintenance" => Ok(Self::Weekly),
            _ => Err(aid::prelude::Bandage::Hint(format!(
                "Unrecognized EventKind: {s}"
            ))),
        }
    }
}

impl Maintenance {
    /// The `from_raw` method converts String values from the `maintenance` field of a [`EventRaw`]
    /// into an `Option<Self>`.
    pub fn from_raw(s: &Option<String>) -> Option<Self> {
        if let Some(value) = s {
            match Self::from_str(value) {
                Ok(result) => Some(result),
                Err(e) => {
                    tracing::warn!(
                        "Could not parse maintenance type: {value}. Error: {}",
                        e.to_string()
                    );
                    None
                }
            }
        } else {
            None
        }
    }
}

/// The `Priority` enum holds the possible variants for the priority level of Beehive Events.
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
pub enum Priority {
    /// The most urgent category, "1 - High"
    High,
    /// Medium-High category.
    MediumHigh,
    /// Medium category, the default setting for a new event.
    #[default]
    Medium,
    /// Medium-Low category.
    MediumLow,
    /// Lowest priority level.
    Low,
}

impl std::str::FromStr for Priority {
    type Err = aid::prelude::Bandage;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1 - High" => Ok(Self::High),
            "2" => Ok(Self::MediumHigh),
            "3" => Ok(Self::Medium),
            "4" => Ok(Self::MediumLow),
            "5 - Low" => Ok(Self::Low),
            _ => Err(aid::prelude::Bandage::Hint(format!(
                "Unrecognized Priority: {s}"
            ))),
        }
    }
}

/// The `Priority` enum holds the possible variants for the priority level of Beehive Events.
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
pub enum Status {
    /// From event status "Complete".
    // Given an arbitrary Default value to enable default initialization of any parent struct.
    #[default]
    Complete,
    /// From event status "In Progress".
    InProgress,
    /// From event status "On Hold".
    OnHold,
}

impl std::str::FromStr for Status {
    type Err = aid::prelude::Bandage;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Complete" => Ok(Self::Complete),
            "In Progress" => Ok(Self::InProgress),
            "On Hold" => Ok(Self::OnHold),
            _ => Err(aid::prelude::Bandage::Hint(format!(
                "Unrecognized Status: {s}"
            ))),
        }
    }
}

impl Status {
    /// The `from_raw` method converts String values from the `priority` field of a [`EventRaw`]
    /// into an `Option<Self>`.
    pub fn from_raw(s: &Option<String>) -> Option<Self> {
        if let Some(value) = s {
            match Self::from_str(value) {
                Ok(result) => Some(result),
                Err(e) => {
                    tracing::warn!("Could not parse status: {value}. Error: {}", e.to_string());
                    None
                }
            }
        } else {
            None
        }
    }
}
