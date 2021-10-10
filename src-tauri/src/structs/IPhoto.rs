use image::ImageBuffer;

pub struct IPhoto {
  caption: &str,
  original: ImageBuffer
  // generateThumbnail: (photo: ImageBitmap) => ImageBitmap // move to utility-functions
}

pub fn new() -> IPhoto {
  IPhoto {
    caption: "photo-caption",
    original: ImageBuffer::new(0,0)
  }
}