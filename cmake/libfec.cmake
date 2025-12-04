set(target_name "fec")

set(libfec_dir "${CMAKE_SOURCE_DIR}/src/extras/libfec")

set(cflags
    "-Wall"
    "-D_LARGEFILE64_SOURCE"
    "-DFEC_NO_KLOG"
)

set(libfec_srcs
    "${libfec_dir}/fec_open.cpp"
    "${libfec_dir}/fec_read.cpp"
    "${libfec_dir}/fec_verity.cpp"
    "${libfec_dir}/fec_process.cpp"
)

list(APPEND libfec_srcs "${libfec_dir}/avb_utils.cpp")

if(NOT CMAKE_SYSTEM_NAME STREQUAL "Android")
    list(APPEND cflags 
        "-D_GNU_SOURCE"
    )
endif()

add_library(${target_name} STATIC ${libfec_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${libbase_headers}
    ${libcutils_headers}
    ${libcrypto_utils_headers}
    ${libext4_utils_headers}
    ${libsquashfs_utils_headers}
    ${avb_headers}
    ${fec_headers}
    ${squashfs_headers}
    ${core_headers}
    ${libfec_headers}
)
target_link_libraries(${target_name} PUBLIC 
        base
        crypto
        crypto_utils
        cutils
        ext4_utils
        squashfs_utils
        fec_rs
        avb
)