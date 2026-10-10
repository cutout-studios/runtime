use std::sync::{self, Once};
use v8::{self, V8 as V8_Platform};

#[derive(Default)]
pub struct RuntimeOptions {
    pub thread_pool_size: u32,
    pub idle_task_support: bool, // TODO: runtime modules (fetch, webgpu, and so on)
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

// pub fn run_file() {}

static SAFELY_START_V8: Once = sync::Once::new();

fn _init_platform(thread_pool_size: u32, idle_task_support: bool) {
    SAFELY_START_V8.call_once(|/* no arguments */| {
    V8_Platform::initialize_platform(
      v8::new_default_platform(thread_pool_size, idle_task_support).make_shared()
    );

    V8_Platform::initialize();
  });
}
