// #![allow(dead_code)]

use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct CargoFile {
    package: Option<PackageConfig>,
}

#[derive(Debug, Deserialize)]
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

fn main() {
    let cargo_toml_result = read_cargo_file();

    let file_name = cargo_toml_result.unwrap().package.unwrap().name.unwrap();
    let current_working_dir = env::current_dir().unwrap().into_os_string();

    let current_working_dir_string = current_working_dir;
    let cwd_temp = current_working_dir_string.to_str().unwrap();
    let path_from_str = format!("{}/target/release/{}", cwd_temp, file_name);
    let path_from = Path::new(&path_from_str);

    let path_to_str = format!("/Users/gage/cmd-line-programs/rust/{}", file_name);
    let path_to = Path::new(&path_to_str);

    copy(path_from, path_to);
    let file_size = path_to.metadata().unwrap().len();
    println!("FILE SIZE: {} bytes", file_size)
}
