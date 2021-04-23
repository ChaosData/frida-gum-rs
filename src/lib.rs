extern crate libc;
use libc::*;

pub mod gum;

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

extern {
  pub fn gum_init_embedded() -> ();
  pub fn gum_interceptor_obtain() -> *mut c_void;
  pub fn get_new_archetypal_listener() -> *mut c_void;
  pub fn ArchetypalListener_fill(this: *mut c_void,
                                 on_enter: *const c_void,
                                 on_leave: *const c_void,
                                 obj: *mut c_void);

  pub fn setup_hook(interceptor: *mut c_void,
                    listener: *mut c_void,
                    addr: usize);

  pub fn gum_module_find_export_by_name(module_name: *const c_char,
                                        symbol_name: *const c_char) -> usize;
  pub fn gum_interceptor_detach_listener(interceptor: *mut c_void,
                                         listener: *mut c_void);
  pub fn g_object_unref(obj: *const c_void) -> ();
  pub fn gum_deinit_embedded() -> ();
}


pub trait ArchetypalListener {
  fn on_enter(&mut self, gum::GumInvocationContext);
  fn on_leave(&mut self, gum::GumInvocationContext);
  fn ptr(&mut self) -> *mut c_void;
}

pub fn hook_exported_by_name<T: ArchetypalListener>(
    this: &mut T, symname: &str
  ) {
  hook_exported_by_modname(this, "", symname)
}

pub fn hook_exported_by_modname<T: ArchetypalListener>(
    this: &mut T, modname: &str, symname: &str
  ) {
  unsafe { gum_init_embedded(); }
  let interceptor = unsafe { gum_interceptor_obtain() };
  let al = unsafe { get_new_archetypal_listener() };
  unsafe {
    ArchetypalListener_fill(al,
      gum::on_enter::<T> as *const c_void,
      gum::on_leave::<T> as *const c_void,
      this.ptr()
      //this as *mut T as *mut c_void
    );
  }
  let modstr = std::ffi::CString::new(modname).unwrap();
  let modptr = match modname {
    "" => std::ptr::null(),
    _  => modstr.as_ptr()
  };
  let symstr = std::ffi::CString::new(symname).unwrap();
  let funcptr = unsafe {
    gum_module_find_export_by_name(
      modptr,
      symstr.as_ptr()
    )
  };
  unsafe { setup_hook(interceptor, al, funcptr); }
}

/*
pub fn test() {
  println!("gumshoe test");
}

#[cfg(test)]
mod tests {
    use test;

    #[test]
    fn it_works() {
      test()
    }
}
*/
