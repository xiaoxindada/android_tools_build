// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::env;
use std::env::VarError;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// Platform-Specific Instructions: Regenerating gRPC Bindings for Windows

// If you need to manually update the 'android/grpc-bindings.rs' file on Windows,
// follow these steps:

// 1. LLVM Setup
//    - Download and install the pre-built LLVM binaries from:
//      https://releases.llvm.org/download.html
//    - Update the `LIBCLANG_PATH` environment variable to point to the 'bin'
//      directory within your LLVM installation. (e.g., `C:\Program Files\LLVM\bin`)

// 2. Environment Configuration
//    - Set the `GRPCIO_SYS_GRPC_INCLUDE_PATH` environment variable to the
//      location of your gRPC 'include' directory.

// 3. Building the Bindings
//    - Run the following Cargo command to regenerate the bindings:
//      `cargo build --features _gen-bindings`

include!("../link-deps.rs");

fn get_env(name: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={name}");
    match env::var(name) {
        Ok(s) => Some(s),
        Err(VarError::NotPresent) => None,
        Err(VarError::NotUnicode(s)) => {
            panic!("unrecognize env var of {name}: {:?}", s.to_string_lossy());
        }
    }
}

// Generate the bindings to grpc C-core.
// Try to disable the generation of platform-related bindings.
#[cfg(any(
    feature = "_gen-bindings",
    not(all(
        any(target_os = "linux", target_os = "macos"),
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))
))]
fn bindgen_grpc(file_path: &Path) {
    // create a config to generate binding file
    let mut config = bindgen::Builder::default();
    if cfg!(feature = "_secure") {
        config = config.clang_arg("-DGRPC_SYS_SECURE");
    }

    if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "windows") {
        config = config.clang_arg("-D _WIN32_WINNT=0x600");
    }

    // Search header files with API interface
    let mut headers = Vec::new();
    for result in WalkDir::new(Path::new("./grpc/include")) {
        let dent = result.expect("Error happened when search headers");
        if !dent.file_type().is_file() {
            continue;
        }
        let mut file = fs::File::open(dent.path()).expect("couldn't open headers");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Coundn't read header content");
        if buf.contains("GRPCAPI") || buf.contains("GPRAPI") {
            headers.push(String::from(dent.path().to_str().unwrap()));
        }
    }

    // To control the order of bindings
    headers.sort();
    for path in headers {
        config = config.header(path);
    }

    println!("cargo:rerun-if-env-changed=TEST_BIND");
    let gen_tests = env::var("TEST_BIND").map_or(false, |s| s == "1");

    let cfg = config
        .header("grpc_wrap.cc")
        .clang_arg("-xc++")
        .clang_arg("-I./grpc/include")
        .clang_arg("-std=c++11")
        .rustfmt_bindings(true)
        .impl_debug(true)
        .size_t_is_usize(true)
        .disable_header_comment()
        .allowlist_function(r"\bgrpc_.*")
        .allowlist_function(r"\bgpr_.*")
        .allowlist_function(r"\bgrpcwrap_.*")
        .allowlist_var(r"\bGRPC_.*")
        .allowlist_type(r"\bgrpc_.*")
        .allowlist_type(r"\bgpr_.*")
        .allowlist_type(r"\bgrpcwrap_.*")
        .allowlist_type(r"\bcensus_context.*")
        .allowlist_type(r"\bverify_peer_options.*")
        // Block all system headers.
        .blocklist_file(r"^/.*")
        .blocklist_function(r"\bgpr_mu_.*")
        .blocklist_function(r"\bgpr_cv_.*")
        .blocklist_function(r"\bgpr_once_.*")
        .blocklist_type(r"gpr_mu")
        .blocklist_type(r"gpr_cv")
        .blocklist_type(r"gpr_once")
        .constified_enum_module(r"grpc_status_code")
        .layout_tests(gen_tests)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        });
    println!("running {}", cfg.command_line_flags().join(" "));
    cfg.generate()
        .expect("Unable to generate grpc bindings")
        .write_to_file(file_path)
        .expect("Couldn't write bindings!");
}

// Determine if need to update bindings. Supported platforms do not
// need to be updated by default unless the _gen-bindings feature is specified.
fn config_binding_path() {
    let target = env::var("TARGET").unwrap();
    let file_path: PathBuf = match target.as_str() {
        "x86_64-unknown-linux-gnu"
        | "x86_64-unknown-linux-musl"
        | "aarch64-unknown-linux-musl"
        | "aarch64-unknown-linux-gnu"
        | "x86_64-apple-darwin"
        | "aarch64-apple-darwin" => {
            // Cargo treats nonexistent files changed, so we only emit the rerun-if-changed
            // directive when we expect the target-specific pre-generated binding file to be
            // present.
            println!("cargo:rerun-if-changed=bindings/bindings.rs");

            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("bindings")
                .join("bindings.rs")
        }
        _ => PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("android")
            .join("grpc-bindings.rs"),
    };
    #[cfg(feature = "_gen-bindings")]
    {
        // On some system (like Windows), stack size of main thread may
        // be too small.
        let f = file_path.clone();
        std::thread::Builder::new()
            .stack_size(8 * 1024 * 1024)
            .name("bindgen_grpc".to_string())
            .spawn(move || bindgen_grpc(&f))
            .unwrap()
            .join()
            .unwrap();
    }

    println!(
        "cargo:rustc-env=BINDING_PATH={}",
        file_path.to_str().unwrap()
    );
}

fn main() {
    println!("cargo:rerun-if-changed=grpc_wrap.cc");
    println!("cargo:rerun-if-changed=grpc");

    // create a builder to compile grpc_wrap.cc
    let mut cc = cc::Build::new();

    if get_env("CARGO_CFG_TARGET_OS").map_or(false, |s| s == "windows") {
        // At lease vista
        cc.define("_WIN32_WINNT", Some("0x600"));
    }

    let mut include_paths: Vec<PathBuf> = Vec::new();
    let include_path = get_env("GRPCIO_SYS_GRPC_INCLUDE_PATH");
    if include_path.is_none() {
        panic!("$GRPCIO_SYS_GRPC_INCLUDE_PATH is not set");
    }
    include_paths.push(include_path.unwrap().into());
    for inc_path in include_paths {
        cc.include(inc_path);
    }

    cc.cpp(true);
    if !cfg!(target_env = "msvc") {
        cc.flag("-std=c++11");
    }
    cc.file("grpc_wrap.cc");
    cc.warnings_into_errors(true);
    cc.compile("libgrpc_wrap.a");

    config_binding_path();
}
