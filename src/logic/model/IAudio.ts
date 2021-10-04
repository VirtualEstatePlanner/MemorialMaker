interface IAudio {
  audioData: URL // | file location | m4a | mp3 | stream data
  duration: number | null // inSeconds
}

export default IAudio
