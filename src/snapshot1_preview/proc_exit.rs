use std::{ffi::c_void, process::exit};
use v8::fast_api;

pub fn slow(
  scope: &mut v8::HandleScope,
  args: v8::FunctionCallbackArguments,
  _: v8::ReturnValue,
) {
  let code = args.get(0).int32_value(scope).unwrap();
  exit(code);
}

pub fn fast(
  _: v8::Local<v8::Object>,
  code: i32,
  _: *mut fast_api::FastApiCallbackOptions,
) {
  exit(code);
}

pub struct Fast;
impl fast_api::FastFunction for Fast {
  fn args(&self) -> &'static [fast_api::Type] {
    &[
      fast_api::Type::V8Value,
      fast_api::Type::Int32,
      fast_api::Type::CallbackOptions,
    ]
  }

  fn return_type(&self) -> fast_api::CType {
    fast_api::CType::Void
  }

  fn function(&self) -> *const c_void {
    fast as _
  }
}
