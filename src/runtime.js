((globalThis) => {
  const { core } = Deno
  globalThis.runjs = {
    core,
    escape: text => core.opSync('op_escape', text),
    open: (path, flags) => core.opSync('op_open_sync', path, flags),
    close: fd => core.opSync('op_close_sync', fd),
    write: (fd, buf, size) => core.opSync('op_write_sync', fd, buf, size),
    writeString: (fd, str, size) => core.opSync('op_write_string_sync', fd, str, size),
  }
  globalThis.console = {
    log: str => core.print(`${str}\n`, false)
  }
})(globalThis)
