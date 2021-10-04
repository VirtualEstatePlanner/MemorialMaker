const getDaySuffix: Function = (day: number): string => {
  const dayAsstring: string = day.toString()
  const lastChar: string = dayAsstring.substr(dayAsstring.length - 1)
  const dayExt = dayAsstring === '11' ? 'th' : dayAsstring === '12' ? 'th' : lastChar === '1' ? 'st' : lastChar === '2' ? 'nd' : lastChar === '3' ? 'rd' : 'th'
  return dayExt
}

export default getDaySuffix
