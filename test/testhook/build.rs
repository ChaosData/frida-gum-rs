//extern crate gcc;
extern crate gumshoe;

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
  gumshoe::link();
}
