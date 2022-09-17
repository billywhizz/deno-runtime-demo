((globalThis) => {
  const { core } = Deno
  const { ops } = core
  globalThis.runjs = {
    core,
    args: () => ops.op_args(),
    close: fd => ops.op_close_sync.fast(fd),
    encode: text => ops.op_encode(text),
    encode_fast: text => ops.op_encode_fast(text),
    encode_into: (text, u8, res) => ops.op_encoding_encode_into(text, u8, res),
    encode_into_fast: (text, u8, res) => ops.op_encoding_encode_into_fast(text, u8, res),
    escape: text => ops.op_escape(text),
    open: (path, flags) => ops.op_open_sync(path, flags),
    write: (fd, buf, size) => ops.op_write_sync_slow(fd, buf, size),
    write_fast: (fd, buf, size) => ops.op_write_sync.fast(fd, buf, size),
    write_string: (fd, str, size) => ops.op_write_string_sync(fd, str, size),
    write_string_v8: (fd, str, size) => ops.op_write_string_sync_v8(fd, str, size),
  }
  globalThis.console = {
    log: str => core.print(`${str}\n`, false)
  }
})(globalThis)
