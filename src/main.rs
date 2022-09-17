use crate::serde_v8::from_v8;
use deno_core::op;
use deno_core::Extension;
use deno_core::Snapshot;
use deno_core::serde_v8;
use deno_core::v8;
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

#[op(v8)]
fn op_encoding_encode_into_fast(
  scope: &mut v8::HandleScope,
  input: serde_v8::Value,
  buffer: &mut [u8],
  out_buf: &mut [u8],
) {
  let text = v8::Local::<v8::String>::try_from(input.v8_value).unwrap();
  let out_buf: &mut [u32] = unsafe {
    std::slice::from_raw_parts_mut(out_buf.as_mut_ptr() as *mut u32, 2)
  };
  let mut nchars = 0;
  out_buf[1] = text.write_utf8(
    scope,
    buffer,
    Some(&mut nchars),
    v8::WriteOptions::NO_NULL_TERMINATION
      | v8::WriteOptions::REPLACE_INVALID_UTF8,
  ) as u32;
  out_buf[0] = nchars as u32;
}

#[op(v8)]
fn op_encoding_encode_into(
  scope: &mut v8::HandleScope,
  input: serde_v8::Value,
  buffer: &mut [u8],
  out_buf: &mut [u8],
) {
  let text = v8::Local::<v8::String>::try_from(input.v8_value).unwrap();
  let text_str = serde_v8::to_utf8(text, scope);
  let out_buf: &mut [u32] = unsafe {
    std::slice::from_raw_parts_mut(out_buf.as_mut_ptr() as *mut u32, 2)
  };
  let boundary = if buffer.len() >= text_str.len() {
    text_str.len()
  } else {
    let mut boundary = buffer.len();
    for _ in 0..4 {
      if text_str.is_char_boundary(boundary) {
        break;
      }
      debug_assert!(boundary > 0);
      boundary -= 1;
    }
    debug_assert!(text_str.is_char_boundary(boundary));
    boundary
  };
  buffer[..boundary].copy_from_slice(text_str[..boundary].as_bytes());
  out_buf[0] = text_str[..boundary].encode_utf16().count() as u32;
  out_buf[1] = boundary as u32;
}

#[op(v8)]
fn op_encode<'a>(
  scope: &mut v8::HandleScope,
  text: serde_v8::Value,
) -> serde_v8::Value<'a> {
  let text = v8::Local::<v8::String>::try_from(text.v8_value).unwrap();
  let text_str = serde_v8::to_utf8(text, scope);
  let bytes = text_str.into_bytes();
  let len = bytes.len();
  let backing_store =
    v8::ArrayBuffer::new_backing_store_from_vec(bytes).make_shared();
  let buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
  let u8array = v8::Uint8Array::new(scope, buffer, 0, len).unwrap();
  from_v8(scope, u8array.into()).unwrap()
}

#[op(v8)]
fn op_encode_fast<'a>(
  scope: &mut v8::HandleScope,
  text: serde_v8::Value<'a>,
) -> serde_v8::Value<'a> {
  let s = v8::Local::<v8::String>::try_from(text.v8_value).unwrap();
  let len = s.length();
  let capacity = (len as f64 * 1.2) as usize;
  let mut buf = Vec::with_capacity(capacity);
  let mut nchars = 0;
  let data = buf.as_mut_ptr();
  let length = s.write_utf8(
    scope,
    unsafe { std::slice::from_raw_parts_mut(data, len) },
    Some(&mut nchars),
    v8::WriteOptions::NO_NULL_TERMINATION
      | v8::WriteOptions::REPLACE_INVALID_UTF8,
  );
  unsafe { buf.set_len(length) };
  let backing_store =
    v8::ArrayBuffer::new_backing_store_from_vec(buf).make_shared();
  let buffer = v8::ArrayBuffer::with_backing_store(scope, &backing_store);
  from_v8(scope, buffer.into()).unwrap()
}

#[op]
fn op_escape(text: String) -> Result<String, AnyError> {
  Ok(escape(&text).to_string())
}

#[op(v8)]
fn op_args<'scope>(
  scope: &mut v8::HandleScope<'scope>,
) -> Result<serde_v8::Value<'scope>, AnyError> {
  let args: Vec<String> = env::args().collect();
  let array = v8::Array::new(scope, args.len() as i32);
  let mut i = 0;
  for arg in args {
    let str = v8::String::new(scope, &arg).unwrap().into();
    array.set_index(scope, i, str);
    i += 1;
  }
  let array_value: v8::Local<v8::Value> = array.into();
  Ok(array_value.into())
}

/*
#[op(v8)]
fn op_escape_v8(
  scope: &mut v8::HandleScope,
  str: serde_v8::Value
) -> Result<serde_v8::Value<'a>, Error> {
  let s = v8::Local::<v8::String>::try_from(str.v8_value).unwrap();
  Ok(escape(&s.to_string(scope)))
}
*/

/*

#[op(fast)]
fn op_escape_fast(
  bytes: &[u8]
) -> &[u8] {
  Ok(b_escape(bytes))
}
 */

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
fn op_write_sync_slow(
  fd: i32,
  bytes: &[u8],
  size: u32,
) -> i32 {
  let rc = unsafe { write(fd, bytes.as_ptr(), size) };
  rc
}

#[op(v8)]
fn op_write_string_sync_v8(
  scope: &mut v8::HandleScope,
  fd: i32,
  str: serde_v8::Value,
  size: u32,
) -> i32 {
  let s = v8::Local::<v8::String>::try_from(str.v8_value).unwrap();
  let rc = unsafe { write(fd, s.to_rust_string_lossy(scope).as_ptr(), size) };
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

static RUNJS_SNAPSHOT: &[u8] =
      include_bytes!(concat!(env!("OUT_DIR"), "/RUNJS_SNAPSHOT.bin"));

async fn run_js(file_path: &str) -> Result<(), AnyError> {
  let main_module = deno_core::resolve_path(file_path)?;
  let runjs_extension = Extension::builder()
    .ops(vec![
      op_escape::decl(),
      op_write_sync::decl(),
      op_write_sync_slow::decl(),
      op_open_sync::decl(),
      op_close_sync::decl(),
      op_write_string_sync::decl(),
      op_write_string_sync_v8::decl(),
      op_encode::decl(),
      op_encode_fast::decl(),
      op_encoding_encode_into::decl(),
      op_encoding_encode_into_fast::decl(),
      op_args::decl(),
    ])
    .build();
  let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
    module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
    extensions: vec![runjs_extension],
    startup_snapshot: Some(Snapshot::Static(&*RUNJS_SNAPSHOT)),
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
