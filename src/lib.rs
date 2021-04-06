mod recorder;
mod registry;

pub use recorder::*;
pub use registry::*;

use metrics_util::{CompositeKey, Handle};
use once_cell::sync::OnceCell;

static CONTROLLER: OnceCell<Controller> = OnceCell::new();

#[derive(Debug)]
pub struct Controller {
    pub registry: Registry<CompositeKey, Handle>,
}

pub fn init() {
    let registry = Registry::default();
    let controller = Controller {
        registry: registry.clone(),
    };
    CONTROLLER.set(controller).unwrap();

    let recorder = Recorder::new(registry);
    let recorder: Box<dyn metrics::Recorder> = Box::new(recorder);

    metrics::set_boxed_recorder(recorder).unwrap();
}

/// Get a handle to the globally registered controller, if it's initialized.
pub fn get_controller() -> &'static Controller {
    CONTROLLER.get().unwrap()
}
