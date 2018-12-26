extern crate chrono;
extern crate deb_architectures;
#[macro_use]
extern crate smart_default;

mod entry;
mod image_size;
mod time;

pub use self::entry::*;
pub use self::image_size::*;

use self::time::get_time;
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;
use std::{fs, io};

/// The dist release file is a file in the apt repository that points to all other dist files in the archive.
#[derive(Debug, SmartDefault, Clone, PartialEq)]
pub struct DistRelease {
    pub architectures: Vec<String>,
    pub codename: String,
    pub components: Vec<String>,
    #[default = "Utc::now()"]
    pub date: DateTime<Utc>,
    pub description: String,
    pub label: String,
    pub origin: String,
    pub suite: String,
    pub version: String,
    pub sums: BTreeMap<String, EntryComponents>,
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

        fn get_vec(value: &str) -> Vec<String> {
            value.split_whitespace().map(String::from).collect()
        }

        let mut sum = None;

        while !entries.is_empty() {
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
                        Variant::Version => release.version = get_string(value),
                    }
                }
            }

            if let Some(pos) = remove.take() {
                entries.remove(pos);
            } else if line.ends_with(':') {
                sum = Some(line.trim()[..line.len() - 1].to_owned());
                break
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown key in release file: {}", line),
                ));
            }
        }

        let mut active_hash = sum.unwrap_or_default();
        let mut active_components = EntryComponents::default();

        for line in iterator {
            if line.starts_with(' ') {
                match line.parse::<ReleaseEntry>() {
                    Ok(mut entry) => {
                        let mut path = String::new();
                        ::std::mem::swap(&mut path, &mut entry.path);
                        let base = match path.find('.') {
                            Some(pos) => &path[..pos],
                            None => &path,
                        };

                        match path.find('/') {
                            Some(pos) => {
                                let component = &path[..pos];
                                // TODO: Prevent this allocation.
                                entry.path = path[pos + 1..].to_owned();

                                active_components
                                    .components
                                    .entry(component.into())
                                    .and_modify(|e| {
                                        e.entry(base.into())
                                            .and_modify(|e| e.push(entry.to_owned()))
                                            .or_insert_with(|| vec![entry.to_owned()]);
                                    })
                                    .or_insert_with(|| {
                                        let mut map = BTreeMap::new();
                                        map.insert(base.into(), vec![entry.to_owned()]);
                                        map
                                    });
                            }
                            None => {
                                entry.path = path.clone();
                                active_components
                                    .base
                                    .entry(base.into())
                                    .and_modify(|e| e.push(entry.to_owned()))
                                    .or_insert_with(|| vec![entry.to_owned()]);
                            }
                        }
                    }
                    Err(why) => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("invalid checksum entry: {}", why),
                        ))
                    }
                }
            } else {
                if !active_components.is_empty() {
                    release
                        .sums
                        .insert(active_hash.clone(), active_components.clone());
                    active_components.clear();
                }

                active_hash.clear();
                active_hash.push_str(line.trim());
                active_hash.pop();
            }
        }

        if !active_components.is_empty() {
            release.sums.insert(active_hash, active_components);
        }

        Ok(release)
    }
}

/// Stores the entries for each component for this checksum method.
#[derive(Debug, Default, Clone, Hash, PartialEq)]
pub struct EntryComponents {
    pub base: BTreeMap<String, Vec<ReleaseEntry>>,
    pub components: BTreeMap<String, BTreeMap<String, Vec<ReleaseEntry>>>,
}

impl EntryComponents {
    pub fn clear(&mut self) {
        self.base.clear();
        self.components.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.base.is_empty() && self.components.is_empty()
    }
}
