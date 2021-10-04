import TNamePrefix from './TNamePrefix'
import TNameSuffix from './TNameSuffix'

interface IPerson {
  first: string
  middle: string
  last: string
  suffix: TNamePrefix | null
  prefix: TNameSuffix | null
}

export default IPerson
