fn main() {
    // Wrapper compilation
    cc::Build::new()
        .cpp(true)
        .cargo_metadata(false)
        .define("TF_LITE", None)
        .flag("-std=c++17")
        .flag("-fno-exceptions")
        .flag("-fno-rtti")
        .flag("-Os")
        .flag("-mthumb")
        .flag("-mcpu=cortex-m4")
        .flag("-mfpu=fpv4-sp-d16")
        .flag("-mfloat-abi=hard")
        .flag("-Wno-unused-parameter")
        .define("TF_LITE_STATIC_MEMORY", None)
        .include("lib/tflm_tree")
        .include("lib/tflm_tree/tensorflow")
        .include("lib/tflm_tree/tensorflow/lite")
        .include("lib/tflm_tree/tensorflow/lite/micro")
        .include("lib/tflm_tree/third_party/flatbuffers/include")
        .include("lib/tflm_tree/third_party/gemmlowp")
        .file("lib/tflm_tree/tflm_wrapper.cc")
        .compile("tflm_wrapper");

    println!(
        "cargo:rustc-link-search=native={}",
        std::env::var("OUT_DIR").unwrap()
    );
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=tensorflow-microlite");
    println!("cargo:rustc-link-lib=static=tflm_wrapper");
}
