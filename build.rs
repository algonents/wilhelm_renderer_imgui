use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    // GLFW headers come from wilhelm_renderer_sys (single source of truth for the
    // GLFW version we link against). This crate's ImGui GLFW backend is compiled
    // against these headers and calls into the libglfw3.a that sys links statically.
    let glfw_include = env::var("DEP_WILHELM_RENDERER_INCLUDE").expect(
        "wilhelm_renderer_sys did not publish DEP_WILHELM_RENDERER_INCLUDE — \
         require wilhelm_renderer_sys >= 0.10.1.",
    );

    // GL functions are resolved at runtime by ImGui's embedded GL3 loader;
    // EGL-only platforms whose Mesa ships no libGL (no GLX) set
    // GLRENDERER_LINK_GL=OFF to skip the explicit libGL link (same switch as
    // wilhelm_renderer_sys).
    println!("cargo:rerun-if-env-changed=GLRENDERER_LINK_GL");
    let link_gl = !env::var("GLRENDERER_LINK_GL").is_ok_and(|v| {
        matches!(v.to_ascii_lowercase().as_str(), "0" | "off" | "false" | "no")
    });

    // Build the C++ imgui_wrapper library using CMake
    let mut cmake_config = cmake::Config::new("cpp");
    cmake_config
        .build_target("imgui_wrapper")
        .define("GLFW_INCLUDE_DIR", &glfw_include)
        .static_crt(true);
    if !link_gl {
        cmake_config.define("GLRENDERER_LINK_GL", "OFF");
    }
    let dst = cmake_config.build();

    // Add library search path
    // On Windows with MSVC, CMake puts libraries in build/Debug or build/Release
    println!(
        "cargo:rustc-link-search=native={}/build",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/Debug",
        dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/build/Release",
        dst.display()
    );

    // Link the static library
    println!("cargo:rustc-link-lib=static=imgui_wrapper");

    // Platform-specific linking
    if target.contains("linux") {
        if link_gl {
            println!("cargo:rustc-link-lib=dylib=GL");
        }
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
