import TNamePrefix from './TNamePrefix'
import TNameSuffix from './TNameSuffix'

interface IName {
  first: String
  middle: String
  last: String
  suffix: TNamePrefix | null
  prefix: TNameSuffix | null
}

export default IName
