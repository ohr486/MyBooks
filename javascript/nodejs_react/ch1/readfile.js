const fs = require('fs')

fs.readFile('kakugen.txt', 'utf-8', kakugenLoaded)

function kakugenLoaded (err, data) {
  if (err) {
    console.log('読み込みに失敗')
    return
  }
  console.log(data)
}
