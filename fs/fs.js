
let fs = require('fs')

fs.writeFile('./test.md', 'test hello', err => {
    if (err) {
        console.log('write error')
        return
    }
    console.log('write success')
})