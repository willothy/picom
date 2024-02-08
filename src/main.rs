#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// const OUT_DIR: &str = env!("CARGO_TARGET_DIR");

fn main() {
    // let bin = std::path::PathBuf::from(OUT_DIR).join("liblibpicom.a");
    //
    // cc::Build::new().file(bin).compile("picombin");

    // unsafe {
    //     ffi::picom_run(0, &mut [] as *mut [i8] as *mut *mut i8);
    // }
}
