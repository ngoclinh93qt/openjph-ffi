use std::env;
use std::path::Path;

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
    let mut cc1 = cc::Build::new();
    cc1.include(coredir);
    cc1.include(codestreamdir);
    cc1.include(codingdir);
    cc1.include(otherdir);
    cc1.include(transformdir);
    cc1.include(commondir);

    cc1.cpp(true);
    if (env::consts::OS == "macos") {
        cc1.cpp_set_stdlib("c++");
    } else {
        cc1.shared_flag(true);
        cc1.static_flag(true);
        cc1.cpp_link_stdlib("stdc++");
    }
    cc1.flag("-std=c++11");

    cc1.flag("-mavx");
    let file1s = [
        "transform/ojph_colour_avx.cpp",
        "transform/ojph_transform_avx.cpp",
    ];
    for file in file1s.iter() {
        cc1.file(coredir.join(file));
    }
    cc1.warnings(false);
    cc1.warnings_into_errors(false);
    cc1.compile("openjph2");

    let mut cc2 = cc::Build::new();
    cc2.include(coredir);
    cc2.include(codestreamdir);
    cc2.include(codingdir);
    cc2.include(otherdir);
    cc2.include(transformdir);
    cc2.include(commondir);

    cc2.cpp(true);
    if (env::consts::OS == "macos") {
        cc2.cpp_set_stdlib("c++");
    } else {
        cc2.shared_flag(true);
        cc2.static_flag(true);
        cc2.cpp_link_stdlib("stdc++");
    }
    cc2.flag("-std=c++11");

    cc2.flag("-mavx2");
    let file2s = [
        "transform/ojph_colour_avx2.cpp",
        "transform/ojph_transform_avx2.cpp",
    ];
    for file in file2s.iter() {
        cc2.file(coredir.join(file));
    }
    cc2.warnings(false);
    cc2.warnings_into_errors(false);
    cc2.compile("openjph3");

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
    //  cc.flag("-mavx");
    // cc.flag("-mavx2");
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
        // "transform/ojph_colour_avx.cpp",
        // "transform/ojph_colour_avx2.cpp",
        "transform/ojph_colour.cpp",
        "transform/ojph_colour_sse.cpp",
        "transform/ojph_colour_sse2.cpp",
        //  "transform/ojph_transform_avx.cpp",
        //  "transform/ojph_transform_avx2.cpp",
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
