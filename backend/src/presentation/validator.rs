use crate::presentation::errors::{AppError, ErrorBody};
use axum::{
    Json,
    extract::{FromRequest, Query, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

impl From<axum::extract::rejection::JsonRejection> for AppError {
    fn from(_: axum::extract::rejection::JsonRejection) -> Self {
        AppError::BadRequest(ErrorBody {
            code: "400",
            message: "Invalid JSON input".to_string(),
        })
    }
}

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + Send + 'static,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(AppError::from)?;

        if let Err(e) = value.validate() {
            let violations = e
                .field_errors()
                .into_iter()
                .flat_map(|(_, errs)| {
                    errs.iter()
                        .filter_map(|err| err.message.clone().map(|m| m.to_string()))
                })
                .collect::<Vec<_>>();

            return Err(AppError::BadRequest(ErrorBody {
                code: "400",
                message: if violations.is_empty() {
                    "Validation failed".into()
                } else {
                    format!("Validation failed: [{}]", violations.join(", "))
                },
            }));
        }

        Ok(ValidatedJson(value))
    }
}

impl From<axum::extract::rejection::QueryRejection> for AppError {
    fn from(_: axum::extract::rejection::QueryRejection) -> Self {
        AppError::BadRequest(ErrorBody {
            code: "400",
            message: "Invalid query input".to_string(),
        })
    }
}

pub struct ValidatedQuery<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedQuery<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + Send + 'static,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request(req, state)
            .await
            .map_err(AppError::from)?;

        if let Err(e) = value.validate() {
            let violations = e
                .field_errors()
                .into_iter()
                .flat_map(|(_, errs)| {
                    errs.iter()
                        .filter_map(|err| err.message.clone().map(|m| m.to_string()))
                })
                .collect::<Vec<_>>();

            return Err(AppError::BadRequest(ErrorBody {
                code: "400",
                message: if violations.is_empty() {
                    "Validation failed".into()
                } else {
                    format!("Validation failed: [{}]", violations.join(", "))
                },
            }));
        }

        Ok(ValidatedQuery(value))
    }
}

impl From<axum::extract::rejection::PathRejection> for AppError {
    fn from(_: axum::extract::rejection::PathRejection) -> Self {
        AppError::BadRequest(ErrorBody {
            code: "400",
            message: "Invalid path input".to_string(),
        })
    }
}
