#![allow(dead_code)]

use tracing::{debug, error, info, span, warn, Level};
use std::{env, fs::{File, read_to_string}};

mod config;
mod utility;
mod objects;
mod camera;
mod scene;
mod ray_tracer;
mod sampler;
mod light;
mod integrators;
mod scene_parsing;

struct InternalState {
    tracing_subscriber: Box<dyn tracing::Subscriber>,
}

fn main() {
    initialize_internal_state(); 
    config::validate_config();

    let args: Vec<String> = env::args().collect();

    let scene_file = {
        let filename = args.get(1);
        if filename.is_none() {
            error!("no filename specified (as the 1st argument)");
            panic!();
        }
        
        let file_string = read_to_string(filename.unwrap());
        if file_string.is_err() {
            error!("could not open file '{}'", filename.unwrap());
            panic!();
        }

        file_string.unwrap()
    };

    let mut scene = {
        let json = serde_json::from_str::<serde_json::Value>(&scene_file);
        if json.is_err() {
            error!("couldn't parse json: {}", json.unwrap_err());
            panic!();
        }

        let parsed = scene_parsing::parse_json(&json.unwrap());
        if parsed.is_err() {
            error!("failed to parse scene: {}", parsed.unwrap_err());
            panic!();
        }

        parsed.unwrap()
    };
    info!("finished parsing scene");

    scene.ray_trace();
}

fn initialize_internal_state() {
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
    tracing::subscriber::set_global_default(tracing_subscriber).expect("setting default subscriber failed");

    // InternalState { 
    //     tracing_subscriber,
    // }
}

