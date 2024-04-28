use actix_service::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use bson::doc;
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

use crate::middleware::auth::validate_token;

pub struct AuthCheck;

impl<S, B> Transform<S, ServiceRequest> for AuthCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .map(|value| value.replace("Bearer ", ""))
            .unwrap_or_default();

        let fut = self.service.call(req);

        Box::pin(async move {
            let is_valid = validate_token(&token).await;

            if is_valid.is_err() {
                let message =
                    doc! { "message": "Unauthorized access.", "error": "InvalidSignature" };
                return Err(actix_web::error::ErrorUnauthorized(message.to_string()));
            }

            let res = fut.await?;
            Ok(res)
        })
    }
}
