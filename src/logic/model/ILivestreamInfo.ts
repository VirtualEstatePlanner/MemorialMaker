interface ILivestreamInfo {
  requiresLogin: Boolean
  url: URL
  credentials?: {
    username: string
    password: string
  }
}

export default ILivestreamInfo
