use crate::*;

struct Client {
    player_token: Option<Token>,
    model: Arc<Mutex<Model>>,
    sender: Box<dyn geng::net::Sender<ServerMessage>>,
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Some(player_token) = &self.player_token {
            self.model.lock().unwrap().disconnect(player_token);
        }
    }
}

impl geng::net::Receiver<ClientMessage> for Client {
    fn handle(&mut self, message: ClientMessage) {
        match message {
            ClientMessage::GetToken => {
                self.sender.send(ServerMessage::Token(Token::new()));
                return;
            }
            ClientMessage::Connect(token) => {
                self.model.lock().unwrap().connect(&token);
                self.player_token = Some(token);
                return;
            }
            _ => {}
        }
        if let Some(player_token) = &self.player_token {
            for reply in self.model.lock().unwrap().handle(player_token, message) {
                self.sender.send(reply);
            }
        }
    }
}
struct ServerApp {
    model: Arc<Mutex<Model>>,
}
impl geng::net::server::App for ServerApp {
    type Client = Client;
    type ServerMessage = ServerMessage;
    type ClientMessage = ClientMessage;
    fn connect(&mut self, mut sender: Box<dyn geng::net::Sender<ServerMessage>>) -> Client {
        Client {
            player_token: None,
            model: self.model.clone(),
            sender,
        }
    }
}

pub struct Server {
    model: Arc<Mutex<Model>>,
    server: geng::net::Server<ServerApp>,
}

impl Server {
    pub const TICKS_PER_SECOND: f64 = Model::TICKS_PER_SECOND;
    pub fn new(net_opts: &NetOpts) -> Self {
        let model = Arc::new(Mutex::new(default()));
        Self {
            model: model.clone(),
            server: geng::net::Server::new(
                ServerApp {
                    model: model.clone(),
                },
                (net_opts.server_host.as_str(), net_opts.server_port),
            ),
        }
    }
    pub fn handle(&self) -> geng::net::ServerHandle {
        self.server.handle()
    }
    pub fn run(self) {
        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let server_thread = std::thread::spawn({
            let model = self.model;
            let running = running.clone();
            move || {
                while running.load(std::sync::atomic::Ordering::Relaxed) {
                    // TODO: smoother TPS
                    std::thread::sleep(std::time::Duration::from_millis(
                        (1000.0 / Self::TICKS_PER_SECOND) as u64,
                    ));
                    let mut model = model.lock().unwrap();
                    model.tick();
                }
            }
        });
        self.server.run();
        running.store(false, std::sync::atomic::Ordering::Relaxed);
        server_thread.join().expect("Failed to join server thread");
    }
}
