import IAudio from './IAudio'

interface ISong extends IAudio {
  title: String
  artist: String | null | undefined
}

export default ISong
