use crate::*;

#[derive(ugli::Vertex)]
struct QuadVertex {
    a_pos: Vec2<f32>,
}

pub struct Drawer {
    geng: Rc<Geng>,
    quad_geometry: ugli::VertexBuffer<QuadVertex>,
    program: ugli::Program,
}

impl Drawer {
    pub fn new(geng: &Rc<Geng>) -> Self {
        Self {
            geng: geng.clone(),
            quad_geometry: ugli::VertexBuffer::new_static(
                geng.ugli(),
                vec![
                    QuadVertex {
                        a_pos: vec2(-1.0, -1.0),
                    },
                    QuadVertex {
                        a_pos: vec2(1.0, -1.0),
                    },
                    QuadVertex {
                        a_pos: vec2(1.0, 1.0),
                    },
                    QuadVertex {
                        a_pos: vec2(-1.0, 1.0),
                    },
                ],
            ),
            program: geng
                .shader_lib()
                .compile(include_str!("program.glsl"))
                .unwrap(),
        }
    }

    pub fn draw(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &Camera,
        texture: &ugli::Texture,
        pos: Vec2<f32>,
        size: f32,
        rotation: f32,
        color: Color<f32>,
    ) {
        let size = vec2(texture.size().x as f32 / texture.size().y as f32, 1.0) * size;
        let uniforms = (
            ugli::uniforms! {
                u_size: size,
                u_rotation: rotation,
                u_pos: pos,
                u_texture: texture,
                u_color: color,
            },
            camera.uniforms(framebuffer),
        );
        ugli::draw(
            framebuffer,
            &self.program,
            ugli::DrawMode::TriangleFan,
            &self.quad_geometry,
            uniforms,
            ugli::DrawParameters {
                blend_mode: Some(default()),
                ..default()
            },
        );
    }
}
