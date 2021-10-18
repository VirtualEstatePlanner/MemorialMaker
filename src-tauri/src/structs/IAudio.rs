use url::Url;

pub struct IAudio {
  audio_data: Url,
  duration: u128,
}

pub fn new() -> IAudio {
  return IAudio {
    audio_data: Url::parse("https://default-url-location.com").unwrap(),
    duration: 0,
  };
}
