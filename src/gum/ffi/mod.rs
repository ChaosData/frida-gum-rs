
mod gum_invocation_context;
pub use self::gum_invocation_context::*;

mod gum_module;
pub use self::gum_module::*;

use libc::*;

#[repr(C)]
pub struct ArchetypalListener {
  _hidden: *mut c_void
}

extern "C" {
  pub fn gum_init_embedded() -> ();
  pub fn gum_interceptor_obtain() -> *mut c_void;
  pub fn get_new_archetypal_listener() -> *mut ArchetypalListener;

  pub fn setup_hook(interceptor: *mut c_void,
                    raw_listener: *mut ArchetypalListener,
                    addr: usize);

  pub fn gum_interceptor_detach(interceptor: *mut c_void,
                                listener: *mut c_void);

  pub fn g_object_unref(obj: *const c_void) -> ();

  pub fn gum_deinit_embedded() -> ();

  pub fn ArchetypalListener_fill(this: *mut ArchetypalListener,
                                 on_enter: *const c_void,
                                 on_leave: *const c_void,
                                 listener: *mut c_void,
                                 interceptor: *mut c_void);

  pub fn ArchetypalListener_detach(this: *mut ArchetypalListener);

}
