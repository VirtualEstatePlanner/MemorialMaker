//use image::ImageBuffer;
use url::Url;
use uuid::Uuid;

pub struct IVideo {
  key: Uuid,
  caption: String,
  //thumbnail: Vec<ImageBuffer>,
  // generateThumbnail to an array of frames selected based on the number of frames in the video
  // probably requires video reading libaray such as ffmpeg
  video_data: Url,
}

pub fn new() -> IVideo {
  IVideo {
    key: Uuid::new_v4(),
    caption: "video-caption".to_string(),
    //thumbnail: Vec::<ImageBuffer>::new(),
    video_data: Url::parse("default-url-location.com").unwrap(),
  }
}
