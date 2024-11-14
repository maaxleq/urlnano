use axum::{http::StatusCode, response::IntoResponse};

pub struct HttpError {
    pub msg: String,
    pub status: StatusCode,
}

impl HttpError {
    pub fn not_found(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            status: StatusCode::NOT_FOUND,
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.msg).into_response()
    }
}
