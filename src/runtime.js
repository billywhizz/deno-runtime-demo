((globalThis) => {
  const { core } = Deno;
  const { ops } = core;
  globalThis.runjs = {
    core,
    escape: text => core.opSync('op_escape', text),
    open: (path, flags) => core.opSync('op_open_sync', path, flags),
    close: fd => ops.op_close_sync.fast(fd),
    write: (fd, buf, size) => ops.op_write_sync.fast(fd, buf, size),
    writeString: (fd, str, size) => core.opSync('op_write_string_sync', fd, str, size),
  }
  globalThis.console = {
    log: str => core.print(`${str}\n`, false)
  }
})(globalThis)
