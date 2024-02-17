extern crate statsd;

use crate::metrics_logger::MetricsLogger;

pub struct NoOpMetricsLogger;

impl NoOpMetricsLogger {
    pub fn new() -> Self {
        NoOpMetricsLogger
    }
}

impl MetricsLogger for NoOpMetricsLogger {
    fn increment(&self, _metric: &str) {}

    fn decrement(&self, _metric: &str) {}

    fn gauge(&self, _metric: &str, _value: f64) {}

    fn run_and_measure<F, T>(&self, _metric: &str, f: F) -> T
        where
            F: Fn() -> T,
    {
        f()
    }
}
