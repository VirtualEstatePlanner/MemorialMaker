import IContactPerson from './IContactPerson'
import ILivestreamInfo from './ILivestreamInfo'
import ILocation from './ILocation'
import IServiceAgendaItem from './IServiceAgendaItem'
import TServiceType from './TServiceType'

interface IService {
  key: number // should be generated by UUID
  additionalInfo: string
  contact: IContactPerson
  livestream: ILivestreamInfo
  location: ILocation
  type: TServiceType
  programme: IServiceAgendaItem[]
  time: Date
}

export default IService
