//! The `event` module defines the `Event` type, containing the Beehive Event and the asset
//! pertaining to the event.
use crate::convert;
use crate::import::beehive;
use crate::import::utilities::wastewater::{device, junction, line};
use std::io::prelude::Write;
use std::{fs, path};

/// The `DeviceEvent` struct relates a Beehive event to the corresponding asset on which the event
/// occurred.  The `asset` field holds the asset and the `event` field holds the event.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeviceEvent {
    asset: device::Device,
    event: beehive::Event,
}

impl DeviceEvent {
    /// Constructs a new `DeviceEvent` from its constituent parts.
    pub fn new(asset: device::Device, event: beehive::Event) -> Self {
        Self { asset, event }
    }

    /// The `feature` method converts the `DeviceEvent` to a [`geojson::Feature`].
    pub fn feature(&self) -> geojson::Feature {
        let mut result = convert::Convert::new(self.asset.geometry.clone()).geojson_feature();
        result.set_property("asset_id", self.asset.asset_id().clone());
        result.set_property("historic_id", self.asset.historic_id().clone());
        result.set_property("event_id", self.event.asset_id().clone());
        result.set_property("asset_group", self.asset.asset_group());
        result.set_property("asset_type", self.asset.asset_type());
        result.set_property("owner", self.asset.owner());
        result.set_property("asset_kind", self.event.asset_kind().to_string());
        result.set_property("assigned_to", self.event.assigned_to().clone());
        result.set_property("created_by", self.event.created_by().clone());
        result.set_property("create_date", self.event.create_date().to_string());
        result.set_property(
            "maintenance",
            self.event.maintenance().clone().map(|v| v.to_string()),
        );
        result.set_property("modify_date", self.event.modify_date().to_string());
        result.set_property("modified_by", self.event.modified_by().clone());
        result.set_property("name", self.event.name().clone());
        result.set_property("notes", self.event.notes().clone());
        result.set_property("event_kind", self.event.kind().to_string());
        result.set_property("plan_date", self.event.plan_date().to_string());
        result.set_property("priority", self.event.priority().to_string());
        result.set_property("schedule_time", self.event.schedule_time().to_string());
        result.set_property("status", self.event.status().clone().map(|v| v.to_string()));
        result.set_property(
            "manhole_card",
            self.event
                .manhole_card()
                .clone()
                .map(|v| v.display().to_string()),
        );
        if let Some(cctv) = self.event.cctv() {
            let report = cctv.file().path().display().to_string();
            result.set_property("cctv", report);
        }
        result
    }
}

/// The `DeviceEvents` struct is a wrapper around a vector of type [`DeviceEvent`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct DeviceEvents(Vec<DeviceEvent>);

impl DeviceEvents {
    /// Creates a new `DeviceEvents` from a vector of type [`DeviceEvent`].
    pub fn new(events: Vec<DeviceEvent>) -> Self {
        Self(events)
    }

    /// The `feature_collection` method converts a `DeviceEvents` into a
    /// [`geojson::FeatureCollection`].
    pub fn feature_collection(&self) -> geojson::FeatureCollection {
        self.iter().map(|v| v.feature()).collect()
    }

    /// The `geojson` method exports the contents of Self to the file location at `path`.
    #[tracing::instrument(skip(path))]
    pub fn geojson<P: AsRef<path::Path>>(&self, path: P) -> aid::prelude::Clean<()> {
        let contents = self.feature_collection().to_string().into_bytes();
        let mut file = fs::File::create(path)?;
        file.write_all(&contents)?;
        Ok(())
    }
}

impl TryFrom<(&beehive::Events, &device::Devices)> for DeviceEvents {
    type Error = aid::prelude::Bandage;

    fn try_from(value: (&beehive::Events, &device::Devices)) -> Result<Self, Self::Error> {
        let events = value.0.clone();
        if let Some(res) = events.from_devices(value.1) {
            Ok(res)
        } else {
            Err(aid::prelude::Bandage::Hint(
                "No events returned.".to_string(),
            ))
        }
    }
}

/// The `LineEvent` struct relates a Beehive event to the corresponding asset on which the event
/// occurred.  The `asset` field holds the asset and the `event` field holds the event.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LineEvent {
    asset: line::Line,
    event: beehive::Event,
}

impl LineEvent {
    /// Constructs a new `LineEvent` from its constituent parts.
    pub fn new(asset: line::Line, event: beehive::Event) -> Self {
        Self { asset, event }
    }

