use std::ffi::CString;

pub mod snapshot1_preview;

macro_rules! register_fn {
  ($scope: ident, $global: ident, $slow: ident) => {
    let template = v8::FunctionTemplate::builder(
      snapshot1_preview::$slow::slow,
    )
    .build_fast($scope, &snapshot1_preview::$slow::Fast, None);
    let name = v8::String::new($scope, stringify!($slow)).unwrap();
    let value = template.get_function($scope).unwrap();
    $global.set($scope, name.into(), value.into()).unwrap();
  };
}

struct Env {
  #[allow(dead_code)]
  args: Vec<CString>,
  // env, fds
}

pub const V8_WRAPPER_TYPE_INDEX: i32 = 0;
pub const V8_WRAPPER_OBJECT_INDEX: i32 = 1;

fn main() {
  v8::V8::set_flags_from_string("--no_freeze_flags_after_init --allow_natives_syntax --turbo_fast_api_calls");
  v8::V8::initialize_platform(v8::new_default_platform(0, false).make_shared());
  v8::V8::initialize();
  let isolate = &mut v8::Isolate::new(
    v8::CreateParams::default().embedder_wrapper_type_info_offsets(
      V8_WRAPPER_TYPE_INDEX,
      V8_WRAPPER_OBJECT_INDEX,
    ),
  );
  let scope = &mut v8::HandleScope::new(isolate);
  let context = v8::Context::new(scope);
  let scope = &mut v8::ContextScope::new(scope, context);

  let object_template = v8::ObjectTemplate::new(scope);
  assert!(object_template
    .set_internal_field_count((V8_WRAPPER_OBJECT_INDEX + 1) as usize));

  let obj = object_template.new_instance(scope).unwrap();
  let embedder_obj = Box::into_raw(Box::new(Env { args: vec![] }));
  obj.set_aligned_pointer_in_internal_field(
    V8_WRAPPER_OBJECT_INDEX,
    embedder_obj as _,
  );

  register_fn!(scope, obj, args_get);
  register_fn!(scope, obj, random_get);

  {}
  let template = v8::FunctionTemplate::builder(
    |scope: &mut v8::HandleScope,
     args: v8::FunctionCallbackArguments,
     _: v8::ReturnValue| {
      let string = args.get(0).to_rust_string_lossy(scope);
      print!("{}", string);
    },
  )
  .build(scope);
  let name = v8::String::new(scope, "print").unwrap();
  let value = template.get_function(scope).unwrap();
  obj.set(scope, name.into(), value.into()).unwrap();

  let obj_str = v8::String::new(scope, "wasi").unwrap();
  let global = context.global(scope);
  global.set(scope, obj_str.into(), obj.into()).unwrap();

  let tc = &mut v8::TryCatch::new(scope);
  let promise = {
    let source =
      v8::String::new(tc, include_str!("../examples/hello_world.js")).unwrap();
    let scope = &mut v8::EscapableHandleScope::new(tc);
    let script = v8::Script::compile(scope, source, None).unwrap();

    let promise = script.run(scope).unwrap();

    scope.escape(promise)
  };

  let promise: v8::Local<v8::Promise> = promise.try_into().unwrap();

  while v8::Platform::pump_message_loop(
    &v8::V8::get_current_platform(),
    tc,
    true, // don't block if there are no tasks
  ) {
    match promise.state() {
      v8::PromiseState::Pending => {}
      v8::PromiseState::Rejected => {
        let message = tc.exception().unwrap().to_rust_string_lossy(tc);
        println!("Promise rejected: {}", message);
        break;
      }
      _ => {
        break;
      }
    }
  }
}
