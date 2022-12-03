#![allow(dead_code)]

use tracing::{debug, error, info, span, warn, Level};

mod config;
mod utility;
mod shapes;
mod camera;
mod scene;
mod ray_tracer;
mod sampler;

use utility::image;

fn main() {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    config::validate_config();
}
