// #![allow(dead_code)]

use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Deserialize)]
struct CargoFile {
    package: Option<PackageConfig>,
}

#[derive(Deserialize)]
struct PackageConfig {
    name: Option<String>,
    version: Option<String>,
    edition: Option<String>,
}

fn read_cargo_file() -> std::io::Result<CargoFile> {
    let content = std::fs::read_to_string("cargo.toml")?;
    Ok(toml::from_str(&content)?)
}

fn copy(from_path: &Path, to_path: &Path) -> std::io::Result<()> {
    fs::copy(from_path, to_path)?;
    Ok(())
}

fn get_file_name(cargo: CargoFile) -> String {
    cargo.package.unwrap().name.unwrap()
}

fn get_source_path(file_name: &String) -> PathBuf {
    let mut current_working_dir = env::current_dir().unwrap();
    current_working_dir.push("target");
    current_working_dir.push("release");
    current_working_dir.push(file_name);
    return current_working_dir;
}

fn get_target_path(file_name: &String) -> PathBuf {
    let mut target = PathBuf::new();
    target.push("/Users/gage/cmd-line-programs/rust/");
    target.push(file_name);
    return target;
}

fn main() {
    let cargo_toml_result = read_cargo_file();

    let file_name = get_file_name(cargo_toml_result.unwrap());

    let source_path = get_source_path(&file_name);
    let target_path = get_target_path(&file_name);

    println!("{}", source_path.display());
    println!("{}", target_path.display());

    let _result = copy(&source_path, &target_path);
    let file_size = target_path.metadata().unwrap().len();
    println!("FILE SIZE: {} bytes", file_size)
}
