use std::{collections::HashMap, rc::Rc};
use tracing::error;
use crate::utility::math::vector::Color3;

use super::{traits::TextureLike, constant::ConstantTexture};

/// The basic setup is an array as follows:
///
/// ```
/// {
///     "textures": [
///         {
///             "name": Name1
///             "kind": Kind1
///         },
///         ...
///     ] 
/// }
/// ```
/// Each kind of texture will have its own variation of fields:
///
/// ## Constant texture 
/// ```
/// {
///     "name": Name1,
///     "kind": "constant",
///     "rgb_color": [r,g,b]
/// }
/// ```
///
pub fn parse_json(json: &serde_json::Value) -> HashMap<String, Rc<dyn TextureLike>> {
    let json_array: &Vec<serde_json::Value> = match json {
        serde_json::Value::Array(arr) => arr,
        _ => {
            error!("textures in json file are not listed as an array");
            panic!();
        }
    };   

    let mut to_return: HashMap<String, Rc<dyn TextureLike>> = HashMap::new();
    for texture in json_array.iter() {
        let result = parse_single_texture(&texture);
        to_return.insert(result.0, result.1);
    }

    to_return
}

fn parse_single_texture(json: &serde_json::Value) -> (String, Rc<dyn TextureLike>) {
    let name = match &json["name"] {
        serde_json::Value::String(s) => s,
        serde_json::Value::Null => {
            error!(json = ?json, "no 'name' field");
            panic!();
        },
        _ => {
            error!(json = ?json, "'name' must be a string");
            panic!();
        }
    };

    let kind = match &json["kind"] {
        serde_json::Value::String(s) => s,
        serde_json::Value::Null => {
            error!(json = ?json, "no 'kind' field");
            panic!();
        },
        _ => {
            error!(json = ?json, "'kind' must be a string");
            panic!();
        }
    };

    match kind.as_str() {
        "constant" => { return (name.to_owned(), parse_constant_texture(json)) },
        other => {
            error!(json = ?json, "unknown texture kind '{}'", other);
            panic!();
        }
    }
}

fn parse_constant_texture(json: &serde_json::Value) -> Rc<ConstantTexture> {
    let rgb_color = match serde_json::from_value::<Color3>(json["rgb_color"].clone()) {
        Ok(c) => c,
        Err(e) => {
            error!(json = ?json, serde = ?e, "couldn't parse this constant texture");
            panic!();
        }
    };

    let texture = ConstantTexture::new_from_rgb(rgb_color);
    Rc::new(texture)
}

// S==== TESTS {{{1

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;
    use super::*;

    #[test]
    #[traced_test]
    fn test_parsing_constant_texture() {
        let json = r#"
            {
                "textures": [
                    {
                        "name": "t1",
                        "kind": "constant",
                        "rgb_color": [1,0,0]
                    },
                    {
                        "name": "t2",
                        "kind": "constant",
                        "rgb_color": [0,1,0]
                    }
                ]
            }
        "#;

        let initial_parsed: serde_json::Value = serde_json::from_str(json).unwrap();
        let map = parse_json(&initial_parsed["textures"]);
        println!("{:?}", map);
    }
}

// E==== TESTS }}}1

