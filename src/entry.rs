use super::ImageSize;
use deb_architectures::Architecture;
use std::str::FromStr;

/// The hash, size, and path of a file that this release file points to.
#[derive(Debug, Default, Clone, Hash, PartialEq)]
pub struct ReleaseEntry {
    pub sum: String,
    pub size: u64,
    pub path: String,
}

impl ReleaseEntry {
    /// If required, the precise variant of an apt entry can be determined here.
    ///
    /// Malformed / unsupported apt entries will return `None`.
    pub fn variant(&self) -> Option<EntryVariant> {
        let mut path = self.path.as_str();
        let mut found = false;
        while let Some(pos) = path.find('/') {
            found = true;
            let base = &path[..pos];

            match base {
                _ if base.starts_with("binary-") => {
                    let binary = &path[7..];

                    return binary.find('/').and_then(|pos| {
                        binary[..pos].parse::<Architecture>().ok().and_then(|arch| {
                            let filename = &binary[pos + 1..];
                            if filename.starts_with("Packages") {
                                let ext = extension_from(filename, 8);
                                Some(EntryVariant::Binary(BinaryEntry::Packages(ext), arch))
                            } else if filename.starts_with("Release") {
                                Some(EntryVariant::Binary(BinaryEntry::Release, arch))
                            } else {
                                None
                            }
                        })
                    });
                }
                "debian-installer" => {
                    return None;
                    // TODO
                }
                "dep11" => {
                    let path = &path[6..];
                    return if path.starts_with("icons-") {
                        type_with_extension::<ImageSize>(&path[6..])
                            .map(|(res, ext)| EntryVariant::Dep11(Dep11Entry::Icons(res, ext)))
                    } else if path.starts_with("Components-") {
                        type_with_extension::<Architecture>(&path[11..]).map(|(arch, ext)| {
                            EntryVariant::Dep11(Dep11Entry::Components(arch, ext))
                        })
                    } else {
                        None
                    };
                }
                "i18n" => {
                    let path = &path[5..];
                    return if path.starts_with("Translation") {
                        type_with_extension::<String>(&path[12..])
                            .map(|(loc, ext)| EntryVariant::I18n(I18nEntry::Translations(loc, ext)))
                    } else if path == "Index" {
                        Some(EntryVariant::I18n(I18nEntry::Index))
                    } else {
                        None
                    };
                }
                "source" => {
                    let path = &path[7..];
                    return if path.starts_with("Sources") {
                        let ext = extension_from(path, 7);
                        Some(EntryVariant::Source(SourceEntry::Sources(ext)))
                    } else if path == "Release" {
                        Some(EntryVariant::Source(SourceEntry::Release))
                    } else {
                        None
                    };
                }
                _ => path = &path[pos + 1..],
            }
        }

        if !found && self.path.starts_with("Contents-") {
            return type_with_extension::<Architecture>(&self.path[9..])
                .map(|(arch, ext)| EntryVariant::Contents(arch, ext));
        }

        None
    }
}

impl FromStr for ReleaseEntry {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iterator = input.split_whitespace();

        let output = Self {
            sum: iterator.next().ok_or("missing sum field")?.to_owned(),
            size: iterator
                .next()
                .ok_or("missing size field")?
                .parse::<u64>()
                .map_err(|_| "size field is not a number")?,
            path: iterator.next().ok_or("missing path field")?.to_owned(),
        };

        Ok(output)
    }
}

/// Defines the kind of file that this apt entry is.
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum EntryVariant {
    Binary(BinaryEntry, Architecture),
    Contents(Architecture, Option<String>),
    Dep11(Dep11Entry),
    Source(SourceEntry),
    I18n(I18nEntry),
}

/// Dep11 entries contain appstream metadata and their required icons.
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Dep11Entry {
    Components(Architecture, Option<String>),
    Icons(ImageSize, Option<String>),
}

/// I18n entries contain translations for a given locale.
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum I18nEntry {
    Index,
    Translations(String, Option<String>),
}

/// Binary entries contain the Packages lists, which dpkg and apt use for dependency resolution.
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum BinaryEntry {
    Packages(Option<String>),
    Release,
}

/// Similar to binary entries, but for source packages.
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum SourceEntry {
    Sources(Option<String>),
    Release,
}

// If the apt entry is not a base length, it has an extension.
fn extension_from(input: &str, len: usize) -> Option<String> {
    if input.len() < len + 1 {
        None
    } else {
        Some(input[len + 1..].to_owned())
    }
}

// Apt entries tend to name a variant with a possible extension (compression).
fn type_with_extension<T: FromStr>(input: &str) -> Option<(T, Option<String>)> {
    let (kind, ext) = match input.find('.') {
        Some(pos) => (&input[..pos], Some(input[pos + 1..].to_owned())),
        None => (input, None),
    };

    kind.parse::<T>().ok().map(|kind| (kind, ext))
}
