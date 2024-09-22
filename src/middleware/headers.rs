use std::{
    future::{ready, Ready},
    pin::Pin,
    task::{Context, Poll},
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    rt::time::Instant,
    Error,
};

pub struct Headers;

impl<S, B> Transform<S, ServiceRequest> for Headers
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = HeadersMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(HeadersMiddleware { service }))
    }
}

pub struct HeadersMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for HeadersMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let now = Instant::now();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let elapsed = now.elapsed().as_millis();
            //println!("Request '{}' took {}μs", res.request().path(), elapsed);
            println!("Request '{}' took {}ms", res.request().path(), elapsed);
            Ok(res)
        })
    }
}
