use crate::*;

#[derive(geng::Assets)]
pub struct Assets {}

pub struct State {
    theme: Rc<geng::ui::Theme>,
    connection: geng::net::client::Connection<ServerMessage, ClientMessage>,
    princess_life: f64,
    princess_alive: bool,
    button: geng::ui::TextButton,
}

impl State {
    fn new(
        geng: &Rc<Geng>,
        connection: geng::net::client::Connection<ServerMessage, ClientMessage>,
    ) -> Self {
        let theme = Rc::new(geng::ui::Theme {
            color: Color::BLACK,
            ..geng::ui::Theme::default(geng)
        });
        let theme = &theme;
        Self {
            theme: theme.clone(),
            connection,
            princess_life: 0.0,
            princess_alive: false,
            button: geng::ui::TextButton::new(geng, theme, "FEED".to_owned(), 64.0),
        }
    }
    fn ui<'a>(&'a mut self) -> impl geng::ui::Widget + 'a {
        use geng::ui::*;
        let connection = &mut self.connection;
        if self.princess_alive {
            Box::new(geng::ui::column![
                geng::ui::text(
                    format!("Princess life: {}", self.princess_life),
                    &self.theme.font,
                    64.0,
                    self.theme.color,
                )
                .align(vec2(0.5, 0.5))
                .uniform_padding(16.0),
                self.button
                    .ui(Box::new(move || { connection.send(ClientMessage::Feed) }))
                    .align(vec2(0.5, 0.5)),
            ]) as Box<dyn geng::ui::Widget + 'a>
        } else {
            Box::new(
                geng::ui::text(
                    "Princess is dead :(",
                    &self.theme.font,
                    64.0,
                    self.theme.color,
                )
                .align(vec2(0.5, 0.5)),
            )
        }
        .align(vec2(0.5, 0.5))
    }
}

pub struct ClientApp {
    geng: Rc<Geng>,
    assets: Assets,
    state: State,
    ui_controller: geng::ui::Controller,
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
            state: State::new(geng, connection),
            ui_controller: geng::ui::Controller::new(),
        }
    }
}

impl geng::State for ClientApp {
    fn update(&mut self, delta_time: f64) {
        for message in self.state.connection.new_messages() {
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
        self.ui_controller.update(self.state.ui(), delta_time);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::WHITE), None);
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        self.ui_controller.draw(self.state.ui(), framebuffer);
    }
    fn handle_event(&mut self, event: geng::Event) {
        if self
            .ui_controller
            .handle_event(self.state.ui(), event.clone())
        {
            return;
        }
        match event {
            geng::Event::KeyDown {
                key: geng::Key::Space,
                ..
            } => {
                self.state.connection.send(ClientMessage::Feed);
            }
            _ => {}
        }
    }
}
