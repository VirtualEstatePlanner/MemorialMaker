import IService from '~/logic/model/IService'

const cremation: IService = {
  additionalInfo: '',
  contact: {
    first: '',
    middle: '',
    last: '',
    prefix: null,
    suffix: null,
    email: '',
    phonenumber: '',
    timeZone: {
      name: '',
      abbreviation: '',
      offset: 0,
      isDaylightSavingsTime: false,
      description: '',
      utc: [],
    },
  },
  livestream: {
    url: new URL(''),
    requiresLogin: true,
    credentials: {
      username: '',
      password: '',
    },
  },
  location: {
    locationName: '',
    streetnumber: '',
    streetName: '',
    cityOrProvince: '',
    state: '',
    country: {
      alpha2: '',
      alpha3: '',
      name: '',
    },
    postalCode: '',
    mapURL: new URL(''),
    timeZone: {
      name: '',
      abbreviation: '',
      offset: 0,
      isDaylightSavingsTime: false,
      description: '',
      utc: [],
    },
  },
  programme: [],
  time: new Date(),
  //     key: number // should be generated by UUID
  key: 0,
  type: 'cremation',
}

export default cremation
