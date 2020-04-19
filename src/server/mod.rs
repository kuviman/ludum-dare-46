use crate::*;

#[derive(Default)]
struct ModelWrapper {
    model: Model,
    events: Events<ServerMessage>,
}

impl Deref for ModelWrapper {
    type Target = Model;
    fn deref(&self) -> &Model {
        &self.model
    }
}

impl DerefMut for ModelWrapper {
    fn deref_mut(&mut self) -> &mut Model {
        &mut self.model
    }
}

impl ModelWrapper {
    fn handle(&mut self, player_id: Id, message: ClientMessage) {
        self.model.handle(player_id, message, &mut self.events);
    }
    fn tick(&mut self) {
        self.model.tick(&mut self.events);
    }
}

struct Client {
    player_id: Option<Id>,
    model: Arc<Mutex<ModelWrapper>>,
    message_handler: Arc<Box<dyn Fn(ServerMessage) + Sync + Send>>,
    sender: Arc<Mutex<Box<dyn geng::net::Sender<ServerMessage>>>>,
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Some(player_id) = self.player_id {
            self.model.lock().unwrap().disconnect(player_id);
        }
    }
}

impl geng::net::Receiver<ClientMessage> for Client {
    fn handle(&mut self, message: ClientMessage) {
        match message {
            ClientMessage::GetToken => {
                self.sender
                    .lock()
                    .unwrap()
                    .send(ServerMessage::Token(Token::new()));
                return;
            }
            ClientMessage::Connect(token) => {
                let mut model = self.model.lock().unwrap();
                self.player_id = Some(model.connect(&token));
                model
                    .events
                    .subscribe(Arc::downgrade(&self.message_handler));
                return;
            }
            _ => {}
        }
        if let Some(player_id) = self.player_id {
            self.model.lock().unwrap().handle(player_id, message);
        }
    }
}
struct ServerApp {
    model: Arc<Mutex<ModelWrapper>>,
}
impl geng::net::server::App for ServerApp {
    type Client = Client;
    type ServerMessage = ServerMessage;
    type ClientMessage = ClientMessage;
    fn connect(&mut self, mut sender: Box<dyn geng::net::Sender<ServerMessage>>) -> Client {
        let sender = Arc::new(Mutex::new(sender));
        Client {
            player_id: None,
            model: self.model.clone(),
            sender: sender.clone(),
            message_handler: Arc::new(Box::new({
                let sender = sender.clone();
                move |message| sender.lock().unwrap().send(message)
            })),
        }
    }
}

pub struct Server {
    model: Arc<Mutex<ModelWrapper>>,
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
