interface IPhoto {
  caption: string
  original: ImageBitmap
  generateThumbnail: (photo: ImageBitmap) => ImageBitmap // move to utility-functions
}

export default IPhoto
