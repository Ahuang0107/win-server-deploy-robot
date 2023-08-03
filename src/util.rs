#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(flatten)]
    pub profiles: std::collections::HashMap<String, ServerConfig>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub target_folder: String,
    pub files: Vec<String>,
}

pub fn set_usr(username: &str) -> String {
    format!("$Username = \"{username}\"")
}

pub fn set_pwd(password: &str) -> String {
    format!("$Password = ConvertTo-SecureString \"{password}\" -AsPlainText -Force")
}

pub fn set_crdl() -> String {
    format!(
        "$Credential = New-Object System.Management.Automation.PSCredential($Username,$Password)"
    )
}

pub fn session(host: &str) -> String {
    format!("$Session = New-PSSession -ComputerName \"{host}\" -Credential $Credential")
}

pub fn new_item(path: &str) -> String {
    format!("New-Item -ItemType Directory -Path \"{path}\" -Force -ToSession $Session")
}

pub fn move_item(from: &str, to: &str) -> String {
    format!("Move-Item -Path \"{from}\" -Destination \"{to}\" -ToSession $Session")
}

pub fn copy_item(from: &str, to: &str) -> String {
    format!("Copy-Item -Path \"{from}\" -Destination \"{to}\" -ToSession $Session")
}

pub fn invoke(command: &Vec<String>) -> String {
    let commands = command.join("\n");
    format!("Invoke-Command -ToSession $Session -ScriptBlock {{ {commands} }}")
}
