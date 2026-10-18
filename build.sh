#!/bin/bash
LOCALDIR=$(pwd)

function usage() {
	echo "Usage: $0 [linux|android] [release|debug]"
	echo ""
	echo "Platforms:"
	echo "  linux    Build for the host Linux x86_64 (default)"
	echo "  android  Build for Android arm64-v8a (requires ndk/)"
	echo ""
	echo "Build types:"
	echo "  release  Release build with size/speed optimizations (default)"
	echo "  debug    Debug build with assertions and debug info"
	echo ""
	echo "Examples:"
	echo "  $0 linux release"
	echo "  $0 android release"
	exit 1
}

function install_deps() {
	if grep -qo "debian" /etc/os-release; then
		sudo apt install -y cmake gcc clang build-essential binutils nasm llvm lld libc++-dev libc++abi-dev ninja-build flex bison git unzip wget patch curl
	fi
}

function install_rust() {
	if ! command -v rustup >/dev/null 2>&1; then
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable
	fi
	if [[ -e "$HOME/.cargo/env" ]]; then
		source "$HOME/.cargo/env"
	fi
	rustup toolchain install stable --profile minimal
}

function download_ndk_toolchains() {
	local ndk_version="$1"
	local ondk_version="$2"

	if grep -qo "debian" /etc/os-release; then
		if [[ ! -e "ndk" ]] && [[ ! -e "ondk-$ondk_version-linux.tar.xz" ]] && [[ ! -e "android-ndk-$ndk_version-linux.zip" ]]; then
			wget https://github.com/topjohnwu/ondk/releases/download/$ondk_version/ondk-$ondk_version-linux.tar.xz
		fi
		if [[ ! -e "ndk" ]]; then
			if [[ -e "ondk-$ondk_version-linux.tar.xz" ]]; then
				tar -xf "ondk-$ondk_version-linux.tar.xz" -C "."
				mv "ondk-$ondk_version" "ndk"
			elif [[ -e "android-ndk-$ndk_version-linux.zip" ]]; then
				unzip -q -o "android-ndk-$ndk_version-linux.zip" -d "."
				mv "android-ndk-$ndk_version" "ndk"
			fi
		fi
	fi
}

function set_ndk_toolchains() {
	export PATH="$LOCALDIR/ndk/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
}

function setup_android_rust_toolchains() {
	local android_platform="$1"

	local ndk_dir="$LOCALDIR/ndk"
	local toolchain_bin="$ndk_dir/toolchains/llvm/prebuilt/linux-x86_64/bin"
	rustup target add aarch64-linux-android
	export ANDROID_NDK_HOME="$ndk_dir"
	export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$toolchain_bin/aarch64-linux-android${android_platform}-clang"
}

function builld_x86_64_aprotoc() {
	local build_dir="$1"
	local target="$2"
	local target_dir="$3"
	local cmake_gen_args="-DCMAKE_C_COMPILER=$cc -DCMAKE_CXX_COMPILER=$cxx"

	rm -rf "$build_dir"
	cmake $cmake_gen_args -G "Ninja" -B "$build_dir" -DCMAKE_BUILD_TYPE="Release"
	cmake --build "$build_dir" --target="$target" -j$(nproc --all)

	if [[ -e "$build_dir/aprotoc" ]]; then
		mkdir -p "$target_dir"
		cp "$build_dir/aprotoc" "$target_dir/aprotoc"
	fi
}

function build_x86_64_aconfig() {
	local aconfig_dir="$1"
	local target_dir="$2"

	rm -rf "$aconfig_dir/target"
	echo "Building x86_64 aconfig ..."
	if ! (cd "$aconfig_dir" && cargo build --release --package aconfig); then
		echo "failed to build x86_64 aconfig" >&2
		exit 1
	fi
	cd $LOCALDIR
	mkdir -p "$target_dir"
	cp "$aconfig_dir/target/release/aconfig" "$target_dir/aconfig"
}

function configure_and_build() {
	local cmake_gen_args="$1"
	local build_type="$2"
	local platform="$3"
	local build_dir="$4"

	rm -rf "$build_dir"
	echo "[$platform] cmake $cmake_gen_args -G Ninja -DCMAKE_BUILD_TYPE=$build_type"
	cmake $cmake_gen_args -G "Ninja" -B "$build_dir" -DCMAKE_INSTALL_PREFIX="$build_dir" -DCMAKE_BUILD_TYPE="$build_type"
	cmake --build "$build_dir" --target="install" -j$(nproc --all)
}

function build_linux() {
	local build_type="$1"
	local build_dir="$2"
	local cc="$3"
	local cxx="$4"
	local cmake_gen_args="-DCMAKE_C_COMPILER=$cc -DCMAKE_CXX_COMPILER=$cxx"

	configure_and_build "$cmake_gen_args" "$build_type" "linux" "$build_dir"
}

function build_android() {
	local build_type="$1"
	local build_dir="$2"
	local android_platform="$3"
	local cc="$4"
	local cxx="$5"
	local toolchain="$LOCALDIR/ndk/build/cmake/android.toolchain.cmake"

	if [[ ! -e "$toolchain" ]]; then
		echo "error: Android NDK toolchain not found at $toolchain" >&2
		echo "       Place the NDK at $LOCALDIR/ndk/ or enable set_ndk_toolchains in $0" >&2
		exit 1
	fi

	local cmake_gen_args="\
        -DCMAKE_C_COMPILER=$cc \
        -DCMAKE_CXX_COMPILER=$cxx \
        -DCMAKE_TOOLCHAIN_FILE=$toolchain \
        -DANDROID_ABI=arm64-v8a \
        -DANDROID_PLATFORM=android-${android_platform} \
        -DANDROID_STL=c++_static"

	setup_android_rust_toolchains "$android_platform"
	build_x86_64_aconfig "src/build/tools/aconfig" "prebuilt/x86_64"
	builld_x86_64_aprotoc "$build_dir" "aprotoc" "prebuilt/x86_64"
	configure_and_build "$cmake_gen_args" "$build_type" "android" "$build_dir"
}

function build() {
	local cc="clang"
	local cxx="clang++"
	local platform=""
	local build_type="Release"
	local android_platform=31

	for arg in "$@"; do
		case "$arg" in
			linux|Linux|LINUX)
				platform="linux"
				;;
			android|Android|ANDROID)
				platform="android"
				;;
			debug|Debug|DEBUG)
				build_type="Debug"
				;;
			release|Release|RELEASE)
				build_type="Release"
				;;
			esac
	done

	echo "Building for platform '$platform' with build type '$build_type'"
	case "$platform" in
		linux) build_linux "$build_type" "build" "$cc" "$cxx" ;;
		android) build_android "$build_type" "build" "$android_platform"  "$cc" "$cxx" ;;
	esac
}

if [[ "$#" -lt 1 ]] then
	usage
	exit 1
fi

install_deps
install_rust
download_ndk_toolchains "r30-beta2" "r30.1"
set_ndk_toolchains
build $@
