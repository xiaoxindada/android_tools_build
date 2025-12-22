#!/bin/bash
LOCALDIR=$(pwd)

cc="clang"
cxx="clang++"

function install_deps() {
	if grep -qo "debian" /etc/os-release; then
		sudo apt install -y cmake gcc clang build-essential binutils nasm llvm lld libc6-dev libc++-dev libc++abi-dev ninja-build flex bison git unzip wget patch
	fi
}

function set_toolchains() {
	local ndk_version="r29"
	local ondk_version="r29.3"

	if grep -qo "debian" /etc/os-release; then
		if [[ ! -e "ondk-$ondk_version-linux.tar.xz" ]] && [[ ! -e "android-ndk-$ndk_version-linux.zip" ]] && [[ ! -e "ndk" ]]; then
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

function builld_x86_64_aprotoc() {
	local target="aprotoc"
	local target_dir="prebuilt/x86_64"
	local cmake_gen_args="-DCMAKE_C_COMPILER=$cc -DCMAKE_CXX_COMPILER=$cxx -DMY_TARGET=\"$target\""

	rm -rf "build"
	cmake $cmake_gen_args -G "Ninja" -B "build" -DCMAKE_BUILD_TYPE="Release"
	cmake --build "build" --target="$target" -j$(nproc --all)

	if [[ -e "build/aprotoc" ]]; then
		mkdir -p "$target_dir"
		cp "build/aprotoc" "$target_dir/aprotoc"
	fi
}

function build() {
	local cmake_gen_args=
	local target=

	if [[ $1 == "android" ]]; then
		cmake_gen_args="\
        -DCMAKE_C_COMPILER=$cc \
        -DCMAKE_CXX_COMPILER=$cxx \
        -DCMAKE_TOOLCHAIN_FILE=$LOCALDIR/ndk/build/cmake/android.toolchain.cmake \
        -DANDROID_ABI=arm64-v8a \
        -DANDROID_PLATFORM=android-31 \
        -DANDROID_STL=c++_static"
	elif [[ $(uname) == "Linux" ]]; then
		cmake_gen_args="-DCMAKE_C_COMPILER=$cc -DCMAKE_CXX_COMPILER=$cxx"
	fi

	rm -rf "build"
	echo "cmake $cmake_gen_args -G Ninja"
	cmake $cmake_gen_args -G "Ninja" -B "build" -DCMAKE_INSTALL_PREFIX="build" -DCMAKE_BUILD_TYPE="Release"
	cmake --build "build" --target="install" -j$(nproc --all)
}

install_deps
set_toolchains
builld_x86_64_aprotoc
build $@
