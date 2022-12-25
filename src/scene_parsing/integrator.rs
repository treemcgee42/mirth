
// S==== IMPORTS {{{1

use crate::integrators::{traits::IntegratorLike, ambient_occlusion::AmbientOcclusionIntegrator};
use super::parse_error::ParseError;

// E==== IMPORTS }}}1

const KIND_FIELD_NAME: &str = "kind";
const AMBIENT_OCCLUSION_KIND: &str = "ambient occlusion";

const NUM_SAMPLES_FIELD_NAME: &str = "number of samples";
const DEFAULT_NUM_SAMPLES: u32 = 64;

const RECURSION_LIMIT_FIELD_NAME: &str = "ray recursion limit";
const DEFAULT_RECURSION_LIMIT: u32 = 64;

pub struct IntegratorParseOutput {
    pub integrator: Box<dyn IntegratorLike>,
    pub num_samples: u32,
    pub recursion_limit: u32,
}

pub fn new_from_json(json: &serde_json::Value) -> Result<IntegratorParseOutput, ParseError> {
    let integrator = get_integrator(json)?;
    let num_samples = get_num_samples(json)?;
    let recursion_limit = get_recursion_limit(json)?;

    Ok(IntegratorParseOutput {
        integrator,
        num_samples,
        recursion_limit
    })
}

fn get_integrator(json: &serde_json::Value) -> Result<Box<dyn IntegratorLike>, ParseError> {
    let integrator_name = match serde_json::from_value::<String>(json[KIND_FIELD_NAME].clone()) {
        Ok(s) => s,
        Err(_) => {
            let pe = ParseError {
                msg: format!("could not parse field '{}' for 'integrator'", KIND_FIELD_NAME),
                json: json.clone(),
            };
            return Err(pe);
        }
    };

    match integrator_name.as_str() {
        AMBIENT_OCCLUSION_KIND => Ok(Box::new(AmbientOcclusionIntegrator {})),
        other => {
            let pe = ParseError {
                msg: format!("invalid integrator kind '{}'", other),
                json: json.clone(),
            };
            return Err(pe);
        },
    }
}

fn get_num_samples(json: &serde_json::Value) -> Result<u32, ParseError> {
    // Default value if none provided.
    if let None = json.get(NUM_SAMPLES_FIELD_NAME) {
        return Ok(DEFAULT_NUM_SAMPLES);
    }

    match serde_json::from_value::<u32>(json[NUM_SAMPLES_FIELD_NAME].clone()) {
        Ok(n) => {
            return Ok(n);
        },
        Err(_) => {
            let pe = ParseError {
                msg: format!("could not parse field '{}' in 'integrator'", NUM_SAMPLES_FIELD_NAME),
                json: json.clone()
            };
            return Err(pe);
        }
    };
}

fn get_recursion_limit(json: &serde_json::Value) -> Result<u32, ParseError> {
    // Default value if none provided.
    if let None = json.get(RECURSION_LIMIT_FIELD_NAME) {
        return Ok(DEFAULT_RECURSION_LIMIT);
    }

    match serde_json::from_value::<u32>(json[RECURSION_LIMIT_FIELD_NAME].clone()) {
        Ok(n) => {
            return Ok(n)
        },
        Err(_) => {
            let pe = ParseError {
                msg: format!("could not parse field '{}'", RECURSION_LIMIT_FIELD_NAME), 
                json: json.clone()
            };
            return Err(pe);
        }
    };
}

