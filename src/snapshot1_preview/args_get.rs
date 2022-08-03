use std::ffi::c_void;
use v8::fast_api;

pub fn slow(
  _: &mut v8::HandleScope,
  _: v8::FunctionCallbackArguments,
  mut rv: v8::ReturnValue,
) {
  // TODO: get args
  rv.set_int32(0);
}

pub fn fast(
  _: v8::Local<v8::Object>,
  _argv_offset: i32,
  _argv_buffer_offset: i32,
  options: *mut fast_api::FastApiCallbackOptions,
) -> i32 {
  let options =
    unsafe { &mut *(options as *mut fast_api::FastApiCallbackOptions) };
  if !options.wasm_memory.is_null() {
    let mem = unsafe {
      &*(options.wasm_memory as *const fast_api::FastApiTypedArray<u8>)
    };
    if let Some(mem) = mem.get_storage_if_aligned() {
      // TODO: Set args to memory
      dbg!(mem.len());
      return 0;
    }
  };
  options.fallback = true;
  0
}

pub struct Fast;
impl fast_api::FastFunction for Fast {
  fn args(&self) -> &'static [fast_api::Type] {
    &[
      fast_api::Type::V8Value,
      fast_api::Type::Int32,
      fast_api::Type::Int32,
      fast_api::Type::CallbackOptions,
    ]
  }

  fn return_type(&self) -> fast_api::CType {
    fast_api::CType::Int32
  }

  fn function(&self) -> *const c_void {
    fast as _
  }
}
