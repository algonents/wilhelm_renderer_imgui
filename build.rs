use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    // Build the C++ imgui_wrapper library using CMake
    let dst = cmake::Config::new("cpp")
        .build_target("imgui_wrapper")
        .static_crt(true)
        .build();

    // Add library search path
    println!(
        "cargo:rustc-link-search=native={}/build",
        dst.display()
    );

    // Link the static library
    println!("cargo:rustc-link-lib=static=imgui_wrapper");

    // Platform-specific linking
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=GL");
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("darwin") {
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=opengl32");
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=shell32");
    }

    // Rebuild if C++ sources change
    println!("cargo:rerun-if-changed=cpp/imgui_wrapper.cpp");
    println!("cargo:rerun-if-changed=cpp/imgui_wrapper.h");
    println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");
}
