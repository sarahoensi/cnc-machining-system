use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolveThreadRequest {
    #[serde(rename = "type")]
    pub thread_type: String,
    pub size: String,
    pub pitch: String,
}
