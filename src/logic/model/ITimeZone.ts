interface ITimeZone {
  name: string
  abbreviation: string
  offset: number
  isDaylightSavingsTime: Boolean
  description: string
  utc: string[]
}

export default ITimeZone
