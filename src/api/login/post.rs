//!
//! Provide the login capabilities to register an user using the credentials
//! 

use axum::response::IntoResponse;
use axum::response::Json;
use serde::{Serialize, Deserialize};

/// UserCredentials defines the required credentials that registering is
/// expecting to be able to successfully register an User into the system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRegisterCredentials {
    username: String,
    password: String
}

///
/// Provides the endpoint to create add a user into the database
/// 
pub async fn register(
    Json(user_request): Json<UserRegisterCredentials>
) -> impl IntoResponse {
    Json::from(user_request)
}