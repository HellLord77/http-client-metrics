use std::fmt::Display;
use std::task::Context;
use std::task::Poll;
use std::time::Instant;

use http::Request;
use http::Response;
use http_body::Body;
use tower::Service;

use super::request_metrics::RequestMetrics;
use crate::MetricsFuture;

#[derive(Debug, Clone)]
pub struct MetricsService<S> {
    inner: S,
}

impl<S> MetricsService<S> {
    #[inline]
    pub(super) fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for MetricsService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    S::Error: Display,
    ReqBody: Body,
    ResBody: Body,
{
    type Response = Response<ResBody>;
    type Error = S::Error;
    type Future = MetricsFuture<S, ReqBody>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let req_met = RequestMetrics::new(&req);
        let inner = self.inner.call(req);

        let start = Instant::now();
        Self::Future::new(inner, start, req_met)
    }
}
