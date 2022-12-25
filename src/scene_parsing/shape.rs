
// S==== IMPORTS {{{1

use std::rc::Rc;

use crate::{
    objects::shapes::{
        traits::ShapeLike, 
        quad::Quad, 
        sphere::{Sphere, SphereInfo}, 
    }, 
    utility::math::{
        float::Float, 
        vector::Vec3
    }
};

use super::{
    parse_error::ParseError, 
    transform
};

// E==== IMPORTS }}}1

const KIND_FIELD_NAME: &str = "kind";
const QUAD_KIND: &str = "quad";
const SPHERE_KIND: &str = "sphere";

pub fn new_from_json(json: &serde_json::Value) -> Result<Rc<dyn ShapeLike>, ParseError> {
    let kind_name = get_kind_name(json)?;
    match kind_name.as_str() {
        QUAD_KIND => Ok(Rc::new(new_quad_from_json(json)?)),
        SPHERE_KIND => Ok(Rc::new(new_sphere_from_json(json)?)),
        other => { 
            let pe = ParseError {
                msg: format!("invalid shape kind '{}'", other),
                json: json.clone(),
            };
            return Err(pe); 
        }
    }
}

fn get_kind_name(json: &serde_json::Value) -> Result<String, ParseError> {
    match &json[KIND_FIELD_NAME] {
        serde_json::Value::String(s) => { return Ok(s.to_string()); },
        serde_json::Value::Null => {
            let pe = ParseError {
                msg: format!("could not find required field '{}' in shape", KIND_FIELD_NAME), 
                json: json.clone()
            };
            return Err(pe);
        },
        _ => {
            let pe = ParseError {
                msg: format!("value of field '{}' in shape must be a string", KIND_FIELD_NAME), 
                json: json.clone(),
            };
            return Err(pe);
        }
    };
}

// S==== QUAD {{{1

fn new_quad_from_json(json: &serde_json::Value) -> Result<Quad, ParseError> {
    let width = match serde_json::from_value::<Float>(json["width"].clone()) {
        Ok(w) => w,
        Err(_) => { 
            let pe = ParseError {
                msg: "could not parse field `width`".to_string(),
                json: json.clone(),
            };
            return Err(pe); 
        }
    };

    let height = match serde_json::from_value::<Float>(json["height"].clone()) {
        Ok(h) => h,
        Err(_) => {
            let pe = ParseError {
                msg: "could not parse field `height`".to_string(),
                json: json.clone(),
            };
            return Err(pe);
        }
    };

    let transform = match transform::new_from_json(&json["transform"]) {
        Ok(t) => t,
        Err(_) => { 
            let pe = ParseError {
                msg: "could not parse field `width`".to_string(),
                json: json.clone(),
            };
            return Err(pe);
        }
    };

    Ok(Quad {
        width,
        height,
        transform,
    })
}

// E==== QUAD }}}1

// S==== SPHERE {{{1

fn new_sphere_from_json(json: &serde_json::Value) -> Result<Sphere, ParseError> {
    let center = match serde_json::from_value::<Vec3>(json["center"].clone()) {
        Ok(c) => c,
        Err(_) => { 
            let pe = ParseError {
                msg: "could not parse field 'center'".to_string(),
                json: json.clone(),
            };
            return Err(pe);
        }
    };

    let radius = match serde_json::from_value::<Float>(json["radius"].clone()) {
        Ok(r) => r,
        Err(_) => { 
            let pe = ParseError {
                msg: "could not parse field 'radius'".to_string(),
                json: json.clone(),
            };
            return Err(pe); 
        }
    };

    let transform = transform::new_from_json(&json["transform"])?; 

    let sphere_info = SphereInfo {
        center,
        radius,
        transform,
    };
    Ok(Sphere::new(sphere_info))
}

// E==== SPHERE }}}1

