const getAsyncstringResultsNow: Function = (promisedThing: Promise<string>) => {
  promisedThing.finally().catch(() => {
    // throw error
  })
  return promisedThing
}

export default getAsyncstringResultsNow
