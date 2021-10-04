import IPerson from './IPerson'

interface IMemorializedPerson extends IPerson {
  dateOfBirth: Date
  dateOfDeath: Date
  // lifespanInDays: (dateOfBirth: Date, dateOfDeath: Date) => number // move to utility-functions
  // lifespanInYears: (lifespanInDays: number) => number // move to utility-functions
}

export default IMemorializedPerson
