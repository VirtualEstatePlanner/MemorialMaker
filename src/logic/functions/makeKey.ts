import { v5 } from 'uuid'

const makeKey = () => v5(Date.now().toString(), '5b0df8e3-78a2-4c2a-b8c0-d8c5f8d1e8f7')

export default makeKey
