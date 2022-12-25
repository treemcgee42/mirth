//! Each shape should have a material. When a ray intersects a surface, the 
//! material determines how that ray scatters. That is precisely what a material 
//! does in Mirth: determines the direction of the scattered ray. 

pub mod traits;
pub mod lambertian;

