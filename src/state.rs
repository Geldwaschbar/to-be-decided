use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub enum GameState {
    Starting,
    Running,
    Won,
    Lost,
    Startup,
}
