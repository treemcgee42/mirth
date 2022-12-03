use crate::{scene::Scene, camera::Camera, utility::image::Image};


pub struct RayTracer {
    camera: Camera,
    scene: Scene,
}

impl RayTracer {
    pub fn trace(&self) {
        let mut image = Image::new(600,600);
        let mut num_samples_rendered: usize = 0;


    }
}

