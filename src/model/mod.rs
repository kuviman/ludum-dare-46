use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Token(Token),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    GetToken,
    Connect(Token),
}

#[derive(Serialize, Deserialize, Default)]
pub struct Model {}

impl Model {
    pub const TICKS_PER_SECOND: f64 = 1.0;

    pub fn handle(&mut self, player_token: &Token, message: ClientMessage) -> Vec<ServerMessage> {
        Vec::new()
    }

    pub fn connect(&mut self, player_token: &Token) {
        info!("{:?} connected", player_token);
    }

    pub fn disconnect(&mut self, player_token: &Token) {
        info!("{:?} disconnected", player_token);
    }

    pub fn tick(&mut self) {}
}
