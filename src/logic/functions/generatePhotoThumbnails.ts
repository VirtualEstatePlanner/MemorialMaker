import glob from 'glob'
import Jimp from 'jimp'
import { fs } from '@tauri-apps/api'

const dir = './assets/gallery'
const thumbdir = './assets/thumbs'
const imagewidth = 1800

fs.removeDir(dir, { recursive: true })
fs.removeDir(thumbdir, { recursive: true })

if (!fs.readDir(dir))
  fs.createDir(dir)

if (!fs.readDir(thumbdir))
  fs.createDir(thumbdir)

const noCaseOption: boolean | undefined = true

glob('assets/images/*.+(png|jpg|jpeg|svg)', { nocase: noCaseOption }, async(_err: Error | null, images: string[]) => {
  for (const file of images) {
    const filenameold = file.replace('assets/images/', '')
    const filename = filenameold.toLowerCase().replace(/[^\w.]+/g, '_')

    Jimp.read(file)
      .then((jfile) => {
        if (jfile.bitmap.width > imagewidth) {
          return jfile
            .resize(imagewidth, Jimp.AUTO)
            .quality(90)
            .write(`${dir}/${filename}`)
        }
        else {
          fs.copyFile(file, `${dir}/${filename}`)
        }
      })
      .catch((err: Error | null) => console.error(err))

    Jimp.read(file)
      .then((jfile) => {
        let width = 120
        let height = Jimp.AUTO as number
        if (jfile.bitmap.height < jfile.bitmap.width) {
          width = Jimp.AUTO
          height = 120
        }
        return jfile
          .resize(width, height, Jimp.RESIZE_BEZIER)
          .quality(90)
          .write(`${thumbdir}/${filename}`)
      })
      .catch((err: Error | null) => console.error(err))
  }
})
