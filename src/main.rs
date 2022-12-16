#![allow(dead_code)]

use tracing::{debug, error, info, span, warn, Level};

mod config;
mod utility;
mod objects;
mod camera;
mod scene;
mod ray_tracer;
mod sampler;
mod light;
mod integrators;

struct InternalState {
    tracing_subscriber: Box<dyn tracing::Subscriber>,
}

fn main() {
    let internal_state = initialize_internal_state(); 
    config::validate_config();
}

fn initialize_internal_state() -> InternalState {
    let tracing_subscriber = Box::new(
        tracing_subscriber::fmt()
            // Use a more compact, abbreviated log format
            .compact()
            // Display source code file paths
            .with_file(true)
            // Display source code line numbers
            .with_line_number(true)
            // Display the thread ID an event was recorded on
            .with_thread_ids(true)
            // Don't display the event's target (module path)
            .with_target(false)
            // Build the subscriber
            .finish()
    );

    // use that subscriber to process traces emitted after this point
    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    InternalState { 
        tracing_subscriber,
    }
}

