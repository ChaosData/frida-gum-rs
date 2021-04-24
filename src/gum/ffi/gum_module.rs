use libc::*;

pub type GumAddress = usize;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct _GumMemoryRange {
  pub base_address: GumAddress,
  pub size: usize,
}
impl Clone for _GumMemoryRange {
  fn clone(&self) -> Self { *self }
}
pub type GumMemoryRange = _GumMemoryRange;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct _GumModuleDetails {
  pub name: *const c_char,
  pub range: *const GumMemoryRange,
  pub path: *const c_char,
}
impl Clone for _GumModuleDetails {
  fn clone(&self) -> Self { *self }
}
pub type GumModuleDetails = _GumModuleDetails;

//#[link(name = "frida-gum")]
extern "C" {
  pub fn gum_process_enumerate_modules(
    func: extern "C" fn(
      details: *const GumModuleDetails,
      user_details: *mut c_void
    ) -> c_int,
    user_data: *mut c_void
  );

  pub fn gum_module_find_export_by_name(module_name: *const c_char,
                                        symbol_name: *const c_char) -> GumAddress;

  pub fn gum_module_find_symbol_by_name(module_name: *const c_char,
                                        symbol_name: *const c_char) -> GumAddress;
  
}
