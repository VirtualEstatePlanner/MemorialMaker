import IService from '~/logic/model/IService'

const casting: IService = {
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
    url: new URL('https://megadocker.net'),
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
    mapURL: new URL('https://megadocker.net'),
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
  type: 'scattering',
}

export default casting
