use std::collections::HashMap;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use validator::ValidationError;

#[derive(Debug, Deserialize, thiserror::Error)]
#[error("validation error: {0:?}")]
pub struct ValidationErrors(pub HashMap<String, Vec<ValidationError>>);

#[derive(Deserialize)]
struct JsonError<T> {
    error: T,
}


#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("network error")]
    NetworkError(#[from] gloo_net::Error),
    #[error("{0}")]
    ValidationError(#[from] ValidationErrors),
    #[error("{0}")]
    AppError(serde_json::Value),
}


pub struct ApiRequest(Request);


impl ApiRequest {
    pub fn get(url: impl AsRef<str>) -> Self {
        Self(Request::get(url.as_ref()))
    }

    pub fn json(self, json: &impl Serialize) -> Self {
        Self(self.0.json(json).unwrap())
    }

    pub async fn json_response<T: DeserializeOwned>(self) -> Result<T, ApiError> {
        let resp = self.0.send().await.map_err(|err| ApiError::NetworkError(err))?;
        if resp.ok() {
            Ok(resp.json().await?)
        } else {
            let json: JsonError<serde_json::Value> = resp.json().await?;
            Err(ApiError::AppError(json.error))?
        }
    }
}