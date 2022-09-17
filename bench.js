function bench (query) {
  const start = Date.now()
  for (let i = 0; i < count; i++) query()
  const elapsed = Date.now() - start
  const rate = Math.floor(count / (elapsed / 1000))
  console.log(`time ${elapsed} ms rate ${rate}`)
  if (--total) queueMicrotask(() => bench(query))
}

const O_RDONLY = 0
const O_WRONLY = 1
const O_RDWR = 2
const u8 = new Uint8Array(4096)
const res = new Uint32Array(2)
let TEXT = `HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: `
const CRLF2 = '\r\n\r\n'
const str = 'Hello, World!'
const resp = `${TEXT}${str.length}${CRLF2}${str}`
const payload = Uint8Array.from(resp, x => x.charCodeAt(0))

let args = []
if (globalThis.runjs) {
  args = runjs.args().slice(2)
} else {
  args = Deno.args
}

let total = parseInt(args[0] || '5', 10)
const count = parseInt(args[1] || '10000000', 10)
const testName = args[2] || 'default'

const tests = {
  runjs_write: () => bench(() => runjs.write(fd, payload, payload.length)),
  runjs_write_fast: () => bench(() => runjs.write_fast(fd, payload, payload.length)),
  runjs_encode_into: () => bench(() => runjs.encode_into(resp, u8, res)),
  runjs_encode_into_fast: () => bench(() => runjs.encode_into_fast(resp, u8, res)),
  deno_encode_into: () => {
    const { ops } = Deno.core
    bench(() => ops.op_encoding_encode_into(resp, u8))
  }
}

tests.default = tests.deno_encode_into

tests[testName]()
