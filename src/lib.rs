/*
 * @Author       : sunzhifeng <ian.sun@auodigitech.com>
 * @Date         : 2021-09-14 22:15:53
 * @LastEditors  : sunzhifeng <ian.sun@auodigitech.com>
 * @LastEditTime : 2021-12-10 22:18:50
 * @FilePath     : /iceoryx-rs/src/lib.rs
 * @Description  : Created by sunzhifeng, Please coding something here
 */
// SPDX-License-Identifier: Apache-2.0

#![recursion_limit="256"]

#[macro_use]
extern crate cpp;

mod error;
mod runtime;

pub mod introspection;
pub mod pb;
pub mod sb;

// re-export structs
pub use error::IceOryxError;
pub use runtime::Runtime;

use shadow_rs::shadow;
shadow!(build);

/**
 * 打印构建信息
 */
#[allow(dead_code)]
pub fn print_build_info() {
    // println!("clap_version:{}", build::clap_version());
    println!("pkg_version:{}", build::PKG_VERSION);
    // println!("pkg_version_major:{}", build::PKG_VERSION_MAJOR);
    // println!("pkg_version_minor:{}", build::PKG_VERSION_MINOR);
    // println!("pkg_version_patch:{}", build::PKG_VERSION_PATCH);
    // println!("pkg_version_pre:{}", build::PKG_VERSION_PRE);

    // println!("tag:{}", build::TAG);
    println!("branch:{}", build::BRANCH);
    // println!("commit_id:{}", build::COMMIT_HASH);
    println!("short_commit:{}", build::SHORT_COMMIT);
    println!("commit_date:{}", build::COMMIT_DATE);
    // println!("commit_date_2822:{}", build::COMMIT_DATE_2822);
    // println!("commit_date_3339:{}", build::COMMIT_DATE_3339);
    println!("commit_author:{}", build::COMMIT_AUTHOR);
    println!("commit_email:{}", build::COMMIT_EMAIL);

    println!("build_os:{}", build::BUILD_OS);
    println!("rust_version:{}", build::RUST_VERSION);
    println!("rust_channel:{}", build::RUST_CHANNEL);
    println!("cargo_version:{}", build::CARGO_VERSION);
    // println!("cargo_tree:{}", build::CARGO_TREE);

    println!("project_name:{}", build::PROJECT_NAME);
    println!("build_time:{}", build::BUILD_TIME);
    // println!("build_time_2822:{}", build::BUILD_TIME_2822);
    // println!("build_time_3339:{}", build::BUILD_TIME_3339);
    println!("build_rust_channel:{}", build::BUILD_RUST_CHANNEL);
}
