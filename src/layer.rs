use tower::Layer;

use crate::MetricsService;

#[derive(Debug, Clone)]
pub struct MetricsLayer;

impl<S> Layer<S> for MetricsLayer {
    type Service = MetricsService<S>;

    #[inline]
    fn layer(&self, inner: S) -> Self::Service {
        Self::Service::new(inner)
    }
}
