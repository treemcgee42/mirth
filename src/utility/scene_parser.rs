use std::fs;

use serde_json::Value;
use tracing::error;

use crate::scene::Scene;


pub fn parse_scene(scene_file: &str) -> Scene {
    let file = fs::read_to_string(scene_file)
        .expect("failed to open scene file");
    let parsed_json: serde_json::Value = serde_json::from_str(file.as_str())
        .expect("failed to parse json");

    

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_camera() {
        let json = r#"
            {
                "camera": 
            }"#;
    }
}

