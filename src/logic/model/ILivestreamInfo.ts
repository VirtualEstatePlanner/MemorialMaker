interface ILivestreamInfo {
  requiresLogin: Boolean
  url: URL
  credentials?: {
    username: String
    password: String
  }
}

export default ILivestreamInfo
