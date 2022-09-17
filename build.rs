use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::include_js_files;
use std::path::PathBuf;
use std::env;

fn main() {
  let o = PathBuf::from(env::var_os("OUT_DIR").unwrap());
  let snapshot_path = o.join("RUNJS_SNAPSHOT.bin");
  let mut js_runtime = JsRuntime::new(RuntimeOptions {
    will_snapshot: true,
    extensions: vec![
        Extension::builder()
        .js(include_js_files!(
            prefix "runjs:internal",
            "src/runtime.js",
          ))
        .build()
    ],
    ..Default::default()
  });
  let snapshot = js_runtime.snapshot();
  let snapshot_slice: &[u8] = &*snapshot;
  std::fs::write(&snapshot_path, snapshot_slice).unwrap();
}