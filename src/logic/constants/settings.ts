// should be part of interface IAppPreferences
/*
displayOnlyMiddleInitial: () => Boolean // from checkbox or switch component
*/

// should be interface ILivestreamInfo
/*
areLivestreamingService: () => Boolean // from checkbox or switch component
areLivestreamingPublicly: () => Boolean // from checkbox or switch component
livestreamLogin: () => String // from text input component
livestreamPassword: () => String // from text input component
serviceBooleans: () => {areHavingservice, areLivestreamingservice, serviceLivestreamIsPublic}
*/

// check boolean logic

// should be part of ILovingMemoryApplicationState
/*
numberOfCharities: () => number
numberOfServices: () => number
*/

// was highlighted message on services page
/*
const replaceDefaultMessage = (defaultValue: string, inputValue: string | null | undefined): string =>
  (inputValue === null || inputValue === undefined || inputValue === defaultValue) ? defaultValue : inputValue
*/

// should be part of IService[]
/*
serviceContactEmail: 'georgegeorgulas@gmail.com'
serviceEmailURL: URL: new URL(`mailto:${contactEmail}`)
serviceStreetAddress: 'Please enter the street address for this service'
serviceCityAddress: 'Please enter the name of the town or city where this service is being held'
serviceStateAddress: '' // should be a state abbreviation from a dropdown menu with text input option
serviceZipAddress: 'Please enter the zip code where the service is being held' 12345
serviceTimeZone: () => ITimeZone // should be from tz picker dropdown with text input option and that autofills based on zip/city/state info from some map API
serviceDate: () => Date() // should be from datepicker component
serviceBeginsTime: { hours:number, minutes: number }: new Date().getTime() // should be from time picker component
serviceEndsTime: { hours:number, minutes: number }: new Date().getTime() // should be from time picker component
serviceLivestreamURL: URL: new URL('https://megadocker.net')
serviceLivestreamUsername: 'loginusername'
serviceLivestreamPassword: 'loginpassword'
serviceAgendaItems: () => IServiceAgendaItem[]

serviceOpeningSong: 'some song'
serviceScriptureReading: 'some reading'
serviceOpeningPrayer: 'some prayer'
serviceSpeaker: 'some person'
serviceClosingPrayer: 'some other prayer'
serviceClosingSong: 'some other song'

cremationOpeningSong: 'some song'
cremationScriptureReading: 'some reading'
cremationOpeningPrayer: 'some prayer'
cremationSpeaker: 'some person'
cremationClosingPrayer: 'some other prayer'
cremationClosingSong: 'some other song'

charities: ICharity[]: createBlankCharities(numberOfCharities)
*/

// should be part of IMemorializedPerson
/*
namePrefix: TNamePrefix: 'Mr.'
nameSuffix: TNameSuffix: 'IV'
firstName: 'George'
middleName: 'Paul'
middleInitial: middleName.substr(0, 1)
lastName: 'Georgulas'
fullName: `${namePrefix ? `${namePrefix} ` : ''}${firstName} ${displayOnlyMiddleInitial ? `${middleInitial}.` : middleName} ${lastName}${nameSuffix ? ` ${nameSuffix}` : ''}`
birthMonthAsNumber: 5
birthMonthAsWord: getMonthName(birthMonthAsNumber)
birthDay: 3
birthDayExt: getDaySuffix(birthDay)
birthYear: 1979
dateOfBirth: `${birthMonthAsNumber} ${birthDay}${birthDayExt} ${birthYear}`
dateOfBirthWithMonthWord: `${birthMonthAsWord} ${birthDay}${birthDayExt} ${birthYear}`
deathMonth: 10
deathMonthAsWord: getMonthName(deathMonth)
deathDay: 3
deathDayExt: getDaySuffix(deathDay)
deathYear: 2021
dateOfDeath: `${deathMonth} ${deathDay}${deathDayExt} ${deathYear}`
dateOfDeathWithMonthWord: `${deathMonthAsWord} ${deathDay}${deathDayExt} ${deathYear}`
*/
