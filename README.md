# RustCC

A simple ANSI-C compliant C compiler written in Rust.

## LibC:

We bundle ziglibc and musl-libc by default.
While we promote ziglibc to inturn promote it's development, musl-libc is also available as an option.

## Build time dependencies:

1. cargo
1. tar
1. curl
1. xz

## Run time dependencies:

1. ld.lld
1. as
