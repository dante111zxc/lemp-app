use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct NginxData {
    pub status: u8,
    pub version: String,
}

#[derive(Serialize, Clone)]
pub struct Nginx {
    pub message: String,
    pub data: NginxData,
}

#[derive(Serialize, Clone)]
pub struct Website {
    pub name: String,
    pub root: String,
    pub enabled: bool,
    pub file_path: String,
}
