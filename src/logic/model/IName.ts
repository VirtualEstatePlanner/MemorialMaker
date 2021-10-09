import TNamePrefix from './TNamePrefix'
import TNameSuffix from './TNameSuffix'

interface IName {
  first: string
  middle: string
  last: string
  suffix: TNamePrefix | null
  prefix: TNameSuffix | null
}

export default IName
