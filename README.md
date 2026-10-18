### sync sources
> ./sync.sh  
> ./apply_patch.sh  
### build environment
> Use ubuntu or debian or wsl  
### build toolchains
> Use magisk [ondk](https://github.com/topjohnwu/ondk), it bundles the nightly rust toolchain used by the android build (Support cross compile x86_64 arm64-v8a)
### build usage
> ./build.sh [linux|android] [release|debug]  
> The arguments can be given in any order and the build type defaults to release  
### build for x86_64
> ./build.sh linux  
### build for android arm64-v8a
> ./build.sh android  
### out
> build/bin
### Contributors
- extract.erofs: [skkk](https://github.com/sekaiacg/erofs-utils)  
- e2fsextract: [affggh](https://github.com/CircleCashTeam/e2fsprogs_build)  
- [android aosp project](https://android.googlesource.com/)  
