pub mod statsd_metrics_logger;
pub use statsd_metrics_logger::StatsDMetricsLogger;

pub trait MetricsLogger {
    fn increment(&self, metric: &str);

    fn decrement(&self, metric: &str);

    fn gauge(&self, metric: &str, value: f64);

    fn run_and_measure<F, T>(&self, metric: &str, f: F) -> T
    where
        F: Fn() -> T;
}
