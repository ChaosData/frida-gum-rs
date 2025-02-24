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

    pub fn link(config_gen: bool) {
      let target = std::env::var("TARGET").unwrap();

      let is_android: bool = target.ends_with("-android");

      if !std::path::Path::new(&format!("frida/{}/libfrida-gum.a", target)).exists() {
        println!("cargo:warning=frida/{}/libfrida-gum.a is missing", target);
        std::process::exit(1);
      }

      if config_gen {
        let _ = std::fs::remove_file(".cargo/config.toml");
      }

      let b64 = gumshoe::codeb64();
      let code = &decode(b64).unwrap();

      std::fs::create_dir_all("gen").unwrap();
      let mut f = File::create("gen/frida-gum-wrapper.c").unwrap();
      f.write_all(code.as_slice()).unwrap();

      println!("cargo:rustc-flags=-L frida/{}", target);
      println!("cargo:rustc-flags=-l frida-gum");
      println!("cargo:rustc-flags=-l dl");
      println!("cargo:rustc-flags=-l m");

      if !is_android {
        println!("cargo:rustc-flags=-l resolv");
        println!("cargo:rustc-flags=-l rt");
        println!("cargo:rustc-flags=-l pthread");
      }

      cc::Build::new()
        .include("frida")
        .file("gen/frida-gum-wrapper.c")
        .compile("frida-gum-wrapper");

      if config_gen {
        let cwd = std::env::current_dir().unwrap();
        let cwd = cwd.display();
        let out_dir = std::env::var("OUT_DIR").unwrap();

        let mut android_links = String::from("");

        if is_android {
          let triple: Vec<&str> = target.split("-").collect(); // e.g. aarch64-linux-android
          let clang_base = match std::env::var("ANDROID_NDK_HOME") {
            Err(_) => {
              if std::path::Path::new("/android-ndk").is_dir() {
                //cross container
                let entry_r = std::fs::read_dir("/android-ndk/lib64/clang").unwrap().next();
                entry_r.unwrap().unwrap().path().to_string_lossy().to_string()
              } else {
                panic!("ANDROID_NDK_HOME not set, nor does /android-ndk exist");
              }
            },
            Ok(ndk_home_path) => {
              // assuming something like cargo-ndk for host-based build
              // e.g. $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/lib/clang/19/lib/linux
              let path = ndk_home_path + "/toolchains/llvm/prebuilt/linux-x86_64/lib/clang";
              let clang_dir = std::path::Path::new(path.as_str());
              if clang_dir.is_dir() {
                let entry_r2 = std::fs::read_dir(path.as_str()).unwrap().next();
                entry_r2.unwrap().unwrap().path().to_string_lossy().to_string()
              } else {
                panic!("currently only supports building from x86_64 linux when using ANDROID_NDK_HOME");
              }
            }
          };
          android_links.push_str(format!("-L {}/lib/linux -l clang_rt.builtins-{}-android", clang_base, triple[0]).as_str());
        }

        let config = format!("\
          [target.{}]\n\
          rustflags = [\"-C\", \"relocation-model=pic\",
             \"-C\", \"link-args=-Wl,-Bstatic \
                       {} \
                       -L {}/frida/{} -lfrida-gum \
                       -L {} -l frida-gum-wrapper\"]\n",
          target, android_links, cwd, target, out_dir
        );

        let mut f = File::create(".cargo/config.toml").unwrap();
        f.write_all(config.as_str().as_bytes()).unwrap();
      }
    }
  }
}

pub trait ArchetypalListener {
  fn on_enter(&mut self, _ic: gum::GumInvocationContext);
  fn on_leave(&mut self, _ic: gum::GumInvocationContext);
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
