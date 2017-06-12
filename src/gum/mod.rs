
mod ffi;

mod gum_invocation_context;
pub use self::gum_invocation_context::*;

pub use self::ffi::on_enter;
pub use self::ffi::on_leave;
pub use self::ffi::do_hook;

//use libc::*;

/*
pub fn gum_invocation_context_get_nth_argument(
  context: *mut c_void, //ffi::GumInvocationContext,
  n: u32 //guint
) -> *mut c_void {
  unsafe {
    ffi::gum_invocation_context_get_nth_argument(context as *mut ffi::GumInvocationContext, n)
  }
}

*/
