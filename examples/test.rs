extern crate apt_release_file;

use apt_release_file::DistRelease;

const RELEASE: &str = include_str!("../tests/Release");

pub fn main() {
    let release = RELEASE.parse::<DistRelease>().unwrap();
    println!("{:#?}", release);

    println!("# Base components in MD5Sum");
    let components = &release.sums["MD5Sum"];
    for components in &components.base {
        println!("{:#?}", components);
    }

    println!("# Components in MD5Sum main");
    for components in &components.components["main"] {
        println!("{:#?}", components);
    }
}
