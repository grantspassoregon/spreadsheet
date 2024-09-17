//! The `manhole_card` module holds methods for linking scans of manhole cards to manhole assets on
//! the city GIS layer.
//! The city has archived its collection of manhole cards as .jpg files named after the historic id
//! of the asset.
//! Derive setters sets off the clippy documentation lint.
// #![allow(missing_docs)]
use crate::convert;
use crate::import::utilities::wastewater::device::{Device, Devices};
use std::io::prelude::Write;
use std::{fs, path};

/// The `ManholeCard` struct holds methods for extracting the path to PDFs of manhole cards scanned
/// by collections staff.  The purpose is to locate manhole cards that match assets in GIS and
/// attach the path to the card in a field on the GIS layer.
/// Dates are handwritten onto the cards, with no digital transcription.
#[derive(
    Debug,
    Clone,
    PartialEq,
    derive_new::new,
    derive_getters::Getters,
    derive_setters::Setters,
    serde::Serialize,
    serde::Deserialize,
)]
#[setters(prefix = "with_")]
pub struct ManholeCard {
    /// Asset associated with the manhole card.
    asset: Device,
    /// Path to where the file is stored.
    path: path::PathBuf,
}

impl ManholeCard {
    /// The `read_dir` method looks in the directory at `path` and returns the file names contained
    /// within, omitting extensions.
    pub fn read_dir<P: AsRef<path::Path>>(path: P) -> aid::prelude::Clean<Vec<String>> {
        // Create iterator over files in directory.
        let dir_iter = fs::read_dir(path)?;
        // Create empty vector to store results.
        let mut names = Vec::new();
        // Step through files in the directory.
        // Lots of these methods are fallible, so there are a lot of if lets here causing drift
        // rightward.
        for entry in dir_iter {
            let dir = entry?;
            // Strip the leading path from the file name.
            let file = dir.file_name();
            // Convert from OsString to PathBuf to use the file_stem method.
            let pth = path::PathBuf::from(file);
            // Ignore non-jpg file types, there are a couple in there.
            if let Some(file_type) = pth.extension() {
                if file_type.to_ascii_lowercase() == "jpg" {
                    // tracing::info!("Type of file is .jpg");
                    if let Some(stem) = pth.file_stem() {
                        if let Some(name) = stem.to_str() {
                            names.push(name.to_string());
                        }
                    }
                }
            }
        }
        Ok(names)
    }

    /// The `from_device` method matches the asset id of the `device` against the list of file
    /// `names` in `path`.
    /// A device may have more than one manhole card, but each card will have only one device, so
    /// the first stage of matching attaches a device to each card.
    pub fn from_device(device: &Device, names: &[String], path: path::PathBuf) -> Option<Self> {
        if let Some(historic_id) = device.historic_id() {
            let mut names = names.to_vec();
            names.retain(|v| v == device.asset_id() || v == historic_id);
            if !names.is_empty() {
                let mut path = path.clone();
                path.push(names[0].clone());
                path.set_extension("jpg");

                Some(Self {
                    asset: device.clone(),
                    path,
                })
            } else {
                None
            }
        } else {
            // Asset does not have historic id so match with card is not possible.
            None
        }
    }

    /// The `feature` method converts `ManholeCards` to a [`geojson::Feature`].
    pub fn feature(&self) -> geojson::Feature {
        let mut result = convert::Convert::new(self.asset.geometry.clone()).geojson_feature();
        result.set_property("asset_id", self.asset.asset_id().clone());
        result.set_property("historic_id", self.asset.historic_id().clone());
        result.set_property("owner", self.asset.owner());
        result.set_property("path", self.path.to_str());
        result
    }
}

/// The `ManholdCards` struct is a wrapper around a vector of type [`ManholeCard`].
/// Uses [`derive_new`] to derive [`derive_new::new`].
/// Uses [`derive_more`] to derive [`derive_more::Deref`] and [`derive_more::DerefMut`]
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct ManholeCards(Vec<ManholeCard>);

impl ManholeCards {
    /// The `from_devices` method converts a list of manhole card names to the type [`ManholeCards`].
    pub fn from_devices(devices: &Devices, names: &[String], path: path::PathBuf) -> Self {
        let cards = devices
            .iter()
            .filter_map(|d| ManholeCard::from_device(d, names, path.clone()))
            .collect::<Vec<ManholeCard>>();
        Self::new(cards)
    }

    /// The `get` method returns the [`ManholeCard`] contained in Self where the historic id of the card matches `name`.
    pub fn get(&self, name: &str) -> Option<ManholeCard> {
        let mut contains = self.to_vec();
        contains.retain(|v| *v.asset.historic_id() == Some(name.to_string()));
        if contains.is_empty() {
            None
        } else {
            Some(contains[0].clone())
        }
    }

    /// The `contains` method indicates whether Self contains a [`ManholeCard`] with a historic id
    /// matching `name`.
    /// Wraps [`Self::get`].
    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// The `orphans` method returns names in `names` that are not contained within Self.
    /// Calls [`Self::contains`].
    pub fn orphans(&self, names: &[String]) -> Vec<String> {
        let mut orphans = names.to_vec();
        orphans.retain(|name| !self.contains(name));
        orphans
    }

    /// The `feature_collection` method converts a `ManholeCards` into a
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
