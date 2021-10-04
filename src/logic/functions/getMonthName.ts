const getMonthName = (month: number): string => {
  const monthNames = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December']
  const monthIndex = month - 1
  return monthNames[monthIndex]
}

export default getMonthName
