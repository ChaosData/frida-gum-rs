extern crate base64;
use base64::decode;

use std::fs::File;
use std::io::Read;
use std::io::Write;

extern crate gcc;

extern crate libc;
use libc::*;

pub mod gum;

include!(concat!(env!("OUT_DIR"), "/frida-gum-wrapper-code.rs"));

pub fn link() {
  let b64 = codeb64();
  let code = &decode(b64).unwrap();

  std::fs::create_dir_all("gen").unwrap();
  let mut f = File::create("gen/frida-gum-wrapper.c").expect("Unable to create file");
  f.write_all(code.as_slice()).expect("Unable to write data");
  
  println!("cargo:rustc-link-search=native=frida/{}",
           std::env::var("TARGET").unwrap()
  );
  println!("cargo:rustc-link-lib=static=frida-gum");
  
  gcc::Config::new()
    .include("frida")
    .file("gen/frida-gum-wrapper.c")
    .compile("libfrida-gum-wrapper.a");
}

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
struct TestListener { }

impl ArchetypalListener for TestListener {
  fn on_enter(&mut self, ic: gum::GumInvocationContext) {
    println!(
      "open on_enter called with {:?}: ",
      ic.get_nth_argument_string(0)
    );
    ic.replace_nth_argument(0, std::ffi::CString::new("/etc/hostname").unwrap().into_raw())
  }

  fn on_leave(&mut self, ic: gum::GumInvocationContext) {
    println!("open on_leave called!")
  }

  fn ptr(&mut self) -> *mut c_void {
    self as *mut TestListener as *mut c_void
  }
}
*/
/*
struct Test2Listener { }

impl ArchetypalListener for Test2Listener {
  fn on_enter(&mut self, ic: gum::GumInvocationContext) {
    println!(
      "execve on_enter called with {:?}: ",
      ic.get_nth_argument_string(0)
    );
    //ic.replace_nth_argument(0, std::ffi::CString::new("/usr/bin/id").unwrap().into_raw())
  }

  fn on_leave(&mut self, ic: gum::GumInvocationContext) {
    println!("execve on_leave called!")
  }
}

struct Test3Listener { }

impl ArchetypalListener for Test3Listener {
  fn on_enter(&mut self, ic: gum::GumInvocationContext) {
    println!(
      "getuid on_enter"
    );
    //ic.replace_nth_argument(0, std::ffi::CString::new("/usr/bin/id").unwrap().into_raw())
  }

  fn on_leave(&mut self, ic: gum::GumInvocationContext) {
    println!("getuid on_leave called!");
    let mut ret = 42u32;
    ic.replace_return_value(ret as *mut c_void);
  }
}
*/


pub fn test() {
  println!("gumshoe test");
  /*
  let mut lis = TestListener{};
  gum::do_hook("open", &mut lis);
  let mut lis2 = Test2Listener{};
  gum::do_hook("execve", &mut lis2);
  let mut lis3 = Test3Listener{};
  gum::do_hook("getuid", &mut lis3);
  */
  //hook_exported_by_name(&mut TestListener{}, "open");
  /*
  let mut data = String::new();
  let mut f = File::open("/etc/hosts").expect("Unable to open file");
  f.read_to_string(&mut data).expect("Unable to read string");
  println!("{}", data);
  let output = std::process::Command::new("date").output().unwrap().stdout;
  println!("stdout: {}", String::from_utf8_lossy(&output));
  println!("{}", unsafe { getuid() });
  */
}

#[cfg(test)]
mod tests {
    use test;

    #[test]
    fn it_works() {
      test()
    }
}
