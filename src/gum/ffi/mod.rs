
mod gum_invocation_context;
pub use self::gum_invocation_context::*;

use ::ArchetypalListener;

use libc::*;
use std;

extern {
  fn gum_init_embedded() -> ();
  fn gum_interceptor_obtain() -> *mut c_void;
  fn get_new_archetypal_listener() -> *mut c_void;
  fn ArchetypalListener_fill(this: *mut c_void,
                             on_enter: *const c_void,
                             on_leave: *const c_void,
                             obj: *mut c_void);

  fn setup_hook(interceptor: *mut c_void,
                listener: *mut c_void,
                addr: usize);

  fn gum_module_find_export_by_name(module_name: *const c_char,
                                    symbol_name: *const c_char) -> usize;
  fn gum_interceptor_detach_listener(interceptor: *mut c_void,
                                     listener: *mut c_void);
  fn g_object_unref(obj: *const c_void) -> ();
  fn gum_deinit_embedded() -> ();


}


pub fn on_enter<T: ArchetypalListener>(this: &mut T,
                                   ic: *mut ::gum::ffi::GumInvocationContext) {
  this.on_enter(::gum::GumInvocationContext::new(ic))
}

pub fn on_leave<T: ArchetypalListener>(this: &mut T,
                                   ic: *mut ::gum::ffi::GumInvocationContext) {
  this.on_leave(::gum::GumInvocationContext::new(ic))
}

pub fn do_hook<T: ArchetypalListener>(symname: &str,
                                      listener: &mut T) {
  unsafe { gum_init_embedded(); }
  let interceptor = unsafe { gum_interceptor_obtain() };
  let al = unsafe { get_new_archetypal_listener() };
  unsafe {
    ArchetypalListener_fill(al,
      on_enter::<T> as *const c_void,
      on_leave::<T> as *const c_void,
      listener as *mut T as *mut c_void
    );
  }
  let funcptr = unsafe {
    gum_module_find_export_by_name(
      std::ptr::null(),
      std::ffi::CString::new(symname).unwrap().as_ptr()
    )
  };

  unsafe {
    setup_hook(interceptor, al, funcptr);
  }
}

