use std::{rc::Rc, collections::HashMap};

use serde::Deserialize;

use super::{traits::MaterialLike, lambertian::Lambertian};

#[derive(Deserialize)]
struct MaterialParser {
    kind: String
}

/// ```
/// "materials": {
///     "Name1": {
///         "kind": Type
///     }
/// }
/// ```
///
/// Return a map with key the name of the material specified in the json, and the 
/// value is an `Rc` of the material.
pub fn parse_json(json: &serde_json::Value) -> Result<HashMap<String, Rc<dyn MaterialLike>>, String> {
    let mut to_return: HashMap<String, Rc<dyn MaterialLike>> = HashMap::new();
    
    /* Check if the json is a map */
    let json_map = match json {
        serde_json::Value::Object(m) => m,
        _ => {
            return Err("json passed to `parse_json` was not a map".to_string());
        }
    };

    for key in json_map.keys().into_iter() {
        println!("{}", key.to_string());
        to_return.insert(key.to_string(), parse_single_material(&json_map.get(key).unwrap())?);
    }

    Ok(to_return)
}

fn parse_single_material(json: &serde_json::Value) -> Result<Rc<dyn MaterialLike>, String> {
    let parsed_json = match serde_json::from_value::<MaterialParser>(json.clone()) {
        Ok(p) => p,
        Err(_) => {
            return Err(format!("couldn't parse field name or value type for material {:?}", json));
        }
    };

    match parsed_json.kind.as_str() {
        "lambertian" => {
            let material = Lambertian {}; 
            return Ok(Rc::new(material));
        },
        other => {
            return Err(format!("unknown material kind {}", other));
        }
    }
}

// S==== TESTS {{{1

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lambertian() {
        let json_str = r#"
        {
            "materials": {
                "mat1": {
                    "kind": "lambertian"
                }
            }
        }
        "#;

        let parsed_value: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let parsed = parse_json(&parsed_value["materials"]).unwrap();
    }
}

// E==== TESTS }}}1

