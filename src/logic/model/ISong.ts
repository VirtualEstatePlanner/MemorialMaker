import IAudio from './IAudio'

interface ISong extends IAudio {
  title: string
  artist: string | null
}

export default ISong
