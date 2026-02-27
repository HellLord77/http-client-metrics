use std::fmt::Display;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::task::ready;
use std::time::Instant;

use http::Request;
use http::Response;
use http_body::Body;
use metrics::histogram;
use pin_project_lite::pin_project;
use tower::Service;

use super::request_metrics::RequestMetrics;
use super::response_metrics::ResponseMetrics;

const HTTP_CLIENT_REQUEST_DURATION: &str = "http.client.request.duration";
const HTTP_CLIENT_REQUEST_BODY_SIZE: &str = "http.client.request.body.size";
const HTTP_CLIENT_RESPONSE_BODY_SIZE: &str = "http.client.response.body.size";

pin_project! {
    pub struct MetricsFuture<S, ReqBody>
    where
        S: Service<Request<ReqBody>>,
    {
        #[pin]
        inner: S::Future,
        start: Instant,
        req_met: Option<RequestMetrics>,
    }
}

impl<S, ReqBody> MetricsFuture<S, ReqBody>
where
    S: Service<Request<ReqBody>>,
{
    #[inline]
    pub(super) fn new(inner: S::Future, start: Instant, req_met: RequestMetrics) -> Self {
        Self {
            inner,
            start,
            req_met: Some(req_met),
        }
    }
}

impl<S, ReqBody, ResBody> Future for MetricsFuture<S, ReqBody>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    S::Error: Display,
    ResBody: Body,
{
    type Output = Result<S::Response, S::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let res = ready!(this.inner.poll(cx));
        let duration = this.start.elapsed();

        if let Some(req_met) = this.req_met.take() {
            let res_met = ResponseMetrics::new(&res);
            let request_body_size = req_met.http_client_request_body_size();
            let response_body_size = res_met.http_client_response_body_size();
            let labels = req_met.labels().chain(res_met.labels()).collect::<Vec<_>>();

            histogram!(HTTP_CLIENT_REQUEST_DURATION, labels.iter())
                .record(duration.as_millis() as f64 / 1000.0);
            histogram!(HTTP_CLIENT_REQUEST_BODY_SIZE, labels.iter())
                .record(request_body_size as f64);
            histogram!(HTTP_CLIENT_RESPONSE_BODY_SIZE, labels).record(response_body_size as f64);
        }

        Poll::Ready(res)
    }
}
