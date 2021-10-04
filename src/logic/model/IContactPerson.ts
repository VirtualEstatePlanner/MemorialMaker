import IPerson from './IPerson'
import ITimeZone from './ITimeZone'

interface IContactPerson extends IPerson {
  phonenumber: string
  email: string
  timeZone: ITimeZone
}

export default IContactPerson
