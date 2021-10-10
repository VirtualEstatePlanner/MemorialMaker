pub struct ISong {
  audio: IAudio,
  title: &str,
  artist: &str,
}

pub fn new() -> ISong {
  ISong {
    audio: IAudio::new(),
    title: "song-title",
    artist: "song-artist",
  }
}