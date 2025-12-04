set(target_name "avb")

set(libavb_dir "${CMAKE_SOURCE_DIR}/src/avb/libavb")

set(cflags
    "-D_FILE_OFFSET_BITS=64"
    "-D_POSIX_C_SOURCE=199309L"
    "-Wa,--noexecstack"
    "-Wall"
    "-Wextra"
    "-Wformat=2"
    "-Wmissing-prototypes"
    "-Wno-psabi"
    "-Wno-unused-parameter"
    "-Wno-format"
    "-ffunction-sections"
    "-fstack-protector-strong"
    "-g"
    "-DAVB_ENABLE_DEBUG"
    "-DAVB_COMPILATION"
)

set(ldflags
    "-Wl,--gc-sections"
    "-rdynamic"
)

set(libavb_sources
    "${libavb_dir}/avb_chain_partition_descriptor.c"
    "${libavb_dir}/avb_cmdline.c"
    "${libavb_dir}/avb_crc32.c"
    "${libavb_dir}/avb_crypto.c"
    "${libavb_dir}/avb_descriptor.c"
    "${libavb_dir}/avb_footer.c"
    "${libavb_dir}/avb_hash_descriptor.c"
    "${libavb_dir}/avb_hashtree_descriptor.c"
    "${libavb_dir}/avb_kernel_cmdline_descriptor.c"
    "${libavb_dir}/avb_property_descriptor.c"
    "${libavb_dir}/avb_rsa.c"
    "${libavb_dir}/avb_slot_verify.c"
    "${libavb_dir}/avb_util.c"
    "${libavb_dir}/avb_vbmeta_image.c"
    "${libavb_dir}/avb_version.c"
)

set(libavb_host_sysdeps_sources
    "${libavb_dir}/avb_sysdeps_posix.c"
)

set(avb_crypto_ops_impl_boringssl
    "${libavb_dir}/boringssl/sha.c"
)


    list(APPEND libavb_sources "${libavb_dir}/avb_sysdeps_posix.c")
    list(APPEND cflags "-fno-stack-protector")


add_library(avb_host_sysdeps STATIC ${libavb_host_sysdeps_sources})
target_compile_options(avb_host_sysdeps PRIVATE ${cflags})
target_link_options(avb_host_sysdeps PRIVATE ${ldflags})
target_include_directories(avb_host_sysdeps PUBLIC 
    ${libavb_dir}
)

add_library(${target_name} STATIC ${libavb_sources} ${avb_crypto_ops_impl_boringssl})
target_compile_options(${target_name} PRIVATE ${cflags})
target_link_options(${target_name} PRIVATE ${ldflags})
target_include_directories(${target_name} PUBLIC 
    ${boringssl_headers}
    ${libavb_dir}
    ${boringssl_headers}
    "${libavb_dir}/boringssl"
)
target_link_libraries(
    avb_host_sysdeps
    crypto
)