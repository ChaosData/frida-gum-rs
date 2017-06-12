use ::gum::ffi;

use libc::*;
use std;

#[derive(Debug)]
pub struct GumInvocationContext {
  real: *mut ffi::GumInvocationContext
}

pub enum GumPointCut {
  GUM_POINT_ENTER = 0,
  GUM_POINT_LEAVE = 1
}

impl GumInvocationContext {
  pub fn new(ic: *mut ffi::GumInvocationContext) -> GumInvocationContext {
    GumInvocationContext{ real: ic }
  }

  pub fn get_point_cut(&self) -> GumPointCut {
    let pc = unsafe { ffi::gum_invocation_context_get_point_cut(self.real) };
    match pc {
      pc if pc == GumPointCut::GUM_POINT_ENTER as u32 => GumPointCut::GUM_POINT_ENTER,
      pc if pc == GumPointCut::GUM_POINT_LEAVE as u32 => GumPointCut::GUM_POINT_LEAVE,
      _ => panic!()
    }
  }

  pub fn get_nth_argument<T>(&self, n: u32) -> *mut T {
    let value = unsafe {
      ffi::gum_invocation_context_get_nth_argument(self.real, n)
    } as *mut T;
    value
  }

  pub fn get_nth_argument_string(&self, n: u32) -> String {
    let value = self.get_nth_argument::<c_char>(n);
    std::str::from_utf8(
      unsafe { std::ffi::CStr::from_ptr(value) }.to_bytes()
    ).unwrap().to_owned()
  }

  pub fn replace_nth_argument<T>(&self, n: u32, value: *mut T) {
    unsafe {
      ffi::gum_invocation_context_replace_nth_argument(
        self.real, n, value as *mut c_void
      );
    }
  }

  pub fn get_return_value<T>(&self) -> *mut T {
    let ret = unsafe {
      ffi::gum_invocation_context_get_return_value(self.real)
    } as *mut T;
    ret
  }

  pub fn replace_return_value<T>(&self, value: *mut T) {
    unsafe {
      ffi::gum_invocation_context_replace_return_value(
        self.real, value as *mut c_void
      )
    }
  }

  pub fn get_return_address(&self) -> *mut c_void {
    unsafe {
      ffi::gum_invocation_context_get_return_address(self.real)
    }
  }

  pub fn get_thread_id(&self) -> u32 {
    unsafe {
      ffi::gum_invocation_context_get_thread_id(self.real)
    }
  }

  pub fn get_depth(&self) -> u32 {
    unsafe {
      ffi::gum_invocation_context_get_depth(self.real)
    }
  }

  fn get_listener_thread_data<T>(&self) -> *mut T {
    let data = unsafe {
      ffi::gum_invocation_context_get_listener_thread_data(
        self.real, std::mem::size_of::<T>()
      ) as *mut T
    };
    data
  }

  fn get_listener_function_data<T>(&self) -> *mut T {
    let data = unsafe {
      ffi::gum_invocation_context_get_listener_function_data(
        self.real
      ) as *mut T
    };
    data
  }

  pub fn get_listener_function_invocation_data<T>(&self) -> *mut T {
    let data = unsafe {
      ffi::gum_invocation_context_get_listener_function_invocation_data(
        self.real, std::mem::size_of::<T>()
      ) as *mut T
    };
    data
  }

  pub fn get_replacement_function_data<T>(&self) -> *mut T {
    let data = unsafe {
      ffi::gum_invocation_context_get_replacement_function_data(
        self.real
      ) as *mut T
    };
    data
  }

}

