use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    ConfigError(String),
    StartingFrpError,
    KillingFrpError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::ConfigError(e) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(e.into())
                .unwrap(),
            Error::StartingFrpError => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("failed to start frpc".into())
                .unwrap(),
            Error::KillingFrpError => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("failed to kill frpc".into())
                .unwrap(),
        }
    }
}
