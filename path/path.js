const path = require('path')


path.resolve(__dirname, './index.html')
console.log(path.resolve(__dirname, './index.html'))
console.log(path.resolve(__dirname, '/index.html'))


console.log(path.sep)
console.log(__filename)

let pathStr = '/hello/world.js'
console.log('path.parse:' + path.parse(pathStr))

// basename
console.log('path.basename:' + path.basename(pathStr))

// dirname
console.log('path.dirname:' + path.dirname(pathStr))

// extname
console.log('path.extname:' + path.extname(pathStr))