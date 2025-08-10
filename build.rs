use anyhow::Result;
use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;

fn main() -> Result<()> {
    if env::var("DOCS_RS").is_err() {
        let out_dir = PathBuf::from(env::var("OUT_DIR")?).canonicalize()?;
        let libczi_dir = out_dir.join("libczirw");
        let rep = if !libczi_dir.exists() {
            git2::Repository::clone("https://github.com/ZEISS/libczi.git", &libczi_dir)
                .expect("unable to clone libczirw")
        } else {
            git2::Repository::open(&libczi_dir)?
        };
        let (object, _) = rep.revparse_ext("494ac62f853de6ab86458f167fd85a03ee6d4f7e")?;
        rep.checkout_tree(&object, None)?;

        let dst = cmake::Config::new(&libczi_dir)
            .define("LIBCZI_BUILD_UNITTESTS", "OFF")
            .define("LIBCZI_BUILD_CZICMD", "OFF")
            .define("LIBCZI_BUILD_DYNLIB", "OFF")
            .define("LIBCZI_BUILD_PREFER_EXTERNALPACKAGE_EIGEN3", "OFF")
            .define("LIBCZI_BUILD_PREFER_EXTERNALPACKAGE_ZSTD", "OFF")
            .define("LIBCZI_BUILD_CURL_BASED_STREAM", "OFF")
            .define("LIBCZI_BUILD_PREFER_EXTERNAL_PACKAGE_LIBCURL", "OFF")
            .define("LIBCZI_BUILD_AZURESDK_BASED_STREAM", "OFF")
            .define("LIBCZI_BUILD_PREFER_EXTERNALPACKAGE_RAPIDJSON", "OFF")
            .define("LIBCZI_BUILD_LIBCZIAPI", "ON")
            .build();

        // let libcziapi_inc = libczi_dir.join("Src/libCZIAPI/inc");
        // let libczi_src = libczi_dir.join("Src/libCZI");
        // let libcziapi_src = libczi_dir.join("Src/libCZIAPI/src");
        // let libczi_h = libcziapi_inc.join("libCZIApi.h");

        let import_export = libczi_dir.join("Src/libCZIAPI/inc/importexport.h");
        {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&import_export)
                .expect("Could not open file");

            let mut data = String::new();
            file.read_to_string(&mut data).expect("Could not read file");
            let data = data.replace(" __declspec(dllexport)", "");
            let bytes = data.as_bytes();
            (&file).rewind().expect("Could not rewind");
            (&file).write_all(bytes).expect("Could not write file");
            file.set_len(bytes.len() as u64)
                .expect("Could not truncate");
        };

        // let bindings = bindgen::Builder::default()
        //     .clang_args([
        //         "-x",
        //         "c++",
        //         "-std=c++14",
        //         "-I",
        //         libcziapi_inc
        //             .to_str()
        //             .ok_or(Error::msg("cannot into string"))?,
        //         "-I",
        //         libcziapi_src
        //             .to_str()
        //             .ok_or(Error::msg("cannot into string"))?,
        //         "-I",
        //         libczi_src
        //             .to_str()
        //             .ok_or(Error::msg("cannot into string"))?,
        //     ])
        //     .header(libczi_h.to_str().ok_or(Error::msg("cannot into string"))?)
        //     .generate()
        //     .expect("Unable to generate bindings");
        //
        // bindings
        //     .write_to_file(out_dir.join("lib_czi_api.rs"))
        //     .expect("Couldn't write bindings!");

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("build/Src/libCZIAPI").display()
        );
        println!("cargo:rustc-link-lib=libCZIAPI");
    }
    println!("cargo::rerun-if-changed=build.rs");
    Ok(())
}
