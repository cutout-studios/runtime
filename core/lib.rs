use v8::{self, V8 as V8_Platform};

use swc_common::{BytePos, GLOBALS, Globals, Mark, SourceMap, sync::Lrc as RefCounter};
use swc_ecma_ast::{Pass, Program};
use swc_ecma_codegen::{Config, Emitter, text_writer::JsWriter};
use swc_ecma_parser::{Parser, StringInput, Syntax::Typescript};
use swc_ecma_transforms_typescript::strip;

use std::fs;
use std::path::Path;
use std::sync::{self, Once};

#[derive(Default)]
pub struct RuntimeOptions {
    pub thread_pool_size: u32,
    pub idle_task_support: bool,
}

pub fn transpile(code: &str) -> String {
    let globals = Globals::new();

    // make threadsafe
    GLOBALS.set(&globals, |/* no args */| {
        let input_ast = Parser::new(
            Typescript(Default::default()),
            StringInput::new(code, BytePos(1), BytePos(1 + code.len() as u32)),
            None,
        )
        .parse_module()
        .expect("ERROR");

        let mut program = Program::Module(input_ast);

        strip(Mark::new(), Mark::new()).process(&mut program);

        let output_ast = match program {
            Program::Module(m) => m,
            _ => unreachable!(),
        };

        let source_map_ref: RefCounter<SourceMap> = Default::default();

        let mut result = vec![];

        Emitter {
            cfg: Config::default(),
            comments: None,
            wr: JsWriter::new(source_map_ref.clone(), "\n", &mut result, None),
            cm: source_map_ref.clone(),
        }
        .emit_module(&output_ast)
        .expect("ERROR");

        String::from_utf8(result).expect("ERROR")
    })
}

pub fn run(code: &str, options: &RuntimeOptions) -> String {
    _init_platform(options.thread_pool_size, options.idle_task_support);

    let sandbox = &mut v8::Isolate::new(Default::default());

    v8::scope!(let sandbox_heap_tracker, sandbox);

    let runtime_environment = v8::Context::new(sandbox_heap_tracker, Default::default());
    let execution_context = &mut v8::ContextScope::new(sandbox_heap_tracker, runtime_environment);

    let v8_code = v8::String::new(execution_context, &transpile(code).to_string()).expect("ERROR");
    let v8_code_result = v8::Script::compile(execution_context, v8_code, None)
        .expect("ERROR")
        .run(execution_context);

    v8_code_result
        .expect("ERROR")
        .to_rust_string_lossy(execution_context)
}

pub fn run_file(path: impl AsRef<Path>, options: &RuntimeOptions) -> String {
    let source = fs::read_to_string(path).expect("ERROR");

    run(&source, &options)
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
