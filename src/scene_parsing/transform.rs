use serde::Deserialize;

use crate::{utility::math::{vector::Vec3, float::Float, matrix::{Matrix4AxisRotationInfo, Matrix4TransformKind, Matrix4}, angle::{Angle, AngleUnits}}, objects::shapes::transform::Transform};

use super::parse_error::ParseError;


/// Only used to parse from json using `serde`
#[derive(Deserialize)]
struct PreViewerTransform {
    look_from: Vec3,
    look_at: Vec3,
    up_direction: Vec3,
}

#[derive(Deserialize)]
struct SimpleRotation {
    axis: Vec3,
    angle: Float,
}

impl From<SimpleRotation> for Matrix4AxisRotationInfo {
    fn from(sr: SimpleRotation) -> Self {
        let axis = sr.axis;
        let angle = Angle {
            amount: sr.angle,
            units: AngleUnits::Degrees,
        };

        Matrix4AxisRotationInfo {
            axis,
            angle,
        }
    }
}

pub fn new_from_json(json: &serde_json::Value) -> Result<Transform, ParseError> {
    // default value
    if let serde_json::Value::Null = json {
        return Ok(Transform::default());
    }

    // Get json as a map
    let map: serde_json::Map<String, serde_json::Value>;
    if let serde_json::Value::Object(obj) = json {
        map = obj.clone();
    } else { 
        let parse_error = ParseError {
            msg: "json passed was not a map".to_string(),
            json: json.clone(),
        };
        return Err(parse_error); 
    }

    let mut keys = map.keys();
    if keys.len() != 1 { 
        let parse_error = ParseError {
            msg: "json passed does not have exactly one key".to_string(),
            json: json.clone()
        };
        return Err(parse_error);
    }

    match keys.next().unwrap().as_str() {
        "viewer" => { 
            return new_for_viewer_from_json(&json["viewer"]); 
        }
        "simple sequence" => { 
            return new_from_simple_sequence_json(&json["simple sequence"]); 
        }
        other => { 
            let parse_error = ParseError {
                msg: format!("json passed has unknown key {}", other),
                json: json.clone(),
            };
            return Err(parse_error); 
        }
    };
}

/// Tries to parse the json assuming it is a "viewer" type, i.e. something that could 
/// be construction using `new_for_viewer`. 
///
/// An error result means that either the json was not a viewer type, or was a viewer 
/// type but was written incorrectly. TODO: we should handle the latter case better.
fn new_for_viewer_from_json(json: &serde_json::Value) -> Result<Transform, ParseError> {
    let to_return: Result<PreViewerTransform, _> = serde_json::from_value(json.clone());

    if let Ok(pvt) = to_return {
        Ok(Transform::new_for_viewer(
            &pvt.look_from, 
            &pvt.look_at,
            &pvt.up_direction
        ))
    } else {
        let parse_error = ParseError {
            msg: "could not parse viewer transform".to_string(),
            json: json.clone(),
        };
        Err(parse_error)
    }
}

fn new_from_simple_sequence_json(json: &serde_json::Value) -> Result<Transform, ParseError> {
    // Convert json to map
    let map: serde_json::Map<String, serde_json::Value>;
    if let serde_json::Value::Object(obj) = json {
        map = obj.clone();
    } else {
        let parse_error = ParseError {
            msg: "json passed to `new_from_simple_sequence_json()` is not a map".to_string(),
            json: json.clone()
        };
        return Err(parse_error);
    }

    let mut sequence: Vec<Matrix4TransformKind> = Vec::new();

    // Handle each simple transform in sequence
    for key in map.keys() {
        match key.as_str() {
            "rotation" => {
                let parsed: Result<SimpleRotation, _> 
                    = serde_json::from_value(json["rotation"].clone());
                
                if parsed.is_err() { 
                    let parse_error = ParseError {
                        msg: "could not parse rotation".to_string(),
                        json: json.clone(),
                    };
                    return Err(parse_error); 
                }
                let simple_rotation = parsed.unwrap();

                sequence.push(Matrix4TransformKind::AxisRotation(simple_rotation.into()));
            }
            "translation" => {
                let parsed: Result<Vec3, _> = serde_json::from_value(json["translation"].clone());

                if parsed.is_err() { 
                    let parse_error = ParseError {
                        msg: "could not parse translation".to_string(),
                        json: json.clone(),
                    };
                    return Err(parse_error); 
                }
                let simple_translation = parsed.unwrap();

                sequence.push(Matrix4TransformKind::Translation(simple_translation));
            }
            "scale" => {
                let parsed: Result<Vec3, _> = serde_json::from_value(json["scale"].clone());

                if parsed.is_err() { 
                    let parse_error = ParseError {
                        msg: "could not parse scale".to_string(),
                        json: json.clone(),
                    };
                    return Err(parse_error); 
                }
                let simple_scale = parsed.unwrap();

                sequence.push(Matrix4TransformKind::Translation(simple_scale));
            }
            other => {
                let parse_error = ParseError {
                    msg: format!("unknown simple transform type {}", other),
                    json: json.clone(),
                };
                return Err(parse_error);
            }
        }
    }

    let matrix = Matrix4::new_from_sequence(&sequence);
    Ok(Transform::new_from_matrix(&matrix))
}

