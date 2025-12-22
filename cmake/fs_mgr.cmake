set(fs_mgr_dir "${CMAKE_SOURCE_DIR}/src/core/fs_mgr")
set(libsnapshot_cow_srcs_dir "${fs_mgr_dir}/libsnapshot/libsnapshot_cow")

set(cflags
        "-D_FILE_OFFSET_BITS=64"
        "-Wall"
        "-Wno-inconsistent-missing-override"
)

set(libsnapshot_cow_srcs
        "${libsnapshot_cow_srcs_dir}/cow_compress.cpp"
        "${libsnapshot_cow_srcs_dir}/cow_decompress.cpp"
        "${libsnapshot_cow_srcs_dir}/cow_format.cpp"
        "${libsnapshot_cow_srcs_dir}/cow_reader.cpp"
        "${libsnapshot_cow_srcs_dir}/parser_v2.cpp"
        "${libsnapshot_cow_srcs_dir}/parser_v3.cpp"
        "${libsnapshot_cow_srcs_dir}/snapshot_reader.cpp"
        "${libsnapshot_cow_srcs_dir}/writer_base.cpp"
        "${libsnapshot_cow_srcs_dir}/writer_v2.cpp"
        "${libsnapshot_cow_srcs_dir}/writer_v3.cpp"
)

add_library(snapshot_cow STATIC ${libsnapshot_cow_srcs})
target_compile_options(snapshot_cow PRIVATE ${cflags})
target_include_directories(snapshot_cow PUBLIC
    ${fs_mgr_headers}
    ${libbase_headers}
    ${liblog_headers}
    ${libbrotli_headers}
    ${liblz4_headers}
    ${libzstd_headers}
    ${update_engine_headers}
    ${libselinux_headers}
)
target_link_libraries(snapshot_cow PUBLIC
        base
        log
        brotli
        zlib
        lz4
        zstd
)

set(liblp_srcs_dir "${fs_mgr_dir}/liblp")


set(liblp_srcs
        "${liblp_srcs_dir}/builder.cpp"
        "${liblp_srcs_dir}/super_layout_builder.cpp"
        "${liblp_srcs_dir}/images.cpp"
        "${liblp_srcs_dir}/partition_opener.cpp"
        "${liblp_srcs_dir}/property_fetcher.cpp"
        "${liblp_srcs_dir}/reader.cpp"
        "${liblp_srcs_dir}/utility.cpp"
        "${liblp_srcs_dir}/writer.cpp"
)

add_library(lp STATIC ${liblp_srcs})
target_compile_options(lp PRIVATE ${cflags})
target_include_directories(lp PUBLIC 
    ${fs_mgr_headers}
    ${libbase_headers}
    ${libcutils_headers}
    ${boringssl_headers}
    ${liblog_headers}
    ${libsparse_headers}
    ${e2fsprogs_lib_headers}
    ${zlib_headers}
)
target_link_libraries(lp PUBLIC 
    crypto
    cutils
    base
    log
    crypto_utils
    sparse
    ext4_utils
    zlib
)