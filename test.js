const {getOpenedList} = require('./index')

const data = getOpenedList("C:\\Users\\Admin\\AppData\\Roaming\\Code\\User\\globalStorage\\state.vscdb")
console.log(data.entries.length)