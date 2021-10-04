import ICharity from '~/logic/model/ICharity'

export const validateCharity = (charity: ICharity): Boolean => charity.name !== undefined && charity.issue !== undefined && charity.url.href !== undefined