    /// The `feature` method converts the `DeviceEvent` to a [`geojson::Feature`].
    pub fn feature(&self) -> geojson::Feature {
        let mut result = convert::Convert::new(self.asset.geometry.clone()).geojson_feature();
        result.set_property("asset_id", self.asset.asset_id().clone());
        result.set_property("historic_id", self.asset.historic_id().clone());
        result.set_property("event_id", self.event.asset_id().clone());
        result.set_property("asset_group", self.asset.asset_group());
        result.set_property("asset_type", self.asset.asset_type());
        result.set_property("owner", self.asset.owner());
        result.set_property("asset_kind", self.event.asset_kind().to_string());
        result.set_property("assigned_to", self.event.assigned_to().clone());
        result.set_property("created_by", self.event.created_by().clone());
        result.set_property("create_date", self.event.create_date().to_string());
        result.set_property(
            "maintenance",
            self.event.maintenance().clone().map(|v| v.to_string()),
        );
        result.set_property("modify_date", self.event.modify_date().to_string());
        result.set_property("modified_by", self.event.modified_by().clone());
        result.set_property("name", self.event.name().clone());
        result.set_property("notes", self.event.notes().clone());
        result.set_property("event_kind", self.event.kind().to_string());
        result.set_property("plan_date", self.event.plan_date().to_string());
        result.set_property("priority", self.event.priority().to_string());
        result.set_property("schedule_time", self.event.schedule_time().to_string());
        result.set_property("status", self.event.status().clone().map(|v| v.to_string()));
        result.set_property(
            "manhole_card",
            self.event
                .manhole_card()
                .clone()
                .map(|v| v.display().to_string()),
        );
        if let Some(cctv) = self.event.cctv() {
            let report = cctv.file().path().display().to_string();
            result.set_property("cctv", report);
        }
        result
    }
}

/// The `LineEvents` struct is a wrapper around a vector of type [`LineEvent`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct LineEvents(Vec<LineEvent>);

impl LineEvents {
    /// Creates a new `LineEvents` from a vector of type [`LineEvent`].
    pub fn new(events: Vec<LineEvent>) -> Self {
        Self(events)
    }

    /// The `feature_collection` method converts a `LineEvents` into a
    /// [`geojson::FeatureCollection`].
    pub fn feature_collection(&self) -> geojson::FeatureCollection {
        self.iter().map(|v| v.feature()).collect()
    }

    /// The `geojson` method exports the contents of Self to the file location at `path`.
    pub fn geojson<P: AsRef<path::Path>>(&self, path: P) -> aid::prelude::Clean<()> {
        let contents = self.feature_collection().to_string().into_bytes();
        let mut file = fs::File::create(path)?;
        file.write_all(&contents)?;
        Ok(())
    }
}

/// The `JunctionEvent` struct relates a Beehive event to the corresponding asset on which the event
/// occurred.  The `asset` field holds the asset and the `event` field holds the event.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JunctionEvent {
    asset: junction::Junction,
    event: beehive::Event,
}

impl JunctionEvent {
    /// Constructs a new `JunctionEvent` from its constituent parts.
    pub fn new(asset: junction::Junction, event: beehive::Event) -> Self {
        Self { asset, event }
    }

    /// The `feature` method converts the `JunctionEvent` to a [`geojson::Feature`].
    pub fn feature(&self) -> geojson::Feature {
        let mut result = convert::Convert::new(self.asset.geometry.clone()).geojson_feature();
        result.set_property("asset_id", self.asset.asset_id().clone());
        result.set_property("historic_id", self.asset.historic_id().clone());
        result.set_property("event_id", self.event.asset_id().clone());
        result.set_property("asset_group", self.asset.asset_group());
        result.set_property("asset_type", self.asset.asset_type());
        result.set_property("owner", self.asset.owner());
        result.set_property("asset_kind", self.event.asset_kind().to_string());
        result.set_property("assigned_to", self.event.assigned_to().clone());
        result.set_property("created_by", self.event.created_by().clone());
        result.set_property("create_date", self.event.create_date().to_string());
        result.set_property(
            "maintenance",
            self.event.maintenance().clone().map(|v| v.to_string()),
        );
        result.set_property("modify_date", self.event.modify_date().to_string());
        result.set_property("modified_by", self.event.modified_by().clone());
        result.set_property("name", self.event.name().clone());
        result.set_property("notes", self.event.notes().clone());
        result.set_property("event_kind", self.event.kind().to_string());
        result.set_property("plan_date", self.event.plan_date().to_string());
        result.set_property("priority", self.event.priority().to_string());
        result.set_property("schedule_time", self.event.schedule_time().to_string());
        result.set_property("status", self.event.status().clone().map(|v| v.to_string()));
        result.set_property(
            "manhole_card",
            self.event
                .manhole_card()
                .clone()
                .map(|v| v.display().to_string()),
        );
        if let Some(cctv) = self.event.cctv() {
            let report = cctv.file().path().display().to_string();
            result.set_property("cctv", report);
        }
        result
    }
}

/// The `JunctionEvents` struct is a wrapper around a vector of type [`JunctionEvent`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct JunctionEvents(Vec<JunctionEvent>);

impl JunctionEvents {
    /// Creates a new `JunctionEvents` from a vector of type [`JunctionEvent`].
    pub fn new(events: Vec<JunctionEvent>) -> Self {
        Self(events)
    }

    /// The `feature_collection` method converts a `JunctionEvents` into a
    /// [`geojson::FeatureCollection`].
    pub fn feature_collection(&self) -> geojson::FeatureCollection {
        self.iter().map(|v| v.feature()).collect()
    }

    /// The `geojson` method exports the contents of Self to the file location at `path`.
    pub fn geojson<P: AsRef<path::Path>>(&self, path: P) -> aid::prelude::Clean<()> {
        let contents = self.feature_collection().to_string().into_bytes();
        let mut file = fs::File::create(path)?;
        file.write_all(&contents)?;
        Ok(())
    }
}
