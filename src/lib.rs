extern crate chrono;
extern crate deb_architectures;
#[macro_use]
extern crate smart_default;

use chrono::{DateTime, Utc};
use deb_architectures::Architecture;
use std::collections::BTreeMap;
use std::{io, fs};
use std::path::Path;
use std::str::FromStr;

/// The dist release file is a file in the apt repository that points to all other dist files in the archive.
#[derive(Debug, SmartDefault, Clone, PartialEq)]
pub struct DistRelease {
    pub architectures: Vec<String>,
    pub codename: String,
    pub components: Vec<String>,
    #[default(Utc::now())]
    pub date: DateTime<Utc>,
    pub description: String,
    pub label: String,
    pub origin: String,
    pub suite: String,
    pub version: String,
    pub sums: BTreeMap<String, ReleaseEntries>
}

impl DistRelease {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        fs::read_to_string(path).and_then(|string| string.parse::<Self>())
    }
}

impl FromStr for DistRelease {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iterator = input.lines();

        let mut release = DistRelease::default();

        #[derive(Copy, Clone)]
        enum Variant {
            Archs,
            Codename,
            Components,
            Date,
            Description,
            Label,
            Origin,
            Suite,
            Version,
        }

        let mut entries = vec![
            ("Architectures:", Variant::Archs),
            ("Codename:", Variant::Codename),
            ("Components:", Variant::Components),
            ("Date:", Variant::Date),
            ("Description:", Variant::Description),
            ("Label:", Variant::Label),
            ("Origin:", Variant::Origin),
            ("Suite:", Variant::Suite),
            ("Version:", Variant::Version),
        ];

        let mut remove = None;

        fn get_string(value: &str) -> String {
            value.trim().to_owned()
        }

        fn get_time(value: &str) -> io::Result<DateTime<Utc>> {
            let fields = value.split_whitespace().collect::<Vec<&str>>();
            if fields.len() != 6 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("timezone is invalid: should have been six fields: {}", value)
                ));
            }

            let mut buffer: String;
            let value = if fields[5] == "UTC" {
                buffer = fields[..5].join(" ");
                buffer.push(' ');
                buffer.push_str("+0000");
                &buffer
            } else {
                value
            };

            DateTime::parse_from_rfc2822(value)
                .map(|tz| tz.with_timezone(&Utc))
                .map_err(|why| io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unable to parse date ({}) in release file: {}", value, why)
                ))
        }

        fn get_vec(value: &str) -> Vec<String> {
            value.split_whitespace().map(String::from).collect()
        }

        while ! entries.is_empty() {
            let line = iterator.next().unwrap();

            for (id, &(ref key, variant)) in entries.iter().enumerate() {
                if line.starts_with(key) {
                    remove = Some(id);

                    let value = &line[key.len()..];

                    match variant {
                        Variant::Archs => release.architectures = get_vec(value),
                        Variant::Codename => release.codename = get_string(value),
                        Variant::Components => release.components = get_vec(value),
                        Variant::Date => release.date = get_time(value)?,
                        Variant::Description => release.description = get_string(value),
                        Variant::Label => release.label = get_string(value),
                        Variant::Origin => release.origin = get_string(value),
                        Variant::Suite => release.suite = get_string(value),
                        Variant::Version => release.version = get_string(value)
                    }
                }
            }

            if let Some(pos) = remove.take() {
                entries.remove(pos);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown key in release file: {}", line)
                ));
            }
        }

        let mut active_hash = String::new();
        let mut active_entries: BTreeMap<String, Vec<ReleaseEntry>> = BTreeMap::new();

        for line in iterator {
            if line.starts_with(' ') {
                match line.parse::<ReleaseEntry>() {
                    Ok(entry) => {
                        let base = match entry.path.rfind('.') {
                            Some(pos) => entry.path[..pos].to_owned(),
                            None => entry.path.to_owned()
                        };

                        active_entries.entry(base)
                            .and_modify(|e| e.push(entry.clone()))
                            .or_insert_with(|| vec![entry]);
                    },
                    Err(why) => return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid checksum entry: {}", why)
                    ))
                }
            } else {
                if ! active_entries.is_empty() {
                    release.sums.insert(active_hash.clone(), ReleaseEntries(active_entries.clone()));
                    active_entries.clear();
                }

                active_hash.clear();
                active_hash.push_str(line.trim());
                active_hash.pop();
            }
        }

        if ! active_entries.is_empty() {
            release.sums.insert(active_hash, ReleaseEntries(active_entries));
        }

        Ok(release)
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq)]
pub struct ReleaseEntries(pub BTreeMap<String, Vec<ReleaseEntry>>);

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum ReleaseVariant {
    Binary(Architecture),
    Contents(Architecture),
    Translation(String),
}

/// The hash, size, and path of a file that this release file points to.
#[derive(Debug, Default, Clone, Hash, PartialEq)]
pub struct ReleaseEntry {
    pub sum: String,
    pub size: u64,
    pub path: String,
}

impl ReleaseEntry {
    pub fn variant(&self) -> Option<ReleaseVariant> {
        let mut components = self.path.split('/');
        let mut variant = None;
        let mut first = true;

        while let Some(component) = components.next() {
            if component == "source" {
                break
            }

            if component == "i18n" {
                while let Some(component) = components.next() {
                    if let Some(lang) = component.split('-').nth(1) {
                        let lang = match lang.rfind('.') {
                            Some(pos) => &lang[..pos],
                            None => lang
                        };

                        variant = Some(ReleaseVariant::Translation(lang.to_owned()));
                        break
                    }
                }

                break
            }

            if first {
                first = false;
                if component.starts_with("Contents") {
                    if let Some(arch) = component.split('-').nth(1) {
                        let arch = match arch.rfind('.') {
                            Some(pos) => &arch[..pos],
                            None => arch
                        };

                        if let Ok(arch) = arch.parse::<Architecture>() {
                            variant = Some(ReleaseVariant::Contents(arch));
                            break
                        }
                    }
                }
            }

            if let Some(arch) = component.split('-').nth(1) {
                let arch = match arch.rfind('.') {
                    Some(pos) => &arch[..pos],
                    None => arch
                };

                if let Ok(arch) = arch.parse::<Architecture>() {
                    variant = Some(ReleaseVariant::Binary(arch));
                    break
                }
            }
        }

        variant
    }
}

impl FromStr for ReleaseEntry {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iterator = input.split_whitespace();

        let output = Self {
            sum: iterator.next().ok_or("missing sum field")?.to_owned(),
            size: iterator.next()
                .ok_or("missing size field")?
                .parse::<u64>()
                .map_err(|_| "size field is not a number")?,
            path: iterator.next().ok_or("missing path field")?.to_owned(),
        };

        Ok(output)
    }
}
