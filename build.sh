#!/bin/bash
LOCALDIR=$(pwd)

cc="clang"
cxx="clang++"
android_platform="31"

function install_deps() {
	if grep -qo "debian" /etc/os-release; then
		sudo apt install -y cmake gcc clang build-essential binutils nasm llvm lld libc++-dev libc++abi-dev ninja-build flex bison git unzip wget patch curl
	fi
}

function install_rust() {
	# rust is needed for the x86_64 aconfig tool and for the
	# aconfig_storage_bridge staticlib, set it up here so cmake only has to
	# call cargo
	if ! command -v rustup >/dev/null 2>&1; then
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable
	fi
	if [[ -e "$HOME/.cargo/env" ]]; then
		source "$HOME/.cargo/env"
	fi
	rustup toolchain install stable --profile minimal
}

function set_toolchains() {
	local ndk_version="r30-beta2"
	local ondk_version="r30.1"

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
		export PATH="$LOCALDIR/ndk/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
	fi
}

function setup_android_rust() {
	local ndk_dir="$LOCALDIR/ndk"
	local toolchain_bin="$ndk_dir/toolchains/llvm/prebuilt/linux-x86_64/bin"
	rustup target add aarch64-linux-android
	export ANDROID_NDK_HOME="$ndk_dir"
	export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$toolchain_bin/aarch64-linux-android${android_platform}-clang"
}

function builld_x86_64_aprotoc() {
	local target="aprotoc"
	local target_dir="prebuilt/x86_64"
	local cmake_gen_args="-DCMAKE_C_COMPILER=$cc -DCMAKE_CXX_COMPILER=$cxx"

	rm -rf "build"
	cmake $cmake_gen_args -G "Ninja" -B "build" -DCMAKE_BUILD_TYPE="Release"
	cmake --build "build" --target="$target" -j$(nproc --all)

	if [[ -e "build/aprotoc" ]]; then
		mkdir -p "$target_dir"
		cp "build/aprotoc" "$target_dir/aprotoc"
	fi
}

function build_x86_64_aconfig() {
	local aconfig_dir="src/build/tools/aconfig"
	local target_dir="prebuilt/x86_64"

	# aconfig only parses .aconfig files and writes C++, it never has to be
	# cross compiled: keep the x86_64 linux binary next to aprotoc
	echo "Building x86_64 aconfig ..."
	if ! ( cd "$aconfig_dir" && cargo build --release --package aconfig ); then
		echo "failed to build x86_64 aconfig" >&2
		exit 1
	fi
	cd $LOCALDIR
	mkdir -p "$target_dir"
	cp "$aconfig_dir/target/release/aconfig" "$target_dir/aconfig"
}

function build() {
	local cmake_gen_args=
	local target=
	local build_type="Release"

	# ./build.sh [android] [release|debug]: the build type selects the cmake
	# build type and with it the cargo profile used by the aconfig modules
	for arg in "$@"; do
		case "$arg" in
			debug|Debug|DEBUG) build_type="Debug" ;;
			release|Release|RELEASE) build_type="Release" ;;
		esac
	done

	if [[ $1 == "android" ]]; then
		cmake_gen_args="\
        -DCMAKE_C_COMPILER=$cc \
        -DCMAKE_CXX_COMPILER=$cxx \
        -DCMAKE_TOOLCHAIN_FILE=$LOCALDIR/ndk/build/cmake/android.toolchain.cmake \
        -DANDROID_ABI=arm64-v8a \
        -DANDROID_PLATFORM=android-${android_platform} \
        -DANDROID_STL=c++_static"

		setup_android_rust
	elif [[ $(uname) == "Linux" ]]; then
		cmake_gen_args="-DCMAKE_C_COMPILER=$cc -DCMAKE_CXX_COMPILER=$cxx"
	fi

	if [[ $1 == "android" ]]; then
		builld_x86_64_aprotoc
	fi
	build_x86_64_aconfig
	rm -rf "build"
	echo "cmake $cmake_gen_args -G Ninja -DCMAKE_BUILD_TYPE=$build_type"
	cmake $cmake_gen_args -G "Ninja" -B "build" -DCMAKE_INSTALL_PREFIX="build" -DCMAKE_BUILD_TYPE="$build_type"
	cmake --build "build" --target="install" -j$(nproc --all)
}

install_deps
install_rust
set_toolchains
build $@
