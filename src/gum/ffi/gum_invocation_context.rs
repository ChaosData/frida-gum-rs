use libc::*;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct _GumInvocationContext {
  pub function: *const c_void, //GCallback
  pub cpu_context: *const c_void, //*mut GumCpuContext,
  pub system_error: i32, //gint
  pub backend: *mut c_void //*mut GumInvocationBackend,
}

impl Clone for _GumInvocationContext {
  fn clone(&self) -> Self { *self }
}
pub type GumInvocationContext = _GumInvocationContext;

//pub type _GumPointCut = u32; //guint;

//#[repr(u32)]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[allow(dead_code)] // rust can't seem to tell that it's returned by an extern
pub enum GumPointCut { GUM_POINT_ENTER = 0, GUM_POINT_LEAVE = 1, }

//#[link(name = "frida-gum")]
extern "C" {
  pub fn gum_invocation_context_get_point_cut(
    context: *const GumInvocationContext
  ) -> GumPointCut;

  pub fn gum_invocation_context_get_nth_argument(
    context: *mut GumInvocationContext,
    n: u32 //guint
  ) -> *mut c_void; //gpointer;

  pub fn gum_invocation_context_replace_nth_argument(
    context: *mut GumInvocationContext,
    n: u32, //guint,
    value: *mut c_void //gpointer
  );

  pub fn gum_invocation_context_get_return_value(
    context: *mut GumInvocationContext
  ) -> *mut c_void; //gpointer;

  pub fn gum_invocation_context_replace_return_value(
    context: *mut GumInvocationContext,
    value: *mut c_void //gpointer
  );

  pub fn gum_invocation_context_get_return_address(
    context: *mut GumInvocationContext
  ) -> *mut c_void; //gpointer;

  pub fn gum_invocation_context_get_thread_id(
    context: *mut GumInvocationContext
  ) -> u32; //guint;

  pub fn gum_invocation_context_get_depth(
    context: *mut GumInvocationContext
  ) -> u32; //guint;

  pub fn gum_invocation_context_get_listener_thread_data(
    context: *mut GumInvocationContext,
    required_size: usize //gsize
  ) -> *mut c_void; //gpointer;

  pub fn gum_invocation_context_get_listener_function_data(
    context: *mut GumInvocationContext
  ) -> *mut c_void; //gpointer;

  pub fn gum_invocation_context_get_listener_function_invocation_data(
    context: *mut GumInvocationContext,
    required_size: usize //gsize
  ) -> *mut c_void; //gpointer;

  pub fn gum_invocation_context_get_replacement_function_data(
    context: *mut GumInvocationContext
  ) -> *mut c_void; //gpointer;
}

