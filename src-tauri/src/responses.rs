use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormItem {
    pub name: String,
    pub location: String,
    pub update_time: String,
    pub is_backups: bool,
}