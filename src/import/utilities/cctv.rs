//! The `cctv` module contains data types and methods for processing Granite CCTV reports on
//! mainline inspections.
use crate::import::utilities::wastewater;
use crate::{convert, utils};
use jiff::civil;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until};
use nom::character::complete::{alpha0, alphanumeric1, digit1, space0};
use nom::combinator::{map_res, opt};
use nom::sequence::delimited;
use std::io::prelude::Write;
use std::{fs, path};

/// The `InspectionFile` struct represents a PDF report produced by Granite summarizing a CCTV
/// inspection.
/// Since the title of each PDF must be parsed successfully into a name, archived name and date, we
/// break reading and parsing the filenames into its own distinct step before trying to match
/// records to assets.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    derive_new::new,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_", into)]
pub struct InspectionFile {
    /// Inspections marked `archive` include an additional name, recorded in this field when
    /// present.
    #[setters(strip_option)]
    #[setters(doc = "Sets the value of the `archive` field.")]
    archive: Option<String>,
    /// The `date` field represents the time associated with the report.
    #[setters(doc = "Sets the value of the `date` field.")]
    date: civil::Date,
    /// The `name` field corresponds to the historic id associated with the asset.
    #[setters(doc = "Sets the value of the `name` field.")]
    name: String,
    #[setters(doc = "Sets the value of the `path` field.")]
    /// The `path` field records the path to the PDF report.
    path: path::PathBuf,
}

impl InspectionFile {
    /// The `read_name` method reads a historic pipe identifier from the given `input`.
    /// Historic pipe identifiers use the historic junction identifiers, going from upstream to
    /// downstream, separated by a period.
    pub fn read_name(input: &str) -> nom::IResult<&str, String> {
        // Upstream junction.
        let (rem, from) = alphanumeric1(input)?;
        // Separator element.
        let (rem, _) = tag(".")(rem)?;
        // Downstream junction.
        let (rem, to) = alphanumeric1(rem)?;
        // Although we could take_until the first space, this method requires the inspection name
        // to follow the naming pattern or throw an error.
        let name = format!("{from}.{to}");
        // Strip any descriptive word inserted between name and date. E.g. UPSTREAM, ARCHIVE
        let (rem, _) = space0(rem)?;
        let (rem, _) = alpha0(rem)?;
        Ok((rem, name))
    }

    /// The title of a CCTV report may list an archived name in parenthesis after the current name,
    /// following the format "current_name (old_name) ARCHIVE date".
    /// The `read_archive` method parses the archived named.
    pub fn read_archive(input: &str) -> nom::IResult<&str, &str> {
        // Strip preceding whitespace.
        let (rem, _) = space0(input)?;
        let (rem, archive) = delimited(tag("("), take_until(")"), tag(")"))(rem)?;
        let (rem, _) = space0(rem)?;
        let (rem, _) = opt(tag("ARCHIVE"))(rem)?;
        Ok((rem, archive))
    }

    /// The `read_mdy` method parses a date from the input.  The format for the date is month, day
    /// and year separated by spaces.
    /// Return in year-month-day format for use by [`jiff`] date constructor.
    /// Since it is the last item to parse in the PDF title, we can return a [`civil::Date`] and
    /// convert the error type to [`aid::prelude::Clean`].
    pub fn read_mdy(input: &str) -> nom::IResult<&str, (i16, i8, i8)> {
        // pub fn read_mdy(input: &str) -> aid::prelude::Clean<civil::Date> {
        // Strip any extra periods.
        let (rem, _) = opt(tag("."))(input)?;
        // Strip preceding whitespace.
        let (rem, _) = space0(rem)?;
        let (rem, month) = map_res(digit1::<&str, nom::error::Error<_>>, str::parse)(rem)?;
        let (rem, _) = space0(rem)?;
        let (rem, day) = map_res(digit1::<&str, nom::error::Error<_>>, str::parse)(rem)?;
        let (rem, _) = space0(rem)?;
        let (rem, year) = map_res(digit1::<&str, nom::error::Error<_>>, str::parse)(rem)?;
        // let date = civil::Date::new(year, month, day)?;
        Ok((rem, (year, month, day)))
    }

    /// The `read_datestamp` method parses a date from the input.  The format for the date is
    /// "_(year)(month)(day)", using an underscore for a prefix and with no other separators.
    pub fn read_datestamp(input: &str) -> nom::IResult<&str, (i16, i8, i8)> {
        let (rem, _) = tag("_")(input)?;
        let (rem, year) = map_res(take(4_usize), str::parse)(rem)?;
        let (rem, month) = map_res(take(2_usize), str::parse)(rem)?;
        let (rem, day) = map_res(take(2_usize), str::parse)(rem)?;
        Ok((rem, (year, month, day)))
    }

    /// The `read_date` method tries to parse the date by two strageties: [`Self::read_mdy`] and
    /// [`Self::read_datestamp`].
    pub fn read_date(input: &str) -> aid::prelude::Clean<civil::Date> {
        let (_, (year, month, day)) = alt((Self::read_mdy, Self::read_datestamp))(input)?;
        let date = civil::Date::new(year, month, day)?;
        Ok(date)
    }

