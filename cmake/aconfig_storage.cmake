set(aconfig_tool_dir "${CMAKE_SOURCE_DIR}/src/build/tools/aconfig")
set(aconfig_tool "${CMAKE_SOURCE_DIR}/prebuilt/x86_64/aconfig")

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(aconfig_cargo_profile "debug")
    set(aconfig_cargo_profile_args "")
    set(aconfig_storage_cxxflags "-g")
    set(aconfig_storage_rustflags "-C debuginfo=2")
else()
    set(aconfig_cargo_profile "release")
    set(aconfig_cargo_profile_args "--release")
    set(aconfig_storage_cxxflags "-Os -ffunction-sections -fdata-sections")
    set(aconfig_storage_rustflags "-C opt-level=s")
endif()

if(NOT CMAKE_SYSTEM_NAME STREQUAL "Android")
    set(aconfig_storage_cxxflags "-stdlib=libc++ ${aconfig_storage_cxxflags}")
    set(aconfig_cargo_target_args "")
    set(aconfig_tool_target_dir "${aconfig_tool_dir}/target/${aconfig_cargo_profile}")
    set(aconfig_cxxbridge_dir "${aconfig_tool_dir}/target/cxxbridge")
    set(aconfig_cargo_target_env "CXXFLAGS=${aconfig_storage_cxxflags}")
else()
    set(aconfig_android_triple "aarch64-linux-android")
    string(REGEX REPLACE "^android-" "" aconfig_android_api_level "${ANDROID_PLATFORM}")
    set(aconfig_cargo_target_args "--target" "${aconfig_android_triple}")
    set(aconfig_tool_target_dir "${aconfig_tool_dir}/target/${aconfig_android_triple}/${aconfig_cargo_profile}")
    set(aconfig_cxxbridge_dir "${aconfig_tool_dir}/target/${aconfig_android_triple}/cxxbridge")
    set(aconfig_cargo_target_env
            "CFLAGS_${aconfig_android_triple}=--target=${aconfig_android_triple}${aconfig_android_api_level}"
            "CXXFLAGS_${aconfig_android_triple}=--target=${aconfig_android_triple}${aconfig_android_api_level}")
endif()

set(aconfig_cargo_env
        "RUSTFLAGS=${aconfig_storage_rustflags}"
        "CXX=${CMAKE_CXX_COMPILER}"
        ${aconfig_cargo_target_env}
)

find_program(cargo_executable "cargo")
if(NOT cargo_executable)
    message(FATAL_ERROR
        "cargo is required to build the aconfig_storage staticlib, please
        install rust first: './build.sh' sets it up")
endif()

# the staticlib and the cxxbridge header are rebuilt on every configure, no
# existence check and no up to date check: a stale artifact is never reused
message(STATUS "Building the aconfig_storage_read_api rust staticlib (${CMAKE_BUILD_TYPE}) ...")
execute_process(
    COMMAND ${CMAKE_COMMAND} -E env ${aconfig_cargo_env}
            ${cargo_executable} rustc -p aconfig_storage_read_api --crate-type staticlib
            ${aconfig_cargo_profile_args} ${aconfig_cargo_target_args}
    WORKING_DIRECTORY ${aconfig_tool_dir}
)

set(aconfig_storage_read_api_crate_dir "${aconfig_tool_dir}/aconfig_storage_read_api")
set(aconfig_storage_read_api_cxx_src_dir "${aconfig_cxxbridge_dir}/aconfig_storage_read_api/src")
file(MAKE_DIRECTORY "${aconfig_cxxbridge_dir}/aconfig_storage")
file(COPY_FILE "${aconfig_storage_read_api_cxx_src_dir}/lib.rs.h" "${aconfig_cxxbridge_dir}/aconfig_storage/lib.rs.h")

set(aconfig_storage_read_srcs
        "${aconfig_storage_read_api_crate_dir}/aconfig_storage_read_api.cpp"
        "${aconfig_storage_read_api_cxx_src_dir}/lib.rs.cc"
)

set(aconfig_storage_headers
        "${aconfig_cxxbridge_dir}"
        "${aconfig_cxxbridge_dir}/rust"
        "${aconfig_storage_read_api_crate_dir}/include"
)

add_library(aconfig_storage_read STATIC ${aconfig_storage_read_srcs})
# the ndk libc.a carries its own copy of the rust std (parts of bionic are
# written in rust), so rust_eh_personality is defined both there and in our
# staticlib: let the linker take the first definition
target_link_options(aconfig_storage_read INTERFACE "-Wl,--allow-multiple-definition")
target_include_directories(aconfig_storage_read PUBLIC
        ${aconfig_storage_headers}
        ${libbase_headers}
)
target_link_directories(aconfig_storage_read PUBLIC 
        "${aconfig_tool_target_dir}"
)
target_link_libraries(aconfig_storage_read PUBLIC
        aconfig_storage_read_api
        base
)
