use std::env;
use std::path::Path;
fn main() {
    println!("cargo:rustc-link-lib=static=openjph");
    println!(r"cargo:rustc-link-search=lib\");

 }
 /*
fn main() {
    println!("HERELRLERLELREL");

    // uniffi_build::generate_scaffolding("./src/openjphffi.udl").unwrap();
    let mut cc = cc::Build::new();
    let coredir = Path::new("vendor/openjph/src/core");
    let codestreamdir = Path::new("vendor/openjph/src/core/codestream");
    let codingdir = Path::new("vendor/openjph/src/core/coding");
    let otherdir = Path::new("vendor/openjph/src/core/others");
    let transformdir = Path::new("vendor/openjph/src/core/transform");
    let commondir = Path::new("vendor/openjph/src/core/common");

    cc.include(coredir);
    cc.include(codestreamdir);
    cc.include(codingdir);
    cc.include(otherdir);
    cc.include(transformdir);
    cc.include(commondir);

    cc.cpp(true);
    println!("xxxxx {:?}", env::consts::OS);

    if (env::consts::OS == "macos") {
        cc.cpp_set_stdlib("c++");

    } else {
        cc.shared_flag(true);
        cc.static_flag(true);
        cc.cpp_link_stdlib("stdc++");

    }
    cc.flag("-std=c++11");
   // cc.flag("-DOJPH_DISABLE_INTEL_SIMD");
    cc.flag("-mavx");
    cc.flag("-mavx2");
   cc.flag("-fexceptions");
    cc.flag("-Wall");
    cc.flag("-Wextra");
    cc.flag("-Wconversion");
    cc.flag("-Wunused-parameter");

    let files = [
        "codestream/ojph_codestream.cpp",
        "codestream/ojph_params.cpp",
        "codestream/ojph_htj2kcompress.cpp",
        "codestream/ojph_htj2kdecompress.cpp",
        "coding/ojph_block_decoder.cpp",
        "coding/ojph_block_encoder.cpp",
        "others/ojph_arch.cpp",
        "others/ojph_file.cpp",
        "others/ojph_mem.cpp",
        "others/ojph_message.cpp",
        "transform/ojph_colour_avx.cpp",
        "transform/ojph_colour_avx2.cpp",
        "transform/ojph_colour.cpp",
        "transform/ojph_colour_sse.cpp",
        "transform/ojph_colour_sse2.cpp",
        "transform/ojph_transform_avx.cpp",
        "transform/ojph_transform_avx2.cpp",
        "transform/ojph_transform_sse.cpp",
        "transform/ojph_transform_sse2.cpp",
        "transform/ojph_transform.cpp",
    ];
    for file in files.iter() {
        cc.file(coredir.join(file));
    }
    cc.warnings(false);
    cc.warnings_into_errors(false);
    cc.compile("openjph");
}
*/