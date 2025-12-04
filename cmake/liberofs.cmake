set(target_name "erofs")

set(eorfs_utils_dir "${CMAKE_SOURCE_DIR}/src/erofs-utils")

function(generate_version_header input_file output_file)
    if(NOT EXISTS ${input_file})
        message(FATAL_ERROR "Input file ${input_file} does not exist")
    endif()

    file(STRINGS ${input_file} FIRST_LINE LIMIT_COUNT 1)
    
    if(FIRST_LINE)
        string(STRIP "${FIRST_LINE}" CLEAN_VERSION)
        set(VERSION_DEFINE "#define PACKAGE_VERSION \"${CLEAN_VERSION}\"")
        file(WRITE ${output_file} "${VERSION_DEFINE}\n")
        message(STATUS "Generated ${output_file} with version: ${CLEAN_VERSION}")
    endif()
endfunction()

set(cflags
        "-Wall"
        "-Wno-error=#warnings"
        "-Wno-ignored-qualifiers"
        "-Wno-pointer-arith"
        "-Wno-unused-parameter"
        "-Wno-unused-function"
        "-DHAVE_FALLOCATE"
        "-DHAVE_LINUX_TYPES_H"
        "-DHAVE_LIBSELINUX"
        "-DHAVE_LIBUUID"
        "-DLZ4_ENABLED"
        "-DLZ4HC_ENABLED"
        "-DWITH_ANDROID"
        "-DHAVE_MEMRCHR"
        "-DHAVE_SYS_IOCTL_H"
        "-DHAVE_LLISTXATTR"
        "-DHAVE_LGETXATTR"
        "-D_FILE_OFFSET_BITS=64"
        "-DEROFS_MAX_BLOCK_SIZE=16384"
        "-DHAVE_UTIMENSAT"
        "-DHAVE_UNISTD_H"
        "-DHAVE_SYSCONF"
        "-DEROFS_MT_ENABLED"
)


file(GLOB liberofs_srcs "${eorfs_utils_dir}/lib/*.c")
list(REMOVE_ITEM liberofs_srcs
    "${eorfs_utils_dir}/lib/compressor_libdeflate.c"
    "${eorfs_utils_dir}/lib/compressor_libzstd.c"
)

add_library(${target_name} STATIC ${liberofs_srcs})
generate_version_header("${eorfs_utils_dir}/VERSION" "${eorfs_utils_dir}/erofs-utils-version.h")
target_compile_options(${target_name} PUBLIC ${cflags} "-include${eorfs_utils_dir}/erofs-utils-version.h")
target_include_directories(${target_name} PUBLIC
    ${libbase_headers}
    ${libcutils_headers}
    ${liblog_headers}
    ${liblz4_headers}
    ${libselinux_headers}
    ${e2fsprogs_lib_headers}
    ${liberofs_headers}
    "${e2fsprogs_lib_headers}/uuid"
)
target_link_libraries(${target_name} PUBLIC
    base
    cutils
    ext2_uuid
    log
    lz4
    selinux
)