    /// The `from_path` method attempts to construct a new instance from the file `title` and `path`.
    pub fn from_path(title: &str, path: path::PathBuf) -> aid::prelude::Clean<Self> {
        let og_title = title.to_string();
        tracing::trace!("Title is: {}", og_title);
        let (rem, name) = Self::read_name(title)?;
        match Self::read_archive(rem) {
            Ok((remaining, value)) => {
                let archive = Some(value.to_string());
                let date = Self::read_date(remaining)?;
                // let mut path = path::PathBuf::from(title);
                // // let mut path = path.clone();
                // // path.set_file_name(title);
                // path.set_extension("pdf");
                let path = utils::build_path(title, "pdf", path)?;
                tracing::trace!("Path is: {:?}", path);
                Ok(Self {
                    archive,
                    name,
                    date,
                    path,
                })
            }
            Err(e) => {
                tracing::trace!("Archive string not present: {}", e.to_string());
                let archive = None;
                // Build the date from the input before read_archive
                let date = Self::read_date(rem)?;
                // let mut path = path::PathBuf::from(title);
                // // let mut path = path.clone();
                // // path.set_file_name(title);
                // path.set_extension("pdf");
                let path = utils::build_path(title, "pdf", path)?;
                tracing::trace!("Path is: {:?}", path);
                Ok(Self {
                    archive,
                    name,
                    date,
                    path,
                })
            }
        }
    }
}

/// The `InspectionFiles` struct is a wrapper around a vector of type [`InspectionFile`].
/// Uses [`derive_more`] to derive [`derive_more::Deref`] and [`derive_more::DerefMut`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct InspectionFiles(Vec<InspectionFile>);

impl InspectionFiles {
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
            // All CCTV reports are in PDF format.
            if let Some(file_type) = pth.extension() {
                if file_type.to_ascii_lowercase() == "pdf" {
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

    /// The `from_path` method reads the file names from the directory at `path` and attempts to
    /// parse the file names into a vector of type [`InspectionFile`], wrapping the results in
    /// `Self`.
    pub fn from_path<P: AsRef<path::Path>>(path: P) -> aid::prelude::Clean<Self> {
        let dir: path::PathBuf = path.as_ref().into();
        let titles = Self::read_dir(path)?;
        let mut files = Vec::new();
        let mut unmatched = Vec::new();
        titles
            .iter()
            .map(|v| match InspectionFile::from_path(v, dir.clone()) {
                Ok(file) => files.push(file),
                Err(_) => unmatched.push(v),
            })
            .for_each(drop);
        if !unmatched.is_empty() {
            tracing::trace!("{unmatched:#?}");
            tracing::warn!("Could not parse {} titles.", unmatched.len());
        }
        Ok(Self::new(files))
    }
}

/// The `Inspection` struct represents a PDF report produced by Granite summarizing a CCTV
/// inspection, with geometry indicating the position of the asset.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    derive_new::new,
    derive_getters::Getters,
    derive_setters::Setters,
)]
#[setters(prefix = "with_", into)]
pub struct Inspection {
    /// The `asset` field denotes the wastewater asset associated with the inspection.
    #[setters(doc = "Sets the value of the `asset` field.")]
    asset: wastewater::line::Line,
    /// The `file` field holds information about the name and location of the report.
    #[setters(doc = "Sets the value of the `file` field.")]
    file: InspectionFile,
}

impl Inspection {
    /// The `from_file` method creates a new instance of `Inspection` from an [`InspectionFile`]
    /// and a list of `lines` assets.
    pub fn from_file(
        file: &InspectionFile,
        lines: &wastewater::line::Lines,
    ) -> aid::prelude::Clean<Self> {
        let mut lines = lines.clone();
        lines.retain(|v| {
            v.asset_id() == file.name() || *v.historic_id() == Some(file.name().clone())
        });
        if !lines.is_empty() {
            Ok(Self {
                asset: lines[0].clone(),
                file: file.clone(),
            })
        } else {
            Err(aid::prelude::Bandage::Hint(format!(
                "Line match to {} not found.",
                file.name()
            )))
        }
    }

    /// The `feature` method converts an `Inspection` to a [`geojson::Feature`].
    pub fn feature(&self) -> geojson::Feature {
        let mut result = convert::Convert::new(self.asset.geometry.clone()).geojson_feature();
        result.set_property("asset_id", self.asset.asset_id().clone());
        result.set_property("historic_id", self.asset.historic_id().clone());
        result.set_property("owner", self.asset.owner());
        result.set_property("date", self.file.date.to_string());
        result.set_property("path", self.file.path.to_str());
        result
    }

    /// The `date` method returns the date associated with the [`InspectionFile`].
    /// Wraps [`InspectionFile::date`].
    pub fn date(&self) -> &civil::Date {
        self.file.date()
    }

    /// The `path` method returns the path associated with the [`InspectionFile`].
    /// Wraps [`InspectionFile::path`].
    pub fn path(&self) -> &path::PathBuf {
        self.file.path()
    }
}

/// The `Inspections` struct is a wrapper around a vector of type [`Inspection`].
/// Uses [`derive_more`] to derive [`derive_more::Deref`] and [`derive_more::DerefMut`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    derive_new::new,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Inspections(Vec<Inspection>);

impl Inspections {
    /// The `from_files` method creates a new instance of [`Inspections`] from [`InspectionFiles`]
    /// `files` and the list of valid assets `lines`.
    #[tracing::instrument]
    pub fn from_files(
        files: &InspectionFiles,
        lines: &wastewater::line::Lines,
    ) -> aid::prelude::Clean<Self> {
        let mut matched = Vec::new();
        let mut unmatched = Vec::new();
        files
            .iter()
            .map(|v| match Inspection::from_file(v, lines) {
                Ok(value) => matched.push(value),
                Err(e) => {
                    tracing::trace!("Could not match file to asset: {}", e.to_string());
                    unmatched.push(v);
                }
            })
            .for_each(drop);
        tracing::trace!("Matched records: {}", matched.len());
        tracing::trace!("Unmatched records: {}", unmatched.len());
        Ok(Self::new(matched))
    }

    /// The `feature_collection` method converts an `Inspections` into a
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
