extern crate apt_release_file;
extern crate deb_architectures;

use apt_release_file::{BinaryEntry, Dep11Entry, ImageSize, ReleaseEntry, EntryVariant, SourceEntry, I18nEntry};
use deb_architectures::Architecture;

#[test]
fn release_entry_binaries() {
    assert_eq!(
        " e58165aee561d376f164717ebe7b89bb            10783 main/binary-ppc64el/Packages.gz"
            .parse::<ReleaseEntry>()
            .map(|r| {
                let variant = r.variant();
                (r, variant)
            })
            .unwrap(),
        (
            ReleaseEntry {
                sum: "e58165aee561d376f164717ebe7b89bb".into(),
                size: 10783,
                path: "main/binary-ppc64el/Packages.gz".into(),
            },
            Some(EntryVariant::Binary(BinaryEntry::Packages(Some("gz".into())), Architecture::Ppc64El))
        )
    );

    assert_eq!(
        " 6cdb0f2a0d80ce797133a49649685685            14184 main/binary-arm64/Packages.xz"
            .parse::<ReleaseEntry>()
            .map(|r| {
                let variant = r.variant();
                (r, variant)
            })
            .unwrap(),
        (
            ReleaseEntry {
                sum: "6cdb0f2a0d80ce797133a49649685685".into(),
                size: 14184,
                path: "main/binary-arm64/Packages.xz".into(),
            },
            Some(EntryVariant::Binary(BinaryEntry::Packages(Some("xz".into())), Architecture::Arm64))
        )
    );
}

#[test]
fn release_entry_translations() {
    assert_eq!(
        " b20ec9bf3bef94dff07ac1f3ca4d826fe0fd0b5cfc3275f34e292f7a79030995            63171 main/i18n/Translation-en"
            .parse::<ReleaseEntry>()
            .map(|r| {
                let variant = r.variant();
                (r, variant)
            })
            .unwrap(),
        (
            ReleaseEntry {
                sum: "b20ec9bf3bef94dff07ac1f3ca4d826fe0fd0b5cfc3275f34e292f7a79030995".into(),
                size: 63171,
                path: "main/i18n/Translation-en".into(),
            },
            Some(EntryVariant::I18n(I18nEntry::Translations("en".to_owned(), None)))
        )
    );

    assert_eq!(
        " 4c233ea1a233462a680ec5be28a576640edb1e2b9ab2da9316b862e3a9c4d8c0            12824 main/i18n/Translation-en.xz"
            .parse::<ReleaseEntry>()
            .map(|r| {
                let variant = r.variant();
                (r, variant)
            })
            .unwrap(),
        (
            ReleaseEntry {
                sum: "4c233ea1a233462a680ec5be28a576640edb1e2b9ab2da9316b862e3a9c4d8c0".into(),
                size: 12824,
                path: "main/i18n/Translation-en.xz".into(),
            },
            Some(EntryVariant::I18n(I18nEntry::Translations("en".to_owned(), Some("xz".into()))))
        )
    );
}

#[test]
fn release_entry_contents() {
    assert_eq!(
        " ef4374f4a7eb9dc65bb51234fce91247         39115618 Contents-arm64.gz"
            .parse::<ReleaseEntry>()
            .map(|r| {
                let variant = r.variant();
                (r, variant)
            })
            .unwrap(),
        (
            ReleaseEntry {
                sum: "ef4374f4a7eb9dc65bb51234fce91247".into(),
                size: 39115618,
                path: "Contents-arm64.gz".into(),
            },
            Some(EntryVariant::Contents(Architecture::Arm64, Some("gz".into())))
        )
    );

    assert_eq!(
        " ef4374f4a7eb9dc65bb51234fce91247         39115618 Contents-arm64"
            .parse::<ReleaseEntry>()
            .map(|r| {
                let variant = r.variant();
                (r, variant)
            })
            .unwrap(),
        (
            ReleaseEntry {
                sum: "ef4374f4a7eb9dc65bb51234fce91247".into(),
                size: 39115618,
                path: "Contents-arm64".into(),
            },
            Some(EntryVariant::Contents(Architecture::Arm64, None))
        )
    );
}

#[test]
fn release_entry_dep11_components() {
    assert_eq!(
        " b6c48cec06853d707de0f23c3d8c989d            20004 main/dep11/Components-ppc64el.yml.xz"
            .parse::<ReleaseEntry>()
            .map(|r| {
                let variant = r.variant();
                (r, variant)
            })
            .unwrap(),
        (
            ReleaseEntry {
                sum: "b6c48cec06853d707de0f23c3d8c989d".into(),
                size: 20004,
                path: "main/dep11/Components-ppc64el.yml.xz".into(),
            },
            Some(EntryVariant::Dep11(Dep11Entry::Components(Architecture::Ppc64El, Some("yml.xz".into()))))
        )
    );
}

#[test]
fn release_entry_dep11_icons() {
    assert_eq!(
        " 31f6566d35ccd604be46ed5b1f813cdf               29 main/dep11/icons-64x64@2.tar.gz"
            .parse::<ReleaseEntry>()
            .map(|r| r.variant())
            .unwrap(),
        Some(EntryVariant::Dep11(Dep11Entry::Icons(
            ImageSize {
                pixels: 64,
                hidpi: 2
            },
            Some("tar.gz".into())
        )))
    );

    assert_eq!(
        " 4a385adab5c0e3f537d4cd32476883a2           173056 main/dep11/icons-48x48.tar"
            .parse::<ReleaseEntry>()
            .map(|r| r.variant())
            .unwrap(),
        Some(EntryVariant::Dep11(Dep11Entry::Icons(
            ImageSize {
                pixels: 48,
                hidpi: 0
            },
            Some("tar".into())
        )))
    );
}

#[test]
fn release_entry_other() {
    assert_eq!(
        " b6c48cec06853d707de0f23c3d8c989d            20004 main/source/Sources.xz"
            .parse::<ReleaseEntry>()
            .map(|r| {
                let variant = r.variant();
                (r, variant)
            })
            .unwrap(),
        (
            ReleaseEntry {
                sum: "b6c48cec06853d707de0f23c3d8c989d".into(),
                size: 20004,
                path: "main/source/Sources.xz".into(),
            },
            Some(EntryVariant::Source(SourceEntry::Sources(Some("xz".into()))))
        )
    );
}
