//! Responsible for building the scene from a JSON file. 
//!
//! # the specification
//!
//! The following fields are _required_:
//! ```
//! {
//!     "camera": ...,
//!     "integrator": ...,
//!     "background color": ...,
//!
//! }
//! ```
//!
//! The following fields are _optional_. We list the default values below:
//! ```
//! {
//!     ...,
//!     "background color": []
//! }
//! ```
//!
//! ## integrator
//!
//! The following fields are common to all integrators: 
//! ```
//! {
//!     ...,
//!     "number of samples": Unsigned Integer (default 64),
//!     "ray recursion limit": Unsigned Integer (default 64)
//! }
//! ```
//!
//! ### ambient occlusion
//! 
//! ```
//! {
//!     "kind": "ambient occlusion",
//!     ...
//! }
//! ```
//!
//! ## camera
//!
//! ```
//! "camera": {
//!     "resolution": [Float, Float],
//!     "focal distance": Float,
//!     "vertical fov": Float,
//!     "aperture radius": Float,
//!     "transform": ViewerTransform
//! }
//! ```
//!
//! ## objects
//!
//! ```
//! "objects": [
//!     {
//!         "shape": Shape,
//!         "texture": Name of Texture,
//!         "material": Name of Material
//!     },
//!     ...
//! ]
//! ```
//! 
//! ## materials 
//!
//! ```
//! "materials": [
//!     {
//!         "name": Name,
//!         "kind": Type
//!     },
//!     ...
//! ]
//! ```
//!
//! ### types
//!
//! #### lambertian
//!
//! ## textures
//!
//! The basic setup is an array as follows:
//!
//! ```
//! {
//!     "textures": [
//!         {
//!             "name": Name1,
//!             "kind": Kind1,
//!             ...
//!         },
//!         ...
//!     ]
//! }
//! ```
//! Each kind of texture will have its own variation of fields:
//!
//! ### Constant texture 
//! ```
//! {
//!     "name": Name1,
//!     "kind": "constant",
//!     "rgb_color": [r,g,b]
//! }
//! ```
//! 
//! ## shapes 
//!
//! ### quad
//!
//! ```
//! {
//!     "kind": "quad",
//!     "width": Float,
//!     "height": Float,
//!     "transform": Transform
//! }
//! ```
//!
//! ### sphere
//!
//! ```
//! {
//!     "kind": "sphere",
//!     "center": [Float, Float, Float]
//!     "radius": Float,
//!     "transform": Transform
//! }
//! ```
//!
//! ## transform
//!
//! For parsing from the scene file, the value of the field "transform". There are 
//! several ways to specify a `Transform` in json:
//!
//! ### viewer type
//! 
//! Corresponds to the construction via `new_for_viewer()`, and can be specified 
//! as follows:
//! ```
//! {
//!     "viewer: {
//!         "look_from": Vec3,
//!         "look_at": Vec3,
//!         "up_direction": Vec3
//!     }
//! }
//! ```
//! All three fields must be specified.
//!
//! ### simple sequence type
//! 
//! This is specified as a map of simple types, which are described below. It is 
//! specified like
//! ```
//! {
//!     "simple sequence": [
//!         Simple1,
//!         Simple2,
//!         ...
//!     ]
//! }
//! ```
//! 
//! The following are the simple types:
//!
//! #### rotation
//! ```
//! {
//!     "rotation": {
//!         "axis": Vec3,
//!         "angle": Float
//!     }
//! }
//! ```
//! Here, the angle is specified in degrees.
//! 
//! #### translation
//! ```
//! {
//!     "translation": Vec3
//! }
//! ```
//!
//! #### scale
//! ```
//! {
//!     "scale"
//! }
//! ```

use tracing::instrument;

use crate::{scene::{Scene, SceneInfo}, utility::rng::RandomNumberGenerator};
use self::{parse_error::ParseError, objects::ObjectParseInfo};

mod camera;
mod transform;
mod objects;
mod parse_error;
mod shape;
mod textures;
mod materials;
mod integrator;

pub fn parse_json(json: &serde_json::Value) -> Result<Scene, ParseError> {
    let camera = camera::new_from_json(&json["camera"])?;

    let parsed_integrator = integrator::new_from_json(&json["integrator"])?;

    let objects = {
        let materials = materials::parse_json(&json["materials"])?;
        let textures = textures::parse_json(&json["textures"])?;
        
        let info = ObjectParseInfo {
            json: &json["objects"],
            textures: &textures,
            materials: &materials,
        };
        objects::parse_json(info)?
    };

    let info = SceneInfo {
        camera,
        integrator: parsed_integrator.integrator,
        num_samples: parsed_integrator.num_samples,
        recursive_depth_limit: parsed_integrator.recursion_limit,
        rng: RandomNumberGenerator::from_seed(1),
        objects,
    };
    Ok(Scene::new(info))
}

