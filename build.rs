use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use bindgen::CargoCallbacks;
use cc;
use pkg_config::{self, Config};

pub fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let src = root.join("src");

    let bindings_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindgen::builder()
        .header(src.join("wrapper.h").to_str().unwrap())
        .detect_include_paths(true)
        // .allowlist_function("picom_run")
        // .allowlist_function("get_lua_state")
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings_path)
        .expect("Failed to write bindings to file.");

    let dependencies = [
        // "LibGL",
        "gl",
        "egl",
        "luajit",   //
        "pixman-1", //
        "xcb",
        "xcb-image",
        "xcb-util",
        "xcb-renderutil",
        "xcb-randr",
        "xcb-composite",
        "xcb-damage",
        "xcb-dpms",
        "xcb-glx",
        "xcb-present",
        "xcb-xfixes",
        "xcb-render",
        "xcb-shape",
        "xcb-sync",
        "xext",
        "libevdev",
        "x11",
        "x11-xcb",
        "libconfig",
        "libpcre",
        "dbus-1",
    ];

    let dependencies = dependencies
        .into_iter()
        .map(|dep| {
            Config::new()
                // .statik(true)
                .probe(dep)
                .expect(&format!("Dependency {dep} not found"))
        })
        .collect::<Vec<_>>();

    let src_exclude = [
        src.join("xrescheck.c"),        //
        src.join("gl_common.c"),        //
        src.join("config_libconfig.c"), //
    ];

    let src_files = [
        "picom.c",
        "win.c",
        "c2.c",
        "x.c",
        "config.c",
        "vsync.c",
        "utils.c",
        "diagnostic.c",
        "string_utils.c",
        "render.c",
        "kernel.c",
        "log.c",
        "options.c",
        "event.c",
        "cache.c",
        "atom.c",
        "file_watch.c",
        "statistics.c",
        "vblank.c",
        "backend/backend_common.c",
        "backend/xrender/xrender.c",
        "backend/backend.c",
        "backend/driver.c",
        "backend/dummy/dummy.c",
        "backend/gl/glx.c",
        "backend/gl/gl_common.c",
        "backend/gl/shaders.c",
        "backend/gl/blur.c",
        "backend/gl/egl.c",
    ]
    .into_iter()
    .map(|f| src.join(f))
    .collect::<Vec<_>>();

    // let src_files = walkdir::WalkDir::new(&src)
    //     .into_iter()
    //     .filter_map(Result::ok)
    //     .filter_map(|f| f.path().is_file().then(|| f.path().to_path_buf()))
    //     .filter(|f| !src_exclude.iter().any(|x| x == f))
    //     .filter(|f| match f.extension() {
    //         Some(ext) => ext == "c",
    //         None => false,
    //     })
    //     .collect::<Vec<_>>();

    // println!("cargo:rustc-link-search=LibGL.so");
    // println!("cargo:rustc-link-lib=dylib=LibGL.so");

    // .includes(include_dirs.iter().map(|x| &x.libs).flatten())

    let mut build = cc::Build::new();

    for dep in dependencies {
        // panic!("Link file: {:?}", &dep);
        // let lib_path = format!(
        //     "{}/lib{}.so",
        //     dep.link_paths.last().unwrap().display(),
        //     dep.libs.last().unwrap()
        // );
        build.includes(dep.include_paths);
        build.include(dep.link_paths.last().unwrap());
        // println!(
        //     "cargo:rustc-link-search=all={}",
        //     dep.link_paths.last().unwrap().display()
        // );
        // for link_file in dep.link_files.iter() {
        //     // println!("cargo:rustc-link-lib={}", link_file.display());
        // }
        for lib in dep.libs {
            build.flag(&format!("-l{}", lib));
            println!("cargo:rustc-link-lib={}", lib);
        }
        for link_path in dep.link_paths {
            build.flag(&format!("-L{}", link_path.to_str().unwrap()));
            println!("cargo:rustc-link-search=all={}", link_path.display());
        }
        // for link_args in dep.ld_args {
        //     let list = link_args.join(",");
        //     build.flag(&format!("-Wl,{}", list));
        // }
    }

    build.include(root.join("subprojects/test.h/"));
    build.include(&src);
    // .include(src.join("backend"))

    build.files(src_files);

    build.pic(true);
    // build.static_flag(true);
    build.shared_flag(true);

    // build.define("PICOM_NO_MAIN", "1");
    build.define("PICOM_VERSION", "\"999\"");
    build.define("CONFIG_LIBCONFIG", "1");
    build.define("CONFIG_OPENGL", "1");
    build.define("GL_GLEXT_PROTOTYPES", "1");
    build.define("CONFIG_DBUS", "1");
    build.define("_GNU_SOURCE", "1");
    build.define("HAS_STDC_PREDEF_H", "1");
    build.define("NDEBUG", "1");

    build.cargo_metadata(false);
    build.compile("picom");

    let out_dir = env::var("OUT_DIR").unwrap();

    let out_file = format!("{}/libpicom.a", out_dir);

    // let executable = format!("{}/picom", out_dir);
    // let mut cmd = Command::new("ar");
    // cmd.arg("crs").arg(out_file).arg("picom.o");
    // cmd.current_dir(&out_dir);
    // cmd.status().expect("Failed to run ar");
    //
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    std::fs::copy(out_file, manifest_dir.join("picom.a")).unwrap();

    // let compiler = env::var("CC").unwrap_or("cc".to_string());
    let mut cmd = Command::new("gcc");
    cmd.arg(manifest_dir.join("picom.a"))
        // .arg("-C")
        .arg("-o")
        .arg(manifest_dir.join("picom"));
    cmd.status().expect("Failed to run cc");

    // println!("cargo:rustc-flags=-l libpicom -L {out_dir}");
    // println!("cargo:rustc-link-lib=static=libpicom");
}
