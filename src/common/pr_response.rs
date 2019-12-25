use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrResponse<T>  {
    pub msg: String,
    pub code: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> PrResponse<T> {
    pub fn ok(data: T) -> PrResponse<T> {
        PrResponse {
            msg: "success".to_string(),
            code: 0,
            data: Some(data)
        }
    }
}

pub fn error(msg: String) -> PrResponse<String> {
    PrResponse {
        msg: msg,
        code: 1,
        data: None
    }
}