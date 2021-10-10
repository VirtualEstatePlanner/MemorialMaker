pub struct IVideo {
  key: &str,
  caption: &str,
  thumbnail: Vector<image::ImageBuffer>,
  // generateThumbnail to an array of frames selected based on the number of frames in the video
  // probably requires video reading libaray such as ffmpeg
  videoData: url::URL
}
