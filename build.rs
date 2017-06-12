//extern crate gcc;
extern crate base64;
use base64::encode;

use std::fs::File;
use std::io::Read;
use std::io::Write;


fn persist() {
  let mut data = String::new();
  let mut f = File::open("src/frida-gum-wrapper.c").expect("Unable to open file");
  f.read_to_string(&mut data).expect("Unable to read string");

  let b64 = encode(data.as_bytes());
  let encap = format!("pub fn codeb64() -> &'static str {{\n  \"{}\"\n}}\n", b64);

  let out_dir = std::env::var("OUT_DIR").unwrap();
  let dest_path = std::path::Path::new(&out_dir).join("frida-gum-wrapper-code.rs");
  let mut f2 = File::create(&dest_path).unwrap();

  f2.write_all(encap.as_bytes()).unwrap();
}

fn main() {
/*
  println!("cargo:rustc-link-search=native=frida/{}",
           std::env::var("TARGET").unwrap()
  );
  println!("cargo:rustc-link-lib=static=frida-gum");

  gcc::Config::new()
    .include("frida")
    .file("src/frida-gum-wrapper.c")
    //.file(format!("frida/{}/libfrida-gum.a", std::env::var("TARGET").unwrap()))
    .compile("libfrida-gum-wrapper.a");
*/
  persist();
}

