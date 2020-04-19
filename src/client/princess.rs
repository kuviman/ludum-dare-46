use crate::*;

#[derive(geng::Assets)]
pub struct Assets {
    #[asset(path = "body.png")]
    body: ugli::Texture,
    #[asset(path = "crown_lf.png")]
    crown_lf: ugli::Texture,
    #[asset(path = "crown_rg.png")]
    crown_rg: ugli::Texture,
    #[asset(path = "eye_lf.png")]
    eye_lf: ugli::Texture,
    #[asset(path = "eye_rg.png")]
    eye_rg: ugli::Texture,
    #[asset(path = "mouth_happy.png")]
    mouth_happy: ugli::Texture,
    #[asset(path = "pupil_lf.png")]
    pupil_lf: ugli::Texture,
    #[asset(path = "pupil_rg.png")]
    pupil_rg: ugli::Texture,
    #[asset(path = "under_eye_lf.png")]
    under_eye_lf: ugli::Texture,
    #[asset(path = "under_eye_rg.png")]
    under_eye_rg: ugli::Texture,
}

pub struct Princess {
    geng: Rc<Geng>,
    assets: Assets,
    eat_time: f32,
}

impl Princess {
    pub fn new(geng: &Rc<Geng>, assets: Assets) -> Self {
        Self {
            geng: geng.clone(),
            assets,
            eat_time: 1.0,
        }
    }
    pub fn eat(&mut self) {
        self.eat_time = 0.0;
    }
    pub fn update(&mut self, delta_time: f32) {
        self.eat_time = (self.eat_time + delta_time * 3.0).min(1.0);
    }
    pub fn draw(&self, drawer: &Drawer, framebuffer: &mut ugli::Framebuffer, camera: &Camera) {
        let size: f32 = 2.0;
        let t = ((self.eat_time * 2.0 * std::f32::consts::PI).cos() + 1.0) / 2.0;
        let angle = t * 1.5;
        drawer.draw(
            framebuffer,
            camera,
            &self.assets.crown_lf,
            vec2(-0.3, 0.4) * size,
            size,
            angle,
            Color::WHITE,
        );
        drawer.draw(
            framebuffer,
            camera,
            &self.assets.crown_rg,
            vec2(0.3, 0.4) * size,
            size,
            -angle,
            Color::WHITE,
        );
        drawer.draw(
            framebuffer,
            camera,
            &self.assets.body,
            vec2(0.0, 0.0),
            size,
            0.0,
            Color::WHITE,
        );
        drawer.draw(
            framebuffer,
            camera,
            &self.assets.eye_lf,
            vec2(-0.3, 0.4) * size,
            size / 5.0,
            0.0,
            Color::WHITE,
        );
        drawer.draw(
            framebuffer,
            camera,
            &self.assets.eye_rg,
            vec2(0.3, 0.4) * size,
            size / 5.0,
            0.0,
            Color::WHITE,
        );
        drawer.draw(
            framebuffer,
            camera,
            &self.assets.pupil_lf,
            vec2(-0.3, 0.4) * size,
            size / 10.0,
            0.0,
            Color::WHITE,
        );
        drawer.draw(
            framebuffer,
            camera,
            &self.assets.pupil_rg,
            vec2(0.3, 0.4) * size,
            size / 10.0,
            0.0,
            Color::WHITE,
        );
        drawer.draw(
            framebuffer,
            camera,
            &self.assets.under_eye_lf,
            vec2(-0.3, 0.2) * size,
            size / 10.0,
            0.0,
            Color::WHITE,
        );
        drawer.draw(
            framebuffer,
            camera,
            &self.assets.under_eye_rg,
            vec2(0.3, 0.2) * size,
            size / 10.0,
            0.0,
            Color::WHITE,
        );
        drawer.draw(
            framebuffer,
            camera,
            &self.assets.mouth_happy,
            vec2(0.0, -0.2) * size,
            size / 3.0,
            0.0,
            Color::WHITE,
        );
    }
}
