
// S==== IMPORTS {{{1

use std::{collections::HashMap, rc::Rc, fmt::format};
use tracing::error;
use crate::{utility::math::vector::Color3, objects::textures::{traits::TextureLike, constant::ConstantTexture}};

use super::parse_error::ParseError;

// E==== IMPORTS }}}1

const NAME_FIELD_NAME: &str = "name";

const KIND_FIELD_NAME: &str = "kind";
const CONSTANT_KIND: &str = "constant";

const RGB_FIELD_NAME: &str = "rgb color";

pub struct TextureMap {
    map: HashMap<String, Rc<dyn TextureLike>>
}

impl TextureMap {
    pub fn get(&self, key: &str) -> Result<Rc<dyn TextureLike>, ParseError> {
        match self.map.get(key) {
            Some(val) => Ok(val.clone()),
            None => {
                let pe = ParseError {
                    msg: format!("no texture named '{}'", key),
                    json: serde_json::Value::Null,
                };
                Err(pe)
            }
        }
    }
}

pub fn parse_json(json: &serde_json::Value) -> Result<TextureMap, ParseError> {
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

    let mut to_return: HashMap<String, Rc<dyn TextureLike>> = HashMap::new();
    for texture in json_array.iter() {
        let result = parse_single_texture(&texture)?;
        to_return.insert(result.0, result.1);
    }

    Ok(TextureMap {
        map: to_return
    })
}

fn parse_single_texture(json: &serde_json::Value) -> Result<(String, Rc<dyn TextureLike>), ParseError> {
    let name = get_name(json)?;

    let kind_name = get_kind_name(json)?; 
    match kind_name.as_str() {
        CONSTANT_KIND => { return Ok((name.to_owned(), parse_constant_texture(json)?)) },
        other => {
            let pe = ParseError {
                msg: format!("unknown texture kind '{}'", other), 
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
                msg: format!("could not find required field '{}' in texture", NAME_FIELD_NAME), 
                json: json.clone()
            };
            return Err(pe);
        },
        _ => {
            let pe = ParseError {
                msg: format!("value of field '{}' in texture must be a string", NAME_FIELD_NAME), 
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
                msg: format!("could not find required field '{}' in texture", KIND_FIELD_NAME), 
                json: json.clone(),
            };
            return Err(pe);
        },
        _ => {
            let pe = ParseError {
                msg: format!("value of field '{}' in texture must be a string", KIND_FIELD_NAME), 
                json: json.clone(),
            };
            return Err(pe);
        }
    };
}

fn parse_constant_texture(json: &serde_json::Value) -> Result<Rc<ConstantTexture>, ParseError> {
    let rgb_color = match serde_json::from_value::<Color3>(json[RGB_FIELD_NAME].clone()) {
        Ok(c) => c,
        Err(_) => {
            let pe = ParseError {
                msg: "couldn't parse this constant texture's RGB color".to_string(),
                json: json.clone(),
            };
            return Err(pe);
        }
    };

    let texture = ConstantTexture::new_from_rgb(rgb_color);
    Ok(Rc::new(texture))
}

// S==== TESTS {{{1

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;
    use super::*;

    // #[test]
    // #[traced_test]
    // fn test_parsing_constant_texture() {
    //     let json = r#"
    //         {
    //             "textures": [
    //                 {
    //                     "name": "t1",
    //                     "kind": "constant",
    //                     "rgb_color": [1,0,0]
    //                 },
    //                 {
    //                     "name": "t2",
    //                     "kind": "constant",
    //                     "rgb_color": [0,1,0]
    //                 }
    //             ]
    //         }
    //     "#;
    //
    //     let initial_parsed: serde_json::Value = serde_json::from_str(json).unwrap();
    //     let map = parse_json(&initial_parsed["textures"]);
    //     println!("{:?}", map);
    // }
}

// E==== TESTS }}}1

