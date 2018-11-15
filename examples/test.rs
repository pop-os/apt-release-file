extern crate apt_release_file;

use apt_release_file::DistRelease;

const RELEASE: &str = include_str!("../tests/Release");

pub fn main() {
    println!("{:#?}", RELEASE.parse::<DistRelease>());
}
