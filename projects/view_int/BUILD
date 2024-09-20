load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("//transpiler/rust:fhe_rs.bzl", "fhe_rust_library")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

licenses(["notice"])

fhe_rust_library(
    name = "view_int_rs_fhe_lib",
    lut_size = 0,
    src = "view_int.cc",
    hdrs = ["view_int.h"],
)

rust_library(
    name = "view_int_rs_lib",
    srcs = ["view_int_rs_lib.rs"],
    deps = [
        ":view_int_rs_fhe_lib",
        "@crate_index//:phantom-zone",
    ],
)

rust_binary(
    name = "view_int_rs_main",
    srcs = ["view_int_rs_lib.rs"],
    deps = [
        ":view_int_rs_fhe_lib",
        "@crate_index//:phantom-zone",
    ],
)

exports_files(
    [
        "view_int.h",
        "view_int.cc",
    ],
)
