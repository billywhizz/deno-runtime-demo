use deno_core::op;
use deno_core::Extension;
use deno_core::error::AnyError;
use std::rc::Rc;
use v_htmlescape::escape;
use std::env;

extern "C" {
  pub fn write(
    fd: ::std::os::raw::c_int,
    buf: *const ::std::os::raw::c_uchar,
    count: ::std::os::raw::c_uint
  ) -> ::std::os::raw::c_int;

  pub fn open(
    path: *const ::std::os::raw::c_uchar,
    flags: ::std::os::raw::c_int
  ) -> ::std::os::raw::c_int;

  pub fn close(
    fd: ::std::os::raw::c_int
  ) -> ::std::os::raw::c_int;
}

#[op]
fn op_escape(text: String) -> Result<String, AnyError> {
  Ok(escape(&text).to_string())
}

#[op]
fn op_open_sync(
  path: String,
  flags: i32,
) -> i32 {
  let rc = unsafe { open(path.as_ptr(), flags) };
  rc
}

#[op(fast)]
fn op_close_sync(
  fd: i32,
) -> i32 {
  let rc = unsafe { close(fd) };
  rc
}

#[op(fast)]
fn op_write_sync(
  fd: i32,
  bytes: &[u8],
  size: u32,
) -> i32 {
  let rc = unsafe { write(fd, bytes.as_ptr(), size) };
  rc
}

#[op]
fn op_write_string_sync(
  fd: i32,
  str: String,
  size: u32,
) -> i32 {
  let rc = unsafe { write(fd, str.as_ptr(), size) };
  rc
}

#[op(v8)]
fn op_encode<'a>(
  scope: &mut v8::HandleScope<'a>,
  text: serde_v8::Value<'a>,
) -> Result<serde_v8::Value<'a>, Error> {

}

async fn run_js(file_path: &str) -> Result<(), AnyError> {
  let main_module = deno_core::resolve_path(file_path)?;
  let runjs_extension = Extension::builder()
    .ops(vec![
      op_escape::decl(),
      op_write_sync::decl(),
      op_open_sync::decl(),
      op_close_sync::decl(),
      op_write_string_sync::decl()
    ])
    .build();
  let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
    module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
    extensions: vec![runjs_extension],
    ..Default::default()
  });
  const RUNTIME_JAVASCRIPT_CORE: &str = include_str!("./runtime.js");
  js_runtime
    .execute_script("[runjs:runtime.js]", RUNTIME_JAVASCRIPT_CORE)
    .unwrap();
  let mod_id = js_runtime.load_main_module(&main_module, None).await?;
  let result = js_runtime.mod_evaluate(mod_id);
  js_runtime.run_event_loop(false).await?;
  result.await?
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let script_name = &args[1];
  let runtime = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();
  if let Err(error) = runtime.block_on(run_js(script_name)) {
    eprintln!("error: {}", error);
  }
}
