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
