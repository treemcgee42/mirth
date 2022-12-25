use crate::{camera::{Camera, CameraInfo}, utility::{image::Resolution, math::{float::Float, angle::{Angle, AngleUnits}}}, objects::shapes::transform::Transform};

use super::{parse_error::ParseError, transform};


pub fn new_from_json(json: &serde_json::Value) -> Result<Camera, ParseError> {
    let resolution = match serde_json::from_value::<Resolution>(json["resolution"].clone()) {
        Ok(resolution_) => resolution_,
        _ => {
            let parse_error = ParseError {
                msg: "failed to parse resolution".to_string(),
                json: json.clone(),
            };
            return Err(parse_error);
        }
    };

    let focal_distance = match serde_json::from_value::<Float>(json["focal distance"].clone()) {
        Ok(focal_distance_) => focal_distance_,
        _ => {
            let parse_error = ParseError {
                msg: "failed to parse focal distance".to_string(),
                json: json.clone(),
            };
            return Err(parse_error);
        }
    };

    let vertical_fov = match serde_json::from_value::<Float>(json["vertical fov"].clone()) {
        Ok(vertical_fov_) => Angle {
            units: AngleUnits::Degrees,
            amount: vertical_fov_,
        },
        _ => {
            let parse_error = ParseError {
                msg: "failed to parse vertical fov".to_string(),
                json: json.clone(),
            };
            return Err(parse_error);
        }
    };

    let aperture_radius = match serde_json::from_value::<Float>(json["aperture radius"].clone()) {
        Ok(aperture_radius_) => aperture_radius_,
        _ => {
            let parse_error = ParseError {
                msg: "failed to parse aperture radius".to_string(),
                json: json.clone(),
            };
            return Err(parse_error);
        }
    };

    let transform = transform::new_from_json(&json["transform"])?;

    let info = CameraInfo {
        transform,
        resolution,
        focal_distance,
        aperture_radius,
        vertical_fov,
    };

    Ok(Camera::new(info))
}

