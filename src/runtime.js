((globalThis) => {
  const { core } = Deno
  const { ops } = core
  globalThis.runjs = {
    core,
    escape: text => ops.op_escape(text),
    open: (path, flags) => ops.op_open_sync(path, flags),
    close: fd => ops.op_close_sync.fast(fd),
    writeSlow: (fd, buf, size) => ops.op_write_sync_slow(fd, buf, size),
    write: (fd, buf, size) => ops.op_write_sync.fast(fd, buf, size),
    writeString: (fd, str, size) => ops.op_write_string_sync(fd, str, size),
    writeStringv8: (fd, str, size) => ops.op_write_string_sync_v8(fd, str, size),
  }
  globalThis.console = {
    log: str => core.print(`${str}\n`, false)
  }
})(globalThis)
