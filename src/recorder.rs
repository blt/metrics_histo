use crate::Registry;
use metrics::{self, GaugeValue, Key, Unit};
use metrics_util::{CompositeKey, Handle, MetricKind};

pub struct Recorder {
    registry: Registry<CompositeKey, Handle>,
}

impl Recorder {
    pub fn new(registry: Registry<CompositeKey, Handle>) -> Self {
        Self { registry }
    }
}

impl metrics::Recorder for Recorder {
    fn register_counter(&self, key: Key, _unit: Option<Unit>, _description: Option<&'static str>) {
        let ckey = CompositeKey::new(MetricKind::Counter, key);
        self.registry.op(ckey, |_| {}, Handle::counter);
    }

    fn register_gauge(&self, key: Key, _unit: Option<Unit>, _description: Option<&'static str>) {
        let ckey = CompositeKey::new(MetricKind::Gauge, key);
        self.registry.op(ckey, |_| {}, Handle::gauge);
    }

    fn register_histogram(
        &self,
        key: Key,
        _unit: Option<Unit>,
        _description: Option<&'static str>,
    ) {
        let ckey = CompositeKey::new(MetricKind::Histogram, key);
        self.registry.op(ckey, |_| {}, Handle::histogram);
    }

    fn increment_counter(&self, key: Key, value: u64) {
        let ckey = CompositeKey::new(MetricKind::Counter, key);
        self.registry.op(
            ckey,
            |handle| handle.increment_counter(value),
            Handle::counter,
        );
    }

    fn update_gauge(&self, key: Key, value: GaugeValue) {
        let ckey = CompositeKey::new(MetricKind::Gauge, key);
        self.registry
            .op(ckey, |handle| handle.update_gauge(value), Handle::gauge);
    }

    fn record_histogram(&self, key: Key, value: f64) {
        let ckey = CompositeKey::new(MetricKind::Histogram, key);
        self.registry.op(
            ckey,
            |handle| handle.record_histogram(value),
            Handle::histogram,
        );
    }
}
