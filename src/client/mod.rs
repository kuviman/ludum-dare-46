use crate::*;

#[derive(geng::Assets)]
pub struct Assets {}

pub struct ClientApp {
    geng: Rc<Geng>,
    assets: Assets,
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
            let (message, connection) = connection.into_future().await;
            info!("Got token");
            if let Some(ServerMessage::Token(token)) = message {
                (connection, token)
            } else {
                panic!("Expected token, got {:?}", message);
            }
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
        }
    }
}

impl geng::State for ClientApp {
    fn update(&mut self, delta_time: f64) {}
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::WHITE), None);
    }
}
