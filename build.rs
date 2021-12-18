/*
 * @Author       : sunzhifeng <ian.sun@auodigitech.com>
 * @Date         : 2021-08-25 19:19:53
 * @LastEditors  : ian <lauer3912@gmail.com>
 * @LastEditTime : 2021-12-18 23:00:35
 * @FilePath     : /iceoryx-rs/build.rs
 * @Description  : Created by sunzhifeng, Please coding something here
 */
// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::process::Command;

use cpp_build;
use shadow_rs;

fn make_and_install(source_dir: &str, build_dir: &str, install_dir: &str) -> std::io::Result<()> {
    let cmake_install_prefix = format!("-DCMAKE_INSTALL_PREFIX={}", install_dir);

    for iceoryx_component in &["iceoryx_utils", "iceoryx_posh", "iceoryx_binding_c"] {
        let component_source_dir = format!("{}/{}", source_dir, iceoryx_component);
        let component_build_dir = format!("{}/{}", build_dir, iceoryx_component);

        Command::new("mkdir")
            .args(&["-p", &component_build_dir])
            .output()
            .map_err(|out| {
                println!("{:?}", out);
                out
            })
            .map(|out| println!("{:?}", out))?;

        Command::new("cmake")
            .current_dir(&component_build_dir)
            .args(&[
                "-DCMAKE_BUILD_TYPE=Release",
                "-DBUILD_SHARED_LIBS=OFF",
                "-DBINDING_C=ON",
                &cmake_install_prefix,
                &component_source_dir,
            ])
            .output()
            .map_err(|out| {
                println!("{:?}", out);
                out
            })
            .map(|out| println!("{:?}", out))?;

        Command::new("cmake")
            .current_dir(&component_build_dir)
            .args(&["--build", ".", "--target", "install"])
            .output()
            .map_err(|out| {
                println!("{:?}", out);
                out
            })
            .map(|out| println!("{:?}", out))?;
    }

    Ok(())
}

fn main() -> shadow_rs::SdResult<()> {
    println!("启动自定义编译构建");
    println!("cargo:rerun-if-changed=build.rs");
    let current_dir = env::current_dir()?;
    let current_dir = current_dir.to_str().expect("Valid dir");

    let iceoryx_source_dir = format!("{}/{}", current_dir, "iceoryx");
    let iceoryx_build_dir = format!("{}/{}/{}", current_dir, "target", "iceoryx-build");
    let iceoryx_install_dir = format!("{}/{}/{}", current_dir, "target", "iceoryx-install");

    make_and_install(
        &iceoryx_source_dir,
        &iceoryx_build_dir,
        &iceoryx_install_dir,
    )?;

    let iceoryx_include_dir = format!("{}/{}", iceoryx_install_dir, "include");
    let iceoryx_lib_dir = format!("{}/{}", iceoryx_install_dir, "lib");
    cpp_build::Config::new()
        .include(iceoryx_include_dir)
        .flag("-Wno-noexcept-type")
        .flag("-std=c++14")
        .build("src/lib.rs");

    println!("cargo:rustc-link-search={}", iceoryx_lib_dir);

    println!("cargo:rustc-link-lib=iceoryx_posh_roudi");
    println!("cargo:rustc-link-lib=iceoryx_posh");
    println!("cargo:rustc-link-lib=iceoryx_utils");
    println!("cargo:rustc-link-lib=iceoryx_platform");
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib=stdc++");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    shadow_rs::new()
}
