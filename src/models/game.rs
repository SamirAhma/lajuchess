use shakmaty::{Chess, Position, Move, fen::Fen};
use serde::{Serialize, Deserialize};
use typeshare::typeshare;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameState {
    pub fen: String,
    pub pgn: String,
    pub creator_color: String,
}

impl GameState {
    pub fn new(creator_color: String) -> Self {
        Self {
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            pgn: String::new(),
            creator_color,
        }
    }

    pub fn make_move(&mut self, uci_move: &str) -> Result<(), String> {
        let mut pos = Fen::from_ascii(self.fen.as_bytes())
            .map_err(|e| e.to_string())?
            .into_position::<Chess>(shakmaty::CastlingMode::Standard)
            .map_err(|e| e.to_string())?;

        let m: Move = uci_move.parse::<shakmaty::uci::UciMove>()
            .map_err(|e| e.to_string())?
            .to_move(&pos)
            .map_err(|e| e.to_string())?;

        if pos.is_legal(&m) {
            pos.play_unchecked(&m);
            self.fen = Fen::from_position(pos, shakmaty::EnPassantMode::Always).to_string();
            Ok(())
        } else {
            Err("Illegal move".to_string())
        }
    }
}

#[derive(Clone)]
pub struct GameSession {
    pub game: Arc<Mutex<GameState>>,
    pub tx: broadcast::Sender<GameState>,
}
