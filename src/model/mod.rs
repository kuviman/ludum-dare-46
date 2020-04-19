use crate::*;

mod events;

pub use events::*;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Token(String);

impl Token {
    pub fn new() -> Self {
        let mut rng = global_rng();
        Self(
            std::iter::repeat(())
                .map(|()| rng.sample(rand::distributions::Alphanumeric))
                .take(16)
                .collect(),
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Id(usize);

impl Id {
    pub fn new() -> Self {
        static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
        Id(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    Token(Token),
    Feed(Id),
    PrincessDied,
    PrincessLife(f64),
    PlayerInfo(Player),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    GetToken,
    Connect(Token),
    SetName(String),
    Feed,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
    pub token: Token,
    pub id: Id,
    pub name: String,
}

impl Player {
    fn new(token: Token) -> Self {
        Self {
            token,
            id: Id::new(),
            name: "<noname>".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct State {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    eaten: Vec<Id>,
    players: HashMap<Id, Player>,
    princess_life: f64,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            eaten: Vec::new(),
            players: HashMap::new(),
            princess_life: 60.0,
        }
    }
}

impl Model {
    pub const TICKS_PER_SECOND: f64 = 1.0;

    pub fn initial_messages(&self) -> Vec<ServerMessage> {
        let mut messages = Vec::new();
        for player in self.players.values() {
            messages.push(ServerMessage::PlayerInfo(player.clone()));
        }
        for &id in &self.eaten {
            messages.push(ServerMessage::Feed(id));
        }
        if self.princess_life > 0.0 {
            messages.push(ServerMessage::PrincessLife(self.princess_life));
        } else {
            messages.push(ServerMessage::PrincessDied);
        }
        messages
    }

    pub fn handle(
        &mut self,
        player_id: Id,
        message: ClientMessage,
        events: &mut Events<ServerMessage>,
    ) {
        match message {
            ClientMessage::Feed => {
                if self.princess_life > 0.0 {
                    self.princess_life += 10.0;
                    self.eaten.push(player_id);
                    events.fire(ServerMessage::Feed(
                        self.players.get(&player_id).unwrap().id,
                    ));
                    events.fire(ServerMessage::PrincessLife(self.princess_life));
                }
            }
            ClientMessage::SetName(name) => {
                let player = self.players.get_mut(&player_id).unwrap();
                player.name = name;
                events.fire(ServerMessage::PlayerInfo(player.clone()));
            }
            _ => {}
        }
    }

    pub fn connect(&mut self, player_token: &Token, events: &mut Events<ServerMessage>) -> Id {
        let player = Player::new(player_token.clone());
        info!("{:?} connected", player);
        let id = player.id;
        events.fire(ServerMessage::PlayerInfo(player.clone()));
        self.players.insert(id, player);
        id
    }

    pub fn disconnect(&mut self, player_id: Id) {
        if let Some(player) = self.players.remove(&player_id) {
            info!("{:?} disconnected", player);
        }
    }

    pub fn tick(&mut self, events: &mut Events<ServerMessage>) {
        let delta_time = 1.0 / Self::TICKS_PER_SECOND;
        if self.princess_life > 0.0 {
            self.princess_life -= delta_time;
            if self.princess_life <= 0.0 {
                self.eaten.clear();
                events.fire(ServerMessage::PrincessDied);
            }
        } else {
            self.princess_life -= delta_time;
            if self.princess_life < -5.0 {
                self.princess_life = 60.0;
            }
        }
        if self.princess_life > 0.0 {
            events.fire(ServerMessage::PrincessLife(self.princess_life));
        }
    }
}
