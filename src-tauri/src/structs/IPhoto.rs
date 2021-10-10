pub struct IPhoto {
  caption: &str,
  original: ImageBitmap
  // generateThumbnail: (photo: ImageBitmap) => ImageBitmap // move to utility-functions
}

pub fn new() -> IPhoto {
  IPhoto {
    caption: "photo-caption",
    original: ImageBitmap::new()
  }
}