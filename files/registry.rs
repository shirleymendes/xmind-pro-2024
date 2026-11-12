//! Windows registry read helpers for Xmind Pro 2024.

pub const APP_KEY: &str = "Software\\Xmind\\Xmind Pro 2024";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Xmind Pro 2024";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Xmind Pro 2024") },
        RegValue { name: "Version",     value: String::from("v1.5.6") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Xmind Pro 2024") },
        RegValue { name: "Publisher",       value: String::from("Xmind") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Xmind Pro 2024\\uninstall.exe\"") },
    ]
}
