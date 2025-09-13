use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};

// Middleware for structured request/response logging using tracing
pub struct TracingLogger;

impl<S, B> Transform<S, ServiceRequest> for TracingLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TracingLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TracingLoggerMiddleware { service }))
    }
}

pub struct TracingLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TracingLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_string();
        let peer_addr = req
            .connection_info()
            .realip_remote_addr()
            .map(|s| s.to_string());
        let fut = self.service.call(req);
        Box::pin(async move {
            let start = std::time::Instant::now();
            let res = fut.await;
            let duration = start.elapsed();
            match &res {
                Ok(resp) => {
                    tracing::info!(
                        method = %method,
                        path = %path,
                        status = %resp.status().as_u16(),
                        duration_ms = duration.as_millis(),
                        remote_addr = ?peer_addr,
                        "Request handled"
                    );
                }
                Err(e) => {
                    tracing::error!(
                        method = %method,
                        path = %path,
                        error = %e,
                        duration_ms = duration.as_millis(),
                        remote_addr = ?peer_addr,
                        "Request error"
                    );
                }
            }
            res
        })
    }
}
