use crate::*;

#[derive(geng::Assets)]
pub struct Assets {}

pub struct State {
    princess_life: f64,
    princess_alive: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            princess_life: 0.0,
            princess_alive: false,
        }
    }
}

pub struct ClientApp {
    geng: Rc<Geng>,
    assets: Assets,
    connection: geng::net::client::Connection<ServerMessage, ClientMessage>,
    state: State,
}

impl ClientApp {
    pub fn run(opts: &Opts) {
        let geng = Rc::new(Geng::new(geng::ContextOptions {
            title: "LudumDare 46 - The Princess".to_owned(),
            ..default()
        }));
        let name = opts.name.clone();
        let opts = opts.clone();
        let connection_future = async move {
            let mut connection = geng::net::client::connect(&opts.net_opts.connect).await;
            connection.send(ClientMessage::GetToken);
            let (message, mut connection) = connection.into_future().await;
            info!("Got token");
            let token = if let Some(ServerMessage::Token(token)) = message {
                token
            } else {
                panic!("Expected token, got {:?}", message);
            };
            connection.send(ClientMessage::Connect(token.clone()));
            (connection, token)
        };
        let assets_future = <Assets as geng::LoadAsset>::load(&geng, ".");
        let app = geng::LoadingScreen::new(
            &geng,
            geng::EmptyLoadingScreen,
            future::join(assets_future, connection_future),
            {
                let geng = geng.clone();
                move |(assets, (mut connection, token))| {
                    Self::new(&geng, connection, assets.unwrap())
                }
            },
        );
        geng::run(geng, app);
    }

    pub fn new(
        geng: &Rc<Geng>,
        connection: geng::net::client::Connection<ServerMessage, ClientMessage>,
        assets: Assets,
    ) -> Self {
        Self {
            geng: geng.clone(),
            assets,
            connection,
            state: default(),
        }
    }
}

impl geng::State for ClientApp {
    fn update(&mut self, delta_time: f64) {
        for message in self.connection.new_messages() {
            match message {
                ServerMessage::PrincessDied => {
                    info!("Princess died!");
                    self.state.princess_alive = false;
                }
                ServerMessage::PrincessLife(life) => {
                    self.state.princess_alive = true;
                    self.state.princess_life = life;
                }
                _ => {}
            }
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::WHITE), None);
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        if self.state.princess_alive {
            self.geng.default_font().draw_aligned(
                framebuffer,
                &format!("Princess life: {}", self.state.princess_life.max(0.0)),
                vec2(framebuffer_size.x / 2.0, framebuffer_size.y / 2.0),
                0.5,
                32.0,
                Color::BLACK,
            );
        } else {
            self.geng.default_font().draw_aligned(
                framebuffer,
                "Princess is dead :(",
                vec2(framebuffer_size.x / 2.0, framebuffer_size.y / 2.0),
                0.5,
                32.0,
                Color::BLACK,
            );
        }
    }
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown {
                key: geng::Key::Space,
                ..
            } => {
                self.connection.send(ClientMessage::Feed);
            }
            _ => {}
        }
    }
}
