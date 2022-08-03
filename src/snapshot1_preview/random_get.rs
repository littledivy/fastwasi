use once_cell;
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use std::{cell::RefCell, ffi::c_void};
use v8::fast_api;

thread_local!(static RNG: RefCell<StdRng>  = RefCell::new(StdRng::from_entropy()));

pub fn slow(
  _: &mut v8::HandleScope,
  _: v8::FunctionCallbackArguments,
  mut rv: v8::ReturnValue,
) {
  rv.set_int32(0);
}

pub fn fast(
  _: v8::Local<v8::Object>,
  buffer_offset: i32,
  buffer_len: i32,
  options: *mut fast_api::FastApiCallbackOptions,
) -> i32 {
  let options =
    unsafe { &mut *(options as *mut fast_api::FastApiCallbackOptions) };
  if !options.wasm_memory.is_null() {
    let mem = unsafe {
      &*(options.wasm_memory as *const fast_api::FastApiTypedArray<u8>)
    };
    if let Some(mem) = mem.get_storage_if_aligned() {
      RNG.with(|rng| {
        let rng = &mut rng.borrow_mut();
        rng.fill(
          &mut mem
            [buffer_offset as usize..(buffer_offset + buffer_len) as usize],
        )
      });
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
