extern crate bindgen;

use anyhow::{Error, Result};
use std::env;
use std::path::PathBuf;

#[cfg(not(feature = "dynamic"))]
use std::fmt::Debug;

#[cfg(not(feature = "dynamic"))]
use bindgen::callbacks::ItemInfo;

#[cfg(not(feature = "dynamic"))]
use std::collections::HashMap;

#[cfg(not(feature = "dynamic"))]
use regex::Regex;

fn main() -> Result<()> {
    if env::var("DOCS_RS").is_err() {
        let out_dir = PathBuf::from(env::var("OUT_DIR")?).canonicalize()?;
        let libczi_dir = PathBuf::from("libczi");
        let libczi_src = libczi_dir.join("Src/libCZI");
        let libcziapi_inc = libczi_dir.join("Src/libCZIAPI/inc");
        let libcziapi_src = libczi_dir.join("Src/libCZIAPI/src");
        let libcziapi_h = libcziapi_inc.join("libCZIApi.h");

        let dst = cmake::Config::new(&libczi_dir)
            .cxxflag("-fms-extensions")
            .define("LIBCZI_BUILD_UNITTESTS", "OFF")
            .define("LIBCZI_BUILD_CZICMD", "OFF")
            .define("LIBCZI_BUILD_DYNLIB", "OFF")
            .define("LIBCZI_BUILD_PREFER_EXTERNALPACKAGE_EIGEN3", "OFF")
            .define("LIBCZI_BUILD_PREFER_EXTERNALPACKAGE_ZSTD", "OFF")
            .define("LIBCZI_BUILD_CURL_BASED_STREAM", "OFF")
            .define("LIBCZI_BUILD_PREFER_EXTERNALPACKAGE_LIBCURL", "OFF")
            .define("LIBCZI_BUILD_AZURESDK_BASED_STREAM", "OFF")
            .define("LIBCZI_BUILD_PREFER_EXTERNALPACKAGE_RAPIDJSON", "OFF")
            .define("LIBCZI_BUILD_LIBCZIAPI", "ON")
            .build();

        #[cfg(not(feature = "dynamic"))]
        let bindings = {
            let mut libcziapi_a = out_dir.join("build/Src/libCZIAPI/liblibCZIAPIStatic.a");
            if !libcziapi_a.exists() {
                libcziapi_a = out_dir.join("build/Src/libCZIAPI/liblibCZIAPIStatic.lib");
            }
            bindgen::Builder::default().parse_callbacks(Box::new(DeMangler::new(libcziapi_a)?))
        };

        #[cfg(feature = "dynamic")]
        let bindings = bindgen::Builder::default();

        let bindings = bindings
            .merge_extern_blocks(true)
            .clang_args([
                "-fms-extensions",
                "-x",
                "c++",
                "-std=c++14",
                "-I",
                libcziapi_inc
                    .to_str()
                    .ok_or(Error::msg("cannot into string"))?,
                "-I",
                libcziapi_src
                    .to_str()
                    .ok_or(Error::msg("cannot into string"))?,
                "-I",
                libczi_src
                    .to_str()
                    .ok_or(Error::msg("cannot into string"))?,
            ])
            .header(
                libcziapi_h
                    .to_str()
                    .ok_or(Error::msg("cannot into string"))?,
            )
            .generate()?;

        bindings.write_to_file(out_dir.join("lib_czi_api.rs"))?;

        #[cfg(not(feature = "dynamic"))]
        {
            println!(
                "cargo:rustc-link-search=native={}",
                dst.join("build/Src/libCZIAPI").display()
            );
            println!("cargo:rustc-link-lib=static=libCZIAPIStatic");

            println!(
                "cargo:rustc-link-search=native={}",
                dst.join("build/Src/libCZI").display()
            );
            let profile = env::var("PROFILE")?;
            match profile.as_str() {
                "debug" => println!("cargo:rustc-link-lib=static=libCZIStaticd"),
                "release" => println!("cargo:rustc-link-lib=static=libCZIStatic"),
                _ => return Err(Error::msg(format!("unsupported profile: {}", profile))),
            }

            println!(
                "cargo:rustc-link-search=native={}",
                dst.join("lib").display()
            );
            println!(
                "cargo:rustc-link-search=native={}",
                dst.join("lib64").display()
            );
            println!("cargo:rustc-link-lib=static=zstd");
        }

        #[cfg(feature = "dynamic")]
        {
            println!(
                "cargo:rustc-link-search=native={}",
                dst.join("build/Src/libCZIAPI").display()
            );
            println!("cargo:rustc-link-lib=libCZIAPI");
        }
    }
    println!("cargo::rerun-if-changed=build.rs");
    Ok(())
}

#[cfg(not(feature = "dynamic"))]
#[derive(Debug)]
struct DeMangler {
    map: HashMap<String, String>,
}

#[cfg(not(feature = "dynamic"))]
impl DeMangler {
    fn new(a_file: PathBuf) -> Result<Self> {
        let cmd = std::process::Command::new("nm").arg(&a_file).output()?;
        let pat = Regex::new(r"^[\da-f]*\s[A-Z]\s(.*_Z(\d+)(libCZI_.*))$")?;
        let mut map = HashMap::new();
        for line in std::str::from_utf8(&cmd.stdout)?.lines() {
            if let Some(caps) = pat.captures(line.trim()) {
                if let (Some(symbol), Some(n), Some(name)) = (caps.get(1), caps.get(2), caps.get(3))
                {
                    let n: usize = n.as_str().parse()?;
                    let name = name.as_str();
                    let demangled = name[..n].to_string();
                    let mangled = symbol.as_str().to_string();
                    if let Some(existing_mangled) = map.get(&demangled) {
                        if existing_mangled != &mangled {
                            return Err(Error::msg(format!(
                                "conflicting mangled symbols for {} in {}: {}, {}",
                                demangled,
                                a_file.to_str().unwrap(),
                                existing_mangled,
                                mangled
                            )));
                        }
                    } else {
                        map.insert(demangled, mangled);
                    }
                }
            }
        }
        Ok(Self { map })
    }
}

#[cfg(not(feature = "dynamic"))]
impl bindgen::callbacks::ParseCallbacks for DeMangler {
    fn generated_link_name_override(&self, item_info: ItemInfo<'_>) -> Option<String> {
        self.map.get(item_info.name).cloned()
    }
}
