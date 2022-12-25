
// S==== IMPORTS {{{1

use std::{rc::Rc, collections::HashMap};
use crate::objects::materials::{lambertian::Lambertian, traits::MaterialLike};
use super::parse_error::ParseError;

// E==== IMPORTS }}}1

const NAME_FIELD_NAME: &str = "name";
const KIND_FIELD_NAME: &str = "kind";
const LAMBERTIAN_KIND: &str = "lambertian";

pub struct MaterialMap {
    map: HashMap<String, Rc<dyn MaterialLike>>
}

impl MaterialMap {
    pub fn get(&self, key: &str) -> Result<Rc<dyn MaterialLike>, ParseError> {
        match self.map.get(key) {
            Some(val) => Ok(val.clone()),
            None => {
                let pe = ParseError {
                    msg: format!("no material named '{}'", key),
                    json: serde_json::Value::Null,
                };
                Err(pe)
            }
        }
    }
}

pub fn parse_json(json: &serde_json::Value) -> Result<MaterialMap, ParseError> {
    let json_array: &Vec<serde_json::Value> = match json {
        serde_json::Value::Array(arr) => arr,
        _ => {
            let pe = ParseError {
                msg: "textures in json file are not listed as an array".to_string(),
                json: json.clone()
            };
            return Err(pe);
        }
    };   

    let mut to_return: HashMap<String, Rc<dyn MaterialLike>> = HashMap::new();
    for material in json_array.iter() {
        let result = parse_single_material(&material)?;
        to_return.insert(result.0, result.1);
    }

    Ok(MaterialMap {
        map: to_return
    })
}

fn parse_single_material(json: &serde_json::Value) -> Result<(String, Rc<dyn MaterialLike>), ParseError> {
    let name = get_name(json)?;

    let kind_name = get_kind_name(json)?; 
    match kind_name.as_str() {
        LAMBERTIAN_KIND => {
            let material = Lambertian {}; 
            return Ok((name, Rc::new(material)));
        },
        other => {
            let pe = ParseError {
                msg: format!("unknown material kind {}", other),
                json: json.clone(),
            };
            return Err(pe);
        }
    }
}

fn get_name(json: &serde_json::Value) -> Result<String, ParseError> {
    match &json[NAME_FIELD_NAME] {
        serde_json::Value::String(s) => { return Ok(s.to_string()); },
        serde_json::Value::Null => {
            let pe = ParseError {
                msg: format!("could not find the required field '{}'", NAME_FIELD_NAME),
                json: json.clone()
            };
            return Err(pe);
        },
        _ => {
            let pe = ParseError {
                msg: format!("value of field '{}' must be a string", NAME_FIELD_NAME),
                json: json.clone(),
            };
            return Err(pe);
        }
    };
}

fn get_kind_name(json: &serde_json::Value) -> Result<String, ParseError> {
    match &json[KIND_FIELD_NAME] {
        serde_json::Value::String(s) => { return Ok(s.to_string()); },
        serde_json::Value::Null => {
            let pe = ParseError {
                msg: format!("could not find required field '{}'", KIND_FIELD_NAME),
                json: json.clone(),
            };
            return Err(pe);
        },
        _ => {
            let pe = ParseError {
                msg: format!("value of field '{}' must be a string", KIND_FIELD_NAME), 
                json: json.clone(),
            };
            return Err(pe);
        }
    };
}

// S==== TESTS {{{1

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn parse_lambertian() {
    //     let json_str = r#"
    //     {
    //         "materials": {
    //             "mat1": {
    //                 "kind": "lambertian"
    //             }
    //         }
    //     }
    //     "#;
    //
    //     let parsed_value: serde_json::Value = serde_json::from_str(json_str).unwrap();
    //     let parsed = parse_json(&parsed_value["materials"]).unwrap();
    // }
}

// E==== TESTS }}}1

