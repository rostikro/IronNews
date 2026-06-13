use axum::Form;
use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InputData {
    name: String,
    email: String,
}

pub async fn subscribe(data: Result<Form<InputData>, FormRejection>) -> StatusCode {
    match data {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
