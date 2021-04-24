use ::gum::ffi;
use std;

pub type GumAddress = ffi::GumAddress;

pub fn gum_module_find_export_by_name(module_name: &str,
                                      symbol_name: &str) -> GumAddress {
  let module_cstr = std::ffi::CString::new(module_name).unwrap();
  let module_ptr = match module_name {
    "" => std::ptr::null(),
    _  => module_cstr.as_ptr()
  };
  let symbol_name = std::ffi::CString::new(symbol_name).unwrap();
  unsafe {
    ffi::gum_module_find_export_by_name(module_ptr, symbol_name.as_ptr())
  }
}

pub fn gum_module_find_symbol_by_name(module_name: &str,
                                      symbol_name: &str) -> GumAddress {
  let module_cstr = std::ffi::CString::new(module_name).unwrap();
  let module_ptr = match module_name {
    "" => std::ptr::null(),
    _  => module_cstr.as_ptr()
  };
  let symbol_name = std::ffi::CString::new(symbol_name).unwrap();
  unsafe {
    ffi::gum_module_find_symbol_by_name(module_ptr, symbol_name.as_ptr())
  }
}
