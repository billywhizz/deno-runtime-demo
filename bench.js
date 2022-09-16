let TEXT = `HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: `
const CRLF2 = '\r\n\r\n'
const str = 'Hello, World!'
const resp = `${TEXT}${str.length}${CRLF2}${str}`
const payload = Uint8Array.from(resp, x => x.charCodeAt(0))

function bench (query) {
  const start = Date.now()
  for (let i = 0; i < count; i++) query()
  const elapsed = Date.now() - start
  const rate = Math.floor(count / (elapsed / 1000))
  console.log(`time ${elapsed} ms rate ${rate}`)
  if (--total) queueMicrotask(() => bench(query))
}

let total = 10
const count = 1000000
const O_RDONLY = 0
const O_WRONLY = 1
const O_RDWR = 2

const fd = runjs.open('/dev/null', O_WRONLY)

//runjs.write(2, payload, payload.length)
//runjs.writeString(2, resp, resp.length)

bench(() => runjs.write(fd, payload, payload.length))
//bench(() => runjs.writeString(fd, resp, resp.length))

