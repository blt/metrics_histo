mod filter;
mod recorder;
mod registry;

pub use filter::*;
pub use recorder::*;
pub use registry::*;

use metrics_tracing_context::{MetricsLayer, TracingContextLayer};
use metrics_util::layers::Layer;
use metrics_util::{CompositeKey, Handle};
use once_cell::sync::OnceCell;
use tracing::dispatcher::{set_global_default, Dispatch};
use tracing_subscriber::layer::SubscriberExt;

static CONTROLLER: OnceCell<Controller> = OnceCell::new();

#[derive(Debug)]
pub struct Controller {
    pub registry: Registry<CompositeKey, Handle>,
}

pub fn tracing() {
    let subscriber = tracing_subscriber::registry::Registry::default();
    let subscriber = subscriber.with(MetricsLayer::new());
    let dispatch = Dispatch::new(subscriber);
    let _ = set_global_default(dispatch);
}

pub fn metrics() {
    let registry = Registry::default();
    let controller = Controller {
        registry: registry.clone(),
    };
    CONTROLLER.set(controller).unwrap();

    let recorder = Recorder::new(registry);
    let recorder: Box<dyn metrics::Recorder> =
        Box::new(TracingContextLayer::new(LabelFilter).layer(recorder));

    metrics::set_boxed_recorder(recorder).unwrap();
}

/// Get a handle to the globally registered controller, if it's initialized.
pub fn get_controller() -> &'static Controller {
    CONTROLLER.get().unwrap()
}
