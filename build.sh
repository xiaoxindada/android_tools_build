#!/bin/bash
LOCALDIR=$(pwd)

cc="clang"
cxx="clang++"
android_platform=31
ondk_version="r30.1"
ondk_dir="$LOCALDIR/ndk"

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
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain nightly
	fi

	if [[ -e "$HOME/.cargo/env" ]]; then
		source "$HOME/.cargo/env"
	fi

	rustup component add rust-src --toolchain nightly
	export RUSTUP_TOOLCHAIN="nightly"
}

function download_ondk_toolchains() {
	if [[ ! -e "$ondk_dir" ]] && [[ ! -e "ondk-$ondk_version-linux.tar.xz" ]]; then
		wget https://github.com/topjohnwu/ondk/releases/download/$ondk_version/ondk-$ondk_version-linux.tar.xz
	fi
}

function set_ondk_toolchains() {
	if [[ ! -e "$ondk_dir" ]]; then
		tar -xf "ondk-$ondk_version-linux.tar.xz" -C "."
		mv "ondk-$ondk_version" "$ondk_dir"
	fi
	# ondk bundles its own nightly rustc, but no prebuilt std
	rustup toolchain link ondk "$ondk_dir/toolchains/rust"
	export RUSTUP_TOOLCHAIN="ondk"
	export ANDROID_NDK_HOME="$ondk_dir"
	local toolchain_bin="$ondk_dir/toolchains/llvm/prebuilt/linux-x86_64/bin"
	export PATH="$ondk_dir/toolchains/llvm/prebuilt/linux-x86_64/bin:$ondk_dir/toolchains/rust/bin:$PATH"
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

function build_linux() {
	local build_type="$1"
	local build_dir="$2"
	local cmake_gen_args="-DCMAKE_C_COMPILER=$cc -DCMAKE_CXX_COMPILER=$cxx"

	build_x86_64_aconfig "src/build/tools/aconfig" "prebuilt/x86_64"
	configure_and_build "$cmake_gen_args" "$build_type" "linux" "$build_dir"
}

function build_android() {
	local build_type="$1"
	local build_dir="$2"

	local cmake_gen_args="\
        -DCMAKE_C_COMPILER=$cc \
        -DCMAKE_CXX_COMPILER=$cxx \
        -DCMAKE_TOOLCHAIN_FILE=$ondk_dir/build/cmake/android.toolchain.cmake \
        -DANDROID_ABI=arm64-v8a \
        -DANDROID_PLATFORM=android-${android_platform} \
        -DANDROID_STL=c++_static"

	build_x86_64_aconfig "src/build/tools/aconfig" "prebuilt/x86_64"
	builld_x86_64_aprotoc "$build_dir" "aprotoc" "prebuilt/x86_64"
	configure_and_build "$cmake_gen_args" "$build_type" "android" "$build_dir"
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

function build() {
	local platform=""
	local build_type="Release"

	for arg in "$@"; do
		case "$arg" in
		linux | Linux | LINUX)
			platform="linux"
			;;
		android | Android | ANDROID)
			platform="android"
			;;
		debug | Debug | DEBUG)
			build_type="Debug"
			;;
		release | Release | RELEASE)
			build_type="Release"
			;;
		esac
	done

	echo "Building for platform $platform with build type $build_type"
	case "$platform" in
	linux) build_linux "$build_type" "build" ;;
	android) build_android "$build_type" "build" ;;
	esac
}

if [[ "$#" -lt 1 ]]; then
	usage
	exit 1
fi

install_deps
install_rust
download_ondk_toolchains
set_ondk_toolchains
build $@
