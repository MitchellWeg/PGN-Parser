use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct PGN {
    pub date: String,
    pub white: String,
    pub black: String,
    pub game_result: String,
    pub white_elo: String,
    pub black_elo: String,
    pub time_control: String,
    pub termination: String,
    pub moves: String,
}

impl Default for PGN {
    fn default() -> PGN {
        PGN {
            date: "".to_string(),
            white: "".to_string(),
            black: "".to_string(),
            game_result: "".to_string(),
            white_elo: "".to_string(),
            black_elo: "".to_string(),
            time_control: "".to_string(),
            termination: "".to_string(),
            moves: "".to_string(),
        }
    }
}
