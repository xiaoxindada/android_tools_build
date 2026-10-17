# libsmpte2094_50

This is not an officially supported Google product. This project is not
eligible for the [Google Open Source Software Vulnerability Rewards
Program](https://bughunters.google.com/open-source-security).

This project includes utilities to use the SMPTE 2094-50 specification.

## Build instructions

Just clone and use CMake to build. You need
[Cargo](https://doc.rust-lang.org/cargo/) on your path.

```sh
git clone https://github.com/webmproject/libsmpte2094-50.git
cmake -S libsmpte2094-50 -B libsmpte2094-50/build
cmake --build libsmpte2094-50/build --config Release --parallel
```

## Release instructions

Before doing a gitag and release, you need to upgrade the Cargo.toml:

```sh
cargo upgrade --incompatible
cargo publish
```
