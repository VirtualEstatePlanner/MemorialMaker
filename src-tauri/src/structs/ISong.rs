use crate::structs::IAudio;

pub struct ISong {
  audio: IAudio::IAudio,
  title: String,
  artist: String,
}

pub fn new() -> ISong {
  return ISong {
    audio: IAudio::new(),
    title: "song-title".to_string(),
    artist: "song-artist".to_string(),
  };
}
