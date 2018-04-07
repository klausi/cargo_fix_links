extern crate cargo_update;
extern crate git2;
extern crate json;

use cargo_update::ops::{find_package_data, get_index_path};
use git2::Repository;
use std::path::Path;

fn main() {
    // @todo how can I get cargo's home dir from Rust?
    let cargo_dir = Path::new("/home/klausi/.cargo");
    let registry = get_index_path(cargo_dir);
    let registry_repo = Repository::open(&registry).unwrap();
    let latest_registry = registry_repo.revparse_single("origin/master").unwrap();

    // @todo read package name from Cargo.toml.
    let package_data = find_package_data("libz-sys", &latest_registry.as_commit().unwrap().tree().unwrap(), &registry_repo).unwrap();
    let json_string = String::from_utf8(package_data).unwrap();

    let new_data: String = json_string.lines()
        .map(|line| {
            let mut parsed = json::parse(line).unwrap();
            // @todo Read links section from Cargo.toml
            parsed["links"] = "z".into();
            parsed.dump() + "\n"
        })
        .collect();

    println!("{}", new_data);
}
