use crate::event::InputId;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IoRequest {
    SaveWorkspace,
    LoadWorkspace,
    ImportImage,
    InputText { id: InputId },
}
