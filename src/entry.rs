use deb_architectures::Architecture;
use std::str::FromStr;
use super::ImageSize;

/// The hash, size, and path of a file that this release file points to.
#[derive(Debug, Default, Clone, Hash, PartialEq)]
pub struct ReleaseEntry {
    pub sum: String,
    pub size: u64,
    pub path: String,
}

impl ReleaseEntry {
    pub fn variant(&self) -> Option<ReleaseVariant> {
        fn extension_from(input: &str, len: usize) -> Option<String> {
            if input.len() == len {
                None
            } else {
                Some(input[len+1..].to_owned())
            }
        }

        fn type_with_extension<T: FromStr>(input: &str) -> Option<(T, Option<String>)> {
            eprintln!("twe: {}", input);
            let (kind, ext) = match input.find('.') {
                Some(pos) => (&input[..pos], Some(input[pos+1..].to_owned())),
                None => (input, None)
            };

            eprintln!("kind: {}", kind);
            kind.parse::<T>().ok().map(|kind| (kind, ext))
        }

        let mut path = self.path.as_str();
        let mut found = false;
        while let Some(pos) = path.find('/') {
            found = true;
            let base = &path[..pos];

            match base {
                _ if base.starts_with("binary-") => {
                    let binary = &path[7..];

                    return binary.find('/').and_then(|pos| {
                        eprintln!("binary: {}", &binary[..pos]);
                        binary[..pos].parse::<Architecture>().ok().and_then(|arch| {
                            let filename = &binary[pos+1..];
                            if filename.starts_with("Packages") {
                                let ext = extension_from(filename, 8);
                                Some(ReleaseVariant::Binary(Binary::Packages(ext), arch))
                            } else if filename.starts_with("Release") {
                                Some(ReleaseVariant::Binary(Binary::Release, arch))
                            } else {
                                None
                            }
                        })
                    })
                }
                "debian-installer" => {
                    return None
                    // TODO
                },
                "dep11" => {
                    let path = &path[6..];
                    return if path.starts_with("icons-") {
                        type_with_extension::<ImageSize>(&path[6..])
                            .map(|(res, ext)| ReleaseVariant::Dep11(Dep11::Icons(res, ext)))
                    } else if path.starts_with("Components-") {
                        type_with_extension::<Architecture>(&path[11..])
                            .map(|(arch, ext)| ReleaseVariant::Dep11(Dep11::Components(arch, ext)))
                    } else {
                        None
                    }
                }
                "i18n" => {
                    let path = &path[5..];
                    return if path.starts_with("Translation") {
                        type_with_extension::<String>(&path[12..])
                            .map(|(loc, ext)| ReleaseVariant::I18n(I18n::Translations(loc, ext)))
                    } else if path == "Index" {
                        Some(ReleaseVariant::I18n(I18n::Index))
                    } else {
                        None
                    }
                }
                "source" => {
                    let path = &path[7..];
                    return if path.starts_with("Sources") {
                        let ext = extension_from(path, 7);
                        Some(ReleaseVariant::Source(Source::Sources(ext)))
                    } else if path == "Release" {
                        Some(ReleaseVariant::Source(Source::Release))
                    } else {
                        None
                    }
                }
                _ => path = &path[pos+1..],
            }
        }

        if ! found && self.path.starts_with("Contents-") {
            return type_with_extension::<Architecture>(&self.path[9..])
                .map(|(arch, ext)| ReleaseVariant::Contents(arch, ext));
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

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum ReleaseVariant {
    Binary(Binary, Architecture),
    Contents(Architecture, Option<String>),
    Dep11(Dep11),
    Source(Source),
    I18n(I18n),
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Dep11 {
    Components(Architecture, Option<String>),
    Icons(ImageSize, Option<String>)
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum I18n {
    Index,
    // A translation file, with a language code and optional compression.
    Translations(String, Option<String>),
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Binary {
    /// A Packages list, which may optionally be compressed.
    Packages(Option<String>),
    Release
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Source {
    Sources(Option<String>),
    Release
}
