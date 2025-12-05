use std::env;
use std::path::PathBuf;

fn main() {
    // Przebuduj gdy wrapper/TFLM się zmienia
    println!("cargo:rerun-if-changed=lib/tflm_tree/tflm_wrapper.cc");
    println!("cargo:rerun-if-changed=lib/tflm_tree");
    println!("cargo:rerun-if-changed=lib");

    let mut build = cc::Build::new();

    build
        .cpp(true)
        .compiler("arm-none-eabi-g++")
        .cargo_metadata(false)
        .define("TF_LITE", None)
        .define("TF_LITE_STATIC_MEMORY", None)
        .flag("-std=c++17")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-Os")
        .flag("-mthumb")
        .flag("-mcpu=cortex-m4")
        .flag("-mfpu=fpv4-sp-d16")
        .flag("-mfloat-abi=hard")
        .flag("-Wno-unused-parameter")
        .include("lib/tflm_tree")
        .include("lib/tflm_tree/tensorflow")
        .include("lib/tflm_tree/tensorflow/lite")
        .include("lib/tflm_tree/tensorflow/lite/micro")
        .include("lib/tflm_tree/third_party/flatbuffers/include")
        .include("lib/tflm_tree/third_party/gemmlowp")
        .file("lib/tflm_tree/tflm_wrapper.cc");

    build.compile("tflm_wrapper");

    // Ścieżki i biblioteki do linkowania
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=tensorflow-microlite");
    println!("cargo:rustc-link-lib=static=tflm_wrapper");

    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     std::env::var("OUT_DIR").unwrap()
    // );
    // println!("cargo:rustc-link-search=native=lib");
    // println!("cargo:rustc-link-lib=static=tensorflow-microlite");
    // println!("cargo:rustc-link-lib=static=tflm_wrapper");
}
