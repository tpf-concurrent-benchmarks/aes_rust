extern crate statsd;

use std::net::ToSocketAddrs;
use crate::metrics_logger::MetricsLogger;

pub struct StatsDMetricsLogger {
    statsd_client: statsd::Client,
}

impl StatsDMetricsLogger {
    pub fn new<T: ToSocketAddrs>(host: T, prefix: &str) -> Self {
        let statsd_client = statsd::Client::new(host, prefix).expect("Failed to create statsd client");

        StatsDMetricsLogger { statsd_client }
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
        println!("Running and measuring {}", metric);
        self.statsd_client.time(metric, f)
    }
}
