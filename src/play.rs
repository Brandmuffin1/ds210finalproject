use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Play {
    pub down: Option<u8>, // Down (e.g., 1, 2, 3, 4)
    pub ydstogo: Option<f32>, // Yards to go for a 1st down
    pub receiver: Option<String>, // Receiver name
}