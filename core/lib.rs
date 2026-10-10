use v8::{self, V8 as V8_Platform};

use std::fs;
use std::path::Path;
use std::sync::{self, Once};

#[derive(Default)]
pub struct RuntimeOptions {
    pub thread_pool_size: u32,
    pub idle_task_support: bool,
}

pub fn run(code: &str, options: &RuntimeOptions) {
    _init_platform(options.thread_pool_size, options.idle_task_support);

    let sandbox = &mut v8::Isolate::new(Default::default());

    v8::scope!(let sandbox_heap, sandbox);

    let runtime = v8::Context::new(sandbox_heap, Default::default());
    let runtime_globals = &mut v8::ContextScope::new(sandbox_heap, runtime);

    let source = v8::String::new(runtime_globals, code).expect("ERROR");

    v8::Script::compile(runtime_globals, source, None)
        .expect("ERROR")
        .run(runtime_globals);
}

pub fn run_file(path: impl AsRef<Path>, options: &RuntimeOptions) {
  let source = fs::read_to_string(path).expect("ERROR");

  run(&source, &options);
}

// TODO: where to put testing?
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        run("let a = 40; let b = 2; a + b;", &RuntimeOptions::default());
    }

    #[test]
    fn test_run_file() {
      run_file("./core/target.js", &RuntimeOptions::default())
    }
}

// platform
static SAFELY_START_V8: Once = sync::Once::new();

fn _init_platform(thread_pool_size: u32, idle_task_support: bool) {
    SAFELY_START_V8.call_once(|/* no arguments */| {
    V8_Platform::initialize_platform(
      v8::new_default_platform(thread_pool_size, idle_task_support).make_shared()
    );

    V8_Platform::initialize();
  });
}
