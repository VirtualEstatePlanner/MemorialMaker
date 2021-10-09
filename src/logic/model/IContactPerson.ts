import IName from './IName'
import ITimeZone from './ITimeZone'

interface IContactPerson extends IName {
  phonenumber: string
  email: string
  timeZone: ITimeZone
}

export default IContactPerson
