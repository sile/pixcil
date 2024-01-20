use crate::event::InputId;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IoRequest {
    SaveWorkspace,
    LoadWorkspace,
    ImportImage,
    InputNumber { id: InputId },
    InputSize { id: InputId },
    Vibrate,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InputNumber {
    pub id: InputId,
    pub number: String,
}
