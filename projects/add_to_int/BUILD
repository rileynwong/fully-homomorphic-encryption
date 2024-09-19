load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("//transpiler/rust:fhe_rs.bzl", "fhe_rust_library")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

licenses(["notice"])

fhe_rust_library(
    name = "add_to_int_rs_fhe_lib",
    lut_size = 0,
    src = "add_to_int.cc",
    hdrs = ["add_to_int.h"],
)

rust_library(
    name = "add_to_int_rs_lib",
    srcs = ["add_to_int_rs_lib.rs"],
    deps = [
        ":add_to_int_rs_fhe_lib",
        "@crate_index//:phantom-zone",
    ],
)

rust_binary(
    name = "add_to_int_rs_main",
    srcs = ["add_to_int_rs_lib.rs"],
    deps = [
        ":add_to_int_rs_fhe_lib",
        "@crate_index//:phantom-zone",
    ],
)

exports_files(
    [
        "add_to_int.h",
        "add_to_int.cc",
    ],
)
