use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrResponse<E> {
    pub success: bool,
    pub message: String,
    pub error: E,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrMessage {
    pub message: String,
}
