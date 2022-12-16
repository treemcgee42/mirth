//! This module encapsulates functionality related to "objects" in the scene. An 
//! object is just a physical item in the scene. Intuitively, this means that it 
//! has geometry (is a `Shape`) and some sort of material (`Material`). Perhaps 
//! less intuitively to non-graphics people, it may also have a texture (`Texture`).

pub mod traits;
pub mod object;
pub mod object_group;
pub mod shapes;
pub mod textures;
pub mod materials;

