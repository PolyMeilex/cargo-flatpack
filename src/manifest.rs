#[derive(Debug, Clone, serde::Serialize)]
pub struct Archive {
    #[serde(rename = "archive-type")]
    pub archive_type: String,
    pub url: String,
    pub sha256: String,
    pub dest: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Inline {
    pub contents: String,
    pub dest: String,
    #[serde(rename = "dest-filename")]
    pub dest_filename: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type")]
pub enum Source {
    #[serde(rename = "archive")]
    Archive(Archive),
    #[serde(rename = "inline")]
    Inline(Inline),
}

#[test]
fn abc() {
    let src = Source::Inline(Inline {
        contents: "a".into(),
        dest: "a".into(),
        dest_filename: "a".into(),
    });

    println!("{}", serde_json::to_string_pretty(&src).unwrap());
}
