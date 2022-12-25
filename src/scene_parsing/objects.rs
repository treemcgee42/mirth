use std::rc::Rc;

use crate::objects::{object::{Object, ObjectInfo}, object_group::ObjectGroup};

use super::{shape, parse_error::ParseError, textures::TextureMap, materials::MaterialMap};

pub struct ObjectParseInfo<'a> {
    pub json: &'a serde_json::Value,
    pub textures: &'a TextureMap, 
    pub materials: &'a MaterialMap 
}

pub fn parse_json(info: ObjectParseInfo) -> Result<ObjectGroup, ParseError> {
    let mut objects_vector: Vec<Rc<Object>> = Vec::new();

    let json_array = match info.json {
        serde_json::Value::Array(arr) => arr,
        _ => {
            let pe = ParseError {
                msg: "objects not listed as json array".to_string(),
                json: info.json.clone(),
            };
            return Err(pe);
        }
    };

    for object in json_array.iter() {
        let object_info = ObjectParseInfo {
            json: &object,
            textures: info.textures,
            materials: info.materials
        };
        objects_vector.push(Rc::new(new_object_from_json(object_info)?));
    }

    Ok(ObjectGroup::new_from_vector(objects_vector))
}

fn new_object_from_json(info: ObjectParseInfo) -> Result<Object, ParseError> {
    let shape = shape::new_from_json(&info.json["shape"])?;

    let texture = match info.json["texture"].as_str() {
        Some(texture_name) => info.textures.get(texture_name)?,
        None => { 
            let pe = ParseError {
                msg: "no texture specified".to_string(),
                json: info.json.clone(),
            };
            return Err(pe); 
        }
    };

    let material = match info.json["material"].as_str() {
        Some(material_name) => info.materials.get(material_name)?,
        None => {
            let pe = ParseError {
                msg: "no material specified".to_string(),
                json: info.json.clone(),
            };
            return Err(pe);
        }
    };

    let object_info = ObjectInfo {
        shape,
        texture,
        material
    };
    Ok(Object::new(object_info))
}
