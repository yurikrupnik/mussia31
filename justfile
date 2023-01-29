#!/usr/bin/env just --justfile

bazel-build:
  bazel build //apps/go/go-app:BUILD.bazel


#release:
#  cargo build --release
#
#lint:
#  cargo clippy
#
#bin:
#  cargo run --bin bin -- arg1
#
#example:
#  cargo run --example exname -- arg1
