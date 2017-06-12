extern crate gumshoe;
use gumshoe::*;

extern crate libc;
use libc::*;

use std::fs::File;
use std::io::Read;
use std::io::Write;

struct TestListener { }

impl ArchetypalListener for TestListener {
  fn on_enter(&mut self, ic: gum::GumInvocationContext) {
    println!(
      "open(2) on_enter called with {:?}: ",
      ic.get_nth_argument_string(0)
    );
    ic.replace_nth_argument(0, std::ffi::CString::new("/etc/hostname").unwrap().into_raw())
  }

  fn on_leave(&mut self, ic: gum::GumInvocationContext) {
    println!("open(2) on_leave called!")
  }

  fn ptr(&mut self) -> *mut c_void {
    self as *mut TestListener as *mut c_void
  }
}

pub fn test_testhook() {
  println!("testhook test");
  hook_exported_by_name(&mut TestListener{}, "open");

  let mut data = String::new();
  let mut f = File::open("/etc/hosts").expect("Unable to open file");
  f.read_to_string(&mut data).expect("Unable to read string");
  println!("{}", data);
}

#[cfg(test)]
mod tests {
  use gumshoe;
  use std;
  use test_testhook;

  #[test]
  fn it_works() {
    gumshoe::test();
    test_testhook();
  }
}
