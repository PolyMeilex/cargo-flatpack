#[derive(Debug, serde::Deserialize)]
pub struct LockFile {
    pub version: u32,
    pub package: Vec<Package>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub source: Option<String>,
    pub checksum: Option<String>,
    pub dependencies: Option<Vec<String>>,
}

#[test]
fn abc() {
    let src = std::fs::read_to_string("./Cargo.lock").unwrap();

    let file: LockFile = toml::from_str(&src).unwrap();

    dbg!(file);
}
