
pub mod ffi;

pub mod gum_invocation_context;
pub use self::gum_invocation_context::*;
pub mod gum_module;
pub use self::gum_module::*;

pub extern "C" fn on_enter<T: ::ArchetypalListener>(this: *mut T,
                                   ic: *mut ffi::GumInvocationContext) {
  unsafe {
    (*this).on_enter(GumInvocationContext::new(ic))
  }
}

pub extern "C" fn on_leave<T: ::ArchetypalListener>(this: *mut T,
                                   ic: *mut ffi::GumInvocationContext) {
  unsafe {
    (*this).on_leave(GumInvocationContext::new(ic))
  }
}

pub fn gum_init_embedded() {
  unsafe {
    ffi::gum_init_embedded();
  }
}
