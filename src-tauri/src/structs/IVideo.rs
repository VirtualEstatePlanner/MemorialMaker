use image::ImageBuffer;
use uuid::Uuid;
use url::URL;

pub struct IVideo {
  key: Uuid::new_v5(),
  caption: &str,
  thumbnail: Vec<image::ImageBuffer>,
  // generateThumbnail to an array of frames selected based on the number of frames in the video
  // probably requires video reading libaray such as ffmpeg
  video_data: url::URL
}

pub fn new() -> IVideo {
  IVideo {
    key: Uuid::new_v5(),
    caption: "video-caption",
    thumbnail: Vec<ImageBuffer>::new(0,0),
    video_data: URL::parse("default-url-location.com").unwrap(),
  }
}