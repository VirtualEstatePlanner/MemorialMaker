// make charities array

import ICharity from '../model/ICharity'

const createBlankCharities = (numberOfCharities: number): ICharity[] => {
  const blankCharities: ICharity[] = []
  for (let i = 0; i < numberOfCharities; i++) {
    blankCharities.push({
      name: '',
      issue: '',
      url: new URL('https://megadocker.net'),
    })
  }
  const blankCharitiesArray = blankCharities
  return blankCharitiesArray
}

export default createBlankCharities
