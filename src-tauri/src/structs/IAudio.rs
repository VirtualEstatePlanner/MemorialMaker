pub struct IAudio {
  audio_data: url::URL,
  duration: u128,
}

pub fn new() -> IAudio {
  IAudio {
    audio_data: url::URL::parse("default-url-location.com").unwrap(),
    duration: 0,
  }
}
