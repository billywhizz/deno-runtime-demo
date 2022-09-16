let total = 10
const runs = 3000000

var ma = /["'&<>]/;

function HTMLEscape(a) {
  if ("boolean" === typeof a || "number" === typeof a) return "" + a;
  a = "" + a;
  var b = ma.exec(a);
  if (b) {
    var c = "", d, f = 0;
    for (d = b.index; d < a.length; d++) {
      switch (a.charCodeAt(d)) {
          case 34:
            b = "&quot;";
            break;
          case 38:
            b = "&amp;";
            break;
          case 39:
            b = "&apos;";
            break;
          case 60:
            b = "&lt;";
            break;
          case 62:
            b = "&gt;";
            break;
          default:
            continue
      }
      f !== d && (c += a.substring(f, d));
      f = d + 1;
      c += b
    }
    a = f !== d ? c + a.substring(f, d) : c
  }
  return a
}

let fn

if (!globalThis.runjs) {
  globalThis.print = str => console.log(str)
}

function bench (query) {
  const start = Date.now()
  for (let i = 0; i < runs; i++) query()
  const elapsed = Date.now() - start
  const rate = Math.floor(runs / (elapsed / 1000))
  print(`time ${elapsed} ms rate ${rate}`)
  if (--total) queueMicrotask(() => bench(query))
}

function escapeHtml (str) {
  //if (!ma.exec(str)) return str
  return runjs.escape(str)
}

//const str = "012345678901234>012345678901234<0123456789012345&".repeat(4)
const str = '0'.repeat(256)

//const str = '<script type=\'\' src="">const s = "국제 회의가"</script>'
//bench(() => Deno.escapeHtml(str))
//bench(() => escapeHtml(str))

//bench(() => HTMLEscape(str))
//const b = new Uint8Array(6)
//b[0] = b[1] = b[2] = b[3] = b[4] = 36
//b[5] = '\n'.charCodeAt(0)
//runjs.write(1, b, 6)


//runjs.writeString(1, 'hello\n', 6)
