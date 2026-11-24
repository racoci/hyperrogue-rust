//! Build script compiling the original HyperRogue C++ repository into a static library
//! for direct function-by-function FFI comparison and performance benchmarking.
//! Never writes or modifies any file inside ../hyperrogue.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=cpp_bridge/bridge.cpp");
    println!("cargo:rerun-if-env-changed=HYPERROGUE_ORIG_DIR");
    println!("cargo:rustc-check-cfg=cfg(has_orig_cpp)");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let orig_repo = env::var("HYPERROGUE_ORIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| manifest_dir.join("../hyperrogue"));
    if !orig_repo.exists() {
        println!(
            "cargo:warning=../hyperrogue not found; cloning https://github.com/zenorogue/hyperrogue/ into {}",
            orig_repo.display()
        );
        let clone_status = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "https://github.com/zenorogue/hyperrogue/",
                orig_repo.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to execute git clone for ../hyperrogue");
        assert!(
            clone_status.success(),
            "git clone https://github.com/zenorogue/hyperrogue/ into {} failed",
            orig_repo.display()
        );
    }
    if !orig_repo.join("hyper.cpp").exists() {
        return;
    }
    println!("cargo:rustc-cfg=has_orig_cpp");
    let cache_dir = manifest_dir.join("target/orig_cache");
    fs::create_dir_all(&cache_dir).expect("Failed to create target/orig_cache");

    // sysconfig.h:598 includes <GL/glu.h> even though HyperRogue calls zero glu* symbols.
    // Provide an empty fallback stub if libglu1-mesa-dev is absent on the host.
    if !Path::new("/usr/include/GL/glu.h").exists() {
        let gl_dir = cache_dir.join("GL");
        fs::create_dir_all(&gl_dir).unwrap();
        let glu_stub = gl_dir.join("glu.h");
        if !glu_stub.exists() {
            fs::write(&glu_stub, "/* stub for sysconfig.h:598 */\n").unwrap();
        }
    }

    let autohdr_path = cache_dir.join("autohdr.h");
    let langdata_path = cache_dir.join("language-data.cpp");
    let base_obj_path = cache_dir.join("hyper_orig_base.o");

    // 1. Build autohdr.h, language-data.cpp, and hyper_orig_base.o if not cached yet
    if !autohdr_path.exists() || !langdata_path.exists() || !base_obj_path.exists() {
        build_orig_base(&orig_repo, &cache_dir);
    }

    // 2. Compile thin FFI bridge.cpp (-O3) against autohdr.h + original headers
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bridge_obj = out_dir.join("bridge.o");
    let lib_archive = out_dir.join("libhyper_orig_bridge.a");

    let status = Command::new("g++")
        .args([
            "-std=c++14",
            "-O3",
            "-DFHS",
            "-DLINUX",
            "-I",
            cache_dir.to_str().unwrap(),
            "-I",
            orig_repo.to_str().unwrap(),
            "-I/usr/include/SDL2",
            "-fPIC",
            "-Wno-invalid-offsetof",
            "-Wno-unused-parameter",
            "-c",
            manifest_dir.join("cpp_bridge/bridge.cpp").to_str().unwrap(),
            "-o",
            bridge_obj.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute g++ for bridge.cpp");

    assert!(status.success(), "g++ failed compiling cpp_bridge/bridge.cpp");

    // 3. Archive hyper_orig_base.o + bridge.o into libhyper_orig_bridge.a
    let _ = fs::remove_file(&lib_archive);
    let ar_status = Command::new("ar")
        .args([
            "rcs",
            lib_archive.to_str().unwrap(),
            bridge_obj.to_str().unwrap(),
            base_obj_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to run ar");

    assert!(ar_status.success(), "ar failed creating libhyper_orig_bridge.a");

    // 4. Emit linker flags
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=hyper_orig_bridge");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=SDL2");
    println!("cargo:rustc-link-lib=dylib=GL");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=z");
}

fn build_orig_base(orig_repo: &Path, cache_dir: &Path) {
    let work_dir = cache_dir.join("shadow_src");
    fs::create_dir_all(&work_dir).unwrap();

    // Symlink all *.cpp, *.h, *.hpp from ../hyperrogue into shadow_src
    for entry in fs::read_dir(orig_repo).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if matches!(ext, "cpp" | "h" | "hpp") {
                let dst = work_dir.join(entry.file_name());
                let _ = fs::remove_file(&dst);
                std::os::unix::fs::symlink(&path, &dst).unwrap();
            }
        }
    }

    // Compile makeh & langen
    let makeh_bin = work_dir.join("makeh");
    let langen_bin = work_dir.join("langen");

    assert!(Command::new("g++")
        .args(["-O2", "makeh.cpp", "-o", makeh_bin.to_str().unwrap()])
        .current_dir(&work_dir)
        .status()
        .unwrap()
        .success());

    assert!(Command::new("g++")
        .args(["-O0", "-std=c++14", "-fPIC", "langen.cpp", "-o", langen_bin.to_str().unwrap()])
        .current_dir(&work_dir)
        .status()
        .unwrap()
        .success());

    assert!(Command::new(&langen_bin)
        .args(["-o", "language-data.cpp"])
        .current_dir(&work_dir)
        .status()
        .unwrap()
        .success());

    let makeh_cmd = "./makeh classes.cpp locations.cpp colors.cpp hyperpoint.cpp geometry.cpp embeddings.cpp goldberg.cpp init.cpp floorshapes.cpp cell.cpp multi.cpp shmup.cpp pattern2.cpp mapeditor.cpp graph.cpp textures.cpp hprint.cpp language.cpp util.cpp complex.cpp multigame.cpp backed-map.cpp arbitrile.cpp rulegen.cpp *.cpp > autohdr.h";
    assert!(Command::new("sh")
        .args(["-c", makeh_cmd])
        .current_dir(&work_dir)
        .status()
        .unwrap()
        .success());

    let base_src = work_dir.join("hyper_orig_base.cpp");
    fs::write(
        &base_src,
        r#"#define SDLVER 2
#define CAP_AUDIO 0
#define CAP_SDLTTF 0
#define CAP_SDLGFX 0
#define CAP_GLEW 0
#define CAP_PNG 0
#define CAP_TRANS 0
#define CAP_TABFONT 1
#define NOMAIN
#include <SDL2/SDL.h>
#ifndef KMOD_SCROLL
#define KMOD_SCROLL 0x0040
#endif
#if !SDL_VERSION_ATLEAST(2, 26, 0)
#define mouseX x
#define mouseY y
#endif
#include "hyper.cpp"
"#,
    )
    .unwrap();

    assert!(Command::new("g++")
        .args([
            "-std=c++14",
            "-O1",
            "-DFHS",
            "-DLINUX",
            "-I/usr/include/SDL2",
            "-I",
            cache_dir.to_str().unwrap(),
            "-I",
            orig_repo.to_str().unwrap(),
            "-fPIC",
            "-Wno-invalid-offsetof",
            "-Wno-unused-parameter",
            "-c",
            "hyper_orig_base.cpp",
            "-o",
            cache_dir.join("hyper_orig_base.o").to_str().unwrap(),
        ])
        .current_dir(&work_dir)
        .status()
        .unwrap()
        .success());

    fs::copy(work_dir.join("autohdr.h"), cache_dir.join("autohdr.h")).unwrap();
    fs::copy(work_dir.join("language-data.cpp"), cache_dir.join("language-data.cpp")).unwrap();
    let _ = fs::remove_dir_all(&work_dir);
}
