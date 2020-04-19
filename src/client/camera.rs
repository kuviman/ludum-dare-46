use crate::*;

pub struct Camera {
    pos: Vec2<f32>,
}

impl Camera {
    const FOV: f32 = 10.0;
    pub fn new() -> Self {
        Self {
            pos: vec2(0.0, 0.0),
        }
    }
    pub fn view_matrix(&self, framebuffer: &ugli::Framebuffer) -> Mat4<f32> {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        Mat4::scale(vec3(framebuffer_size.y / framebuffer_size.x, 1.0, 1.0))
            * Mat4::scale_uniform(2.0 / Self::FOV)
            * Mat4::translate(-self.pos.extend(0.0))
    }
    pub fn uniforms(&self, framebuffer: &ugli::Framebuffer) -> impl ugli::Uniforms {
        ugli::uniforms! {
            u_view_matrix: self.view_matrix(framebuffer),
        }
    }
}
