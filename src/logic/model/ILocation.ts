import ITimeZone from './ITimeZone'
import INation from './INation'

interface ILocation {
  locationName: string
  streetnumber: string
  streetName: string
  cityOrProvince: string
  state: string
  country: INation
  postalCode: string
  mapURL: URL
  timeZone: ITimeZone
}

export default ILocation
