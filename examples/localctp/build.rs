use std::env;
#[cfg(target_os = "macos")]
use std::fs;
use std::path::{Path, PathBuf};

const LIB_NAME: &str = "thosttraderapi_se_v6.7.2";
#[cfg(target_os = "macos")]
const LIB_FILENAME: &str = "libthosttraderapi_se_v6.7.2.dylib"; // adjust for non-mac if needed later
#[cfg(target_os = "linux")]
const LIB_FILENAME: &str = "libthosttraderapi_se_v6.7.2.so";

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir: PathBuf = Path::new(&manifest_dir).join("lib");
    let lib_file = lib_dir.join(LIB_FILENAME);

    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");
    println!("cargo:rerun-if-changed={}", lib_file.display());
    // Re-run if any new file added into lib dir (directory metadata change)
    println!("cargo:rerun-if-changed={}", lib_dir.display());

    if lib_file.exists() {
        let abs_lib_dir = lib_dir.canonicalize().unwrap_or(lib_dir.clone());
        println!(
            "cargo:warning=Found C++ library at: {}",
            abs_lib_dir.display()
        );

        // Search path for linker
        println!("cargo:rustc-link-search=native={}", abs_lib_dir.display());
        println!("cargo:rustc-link-lib=dylib={}", LIB_NAME);

        // pthread always
        println!("cargo:rustc-link-lib=pthread");

        // Linux rpath so the produced binary can find the .so without LD_LIBRARY_PATH.
        // Binary layout: target/debug/<bin>
        // Our library:   examples/localctp/lib/*.so relative to workspace root.
        // From executable: $ORIGIN/../../examples/localctp/lib
        #[cfg(target_os = "linux")]
        {
            println!("cargo:rustc-link-lib=stdc++");
            // Prefer $ORIGIN-based rpaths for relocatable binaries
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../examples/localctp/lib");
            // Fallback if user copies libs into target/debug/lib next to the binary
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../lib");
            // Also embed absolute path rpath for robustness
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", abs_lib_dir.display());
        }

        // macOS rpath so the produced binary can find the dylib without DYLD_LIBRARY_PATH.
        // Binary layout: target/debug/<bin>
        // Our library:   examples/localctp/lib/*.dylib relative to workspace root.
        // From executable: @executable_path/../examples/localctp/lib
        #[cfg(target_os = "macos")]
        {
            println!("cargo:rustc-link-lib=c++");

            // Correct relative path: binary at target/debug/<bin>, need to go up two levels to workspace root.
            // target/debug/<bin> -> ../.. == workspace root
            println!(
                "cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../../examples/localctp/lib"
            );
            // Also embed absolute path rpath for robustness (copy-paste execution elsewhere still resolves)
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", abs_lib_dir.display());
            // Fallback if user copies libs into target/debug/lib
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../lib");

            // The installed dylib's install_name likely refers to @rpath/libthosttraderapi_se.dylib (without version suffix).
            // Create a symlink without version suffix if missing to satisfy dyld lookup.
            let unsuffixed = abs_lib_dir.join("libthosttraderapi_se.dylib");
            if !unsuffixed.exists() {
                if let Err(e) = fs::remove_file(&unsuffixed) {
                    let _ = e;
                } // ignore if not exists
                if let Err(e) = std::os::unix::fs::symlink(&lib_file, &unsuffixed) {
                    println!(
                        "cargo:warning=Failed to create unsuffixed symlink {} -> {}: {}",
                        unsuffixed.display(),
                        lib_file.display(),
                        e
                    );
                } else {
                    println!(
                        "cargo:warning=Created symlink {} -> {}",
                        unsuffixed.display(),
                        lib_file.display()
                    );
                }
            }
        }

        println!(
            "cargo:warning=Linking to C++ library: {}",
            lib_file.display()
        );
    } else {
        println!(
            "cargo:warning=C++ library not found (expected: {}) — skipping native link",
            lib_file.display()
        );
        println!("cargo:warning=You may need to place the vendor dylib into that directory.");
    }
}
