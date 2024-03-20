extern crate statsd;

use super::MetricsLogger;
use std::net::ToSocketAddrs;

pub struct StatsDMetricsLogger {
    statsd_client: statsd::Client,
}

impl StatsDMetricsLogger {
    pub fn new<T: ToSocketAddrs>(host: T, prefix: &str) -> Self {
        let statsd_client =
            statsd::Client::new(host, prefix).expect("Failed to create statsd client");

        StatsDMetricsLogger { statsd_client }
    }
}

impl Default for StatsDMetricsLogger {
    fn default() -> Self {
        let host = "graphite:8125";
        let prefix = "aes_cipher";
        Self::new(host, prefix)
    }
}

impl MetricsLogger for StatsDMetricsLogger {
    fn increment(&self, metric: &str) {
        self.statsd_client.incr(metric);
    }

    fn decrement(&self, metric: &str) {
        self.statsd_client.decr(metric);
    }

    fn gauge(&self, metric: &str, value: f64) {
        self.statsd_client.gauge(metric, value);
    }

    fn run_and_measure<F, T>(&self, metric: &str, f: F) -> T
    where
        F: Fn() -> T,
    {
        self.statsd_client.time(metric, f)
    }
}
