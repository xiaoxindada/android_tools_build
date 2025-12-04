set(target_name "snapshot_cow")

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

add_library(${target_name} STATIC ${libsnapshot_cow_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${libsnapshot_cow_headers}
    ${libbase_headers}
    ${liblog_headers}
    ${libbrotli_headers}
    ${liblz4_headers}
    ${libzstd_headers}
    ${fs_mgr_dir}
    ${update_engine_headers}
    ${libselinux_headers}
    ${libsnapshot_srcs_dir}
    "${fs_mgr_dir}/libstorage_literals"
    "${fs_mgr_dir}/include"
    "${fs_mgr_dir}/libsnapshot/include"
    "${libsnapshot_cow_srcs_dir}/include"
    "${libsnapshot_cow_srcs_dir}/.."
)
target_link_libraries(snapshot_cow PUBLIC
        base
        log
        brotli
        zlib
        lz4
        zstd
)