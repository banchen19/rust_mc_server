#[derive(Clone)]
pub struct Server_Config {
    pub(crate) server_name: String,
    pub(crate) server_ip: String,
    pub(crate) server_port: i32,
}

impl Server_Config {
    pub fn def_config() -> Self {
        Self {
            server_name: "Rust_BDS".to_owned(),
            server_ip:"0.0.0.0".to_owned(),
            server_port: 19132,
        }
    }
    pub fn new(self) -> Self {
        self
    }
    pub fn getmotd(self) -> String {
        format!("{}{}", self.server_name, self.server_port)
    }
    pub fn get_ip_str(self)->String
    {
        format!("{}:{}", self.server_ip, self.server_port)
    }
}
