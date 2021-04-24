extern crate libc;
use libc::*;

pub mod gum;
pub use gum::*;

include!(concat!(env!("OUT_DIR"), "/frida-gum-wrapper-code.rs"));

#[macro_export]
macro_rules! link {
  () => {
    extern crate base64;
    extern crate cc;

    use base64::decode;

    use std::fs::File;
    use std::io::Write;

    pub fn link() {
      let b64 = gumshoe::codeb64();
      let code = &decode(b64).unwrap();

      std::fs::create_dir_all("gen").unwrap();
      let mut f = File::create("gen/frida-gum-wrapper.c").expect("Unable to create file");
      f.write_all(code.as_slice()).expect("Unable to write data");

      println!("cargo:rustc-flags=-L frida/{}", std::env::var("TARGET").unwrap());
      println!("cargo:rustc-flags=-l frida-gum");
      println!("cargo:rustc-flags=-l dl");
      println!("cargo:rustc-flags=-l resolv");
      println!("cargo:rustc-flags=-l rt");
      println!("cargo:rustc-flags=-l m");
      println!("cargo:rustc-flags=-l pthread");

      cc::Build::new()
        .include("frida")
        .file("gen/frida-gum-wrapper.c")
        .compile("frida-gum-wrapper");
    }

    fn main() {
      link()
    }
  }
}

pub trait ArchetypalListener {
  fn on_enter(&mut self, gum::GumInvocationContext);
  fn on_leave(&mut self, gum::GumInvocationContext);
}

pub fn hook_exported_by_name<T: ArchetypalListener>(
    this: &mut T, symname: &str
  ) -> *mut gum::ffi::ArchetypalListener {
  hook_exported_by_modname(this, "", symname)
}

pub fn hook_exported_by_modname<T: ArchetypalListener>(
    this: &mut T, modname: &str, symname: &str
  ) -> *mut gum::ffi::ArchetypalListener {
  let funcptr = gum::gum_module_find_export_by_name(
    modname,
    symname
  );
  if funcptr == 0 {
    return std::ptr::null_mut();
  }

  hook_by_addr(this, funcptr)
}

pub fn hook_by_addr<T: ArchetypalListener>(
    this: &mut T, addr: usize
  ) -> *mut gum::ffi::ArchetypalListener {
  unsafe { gum::ffi::gum_init_embedded(); }
  let interceptor = unsafe { gum::ffi::gum_interceptor_obtain() };
  let al = unsafe { gum::ffi::get_new_archetypal_listener() };
  unsafe {
    gum::ffi::ArchetypalListener_fill(al,
      gum::on_enter::<T> as *const c_void,
      gum::on_leave::<T> as *const c_void,
      this as *mut T as *mut c_void,
      interceptor as *mut c_void
    );
  }
  unsafe { gum::ffi::setup_hook(interceptor, al, addr); }
  al
}

pub fn detach_hook(raw_listener: *mut gum::ffi::ArchetypalListener) {
  unsafe {
    gum::ffi::ArchetypalListener_detach(raw_listener)
  }
}
