
let http = require('http');

let server = http.createServer((request, response) => {
    response.setHeader('content-type', 'text/html;charset=utf-8')
    //response.end("hello nodejs http server")
    response.end("你好")
});

server.listen(8080, () => {
    console.log("service start")
});