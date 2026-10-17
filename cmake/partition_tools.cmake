set(partition_tools_dir "${CMAKE_SOURCE_DIR}/src/system/extras/partition_tools")
set(fs_mgr_dir "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr")
set(libsnapshot_srcs_dir "${fs_mgr_dir}/libsnapshot")
set(update_engine_dir "${CMAKE_SOURCE_DIR}/src/system/update_engine")

set(cflags
    "-Wextra"
    "-D_FILE_OFFSET_BITS=64"
)

set(lpmake_srcs
    "${partition_tools_dir}/lpmake.cc"
)

set(lpunpack_srcs
    "${partition_tools_dir}/lpunpack.cc"
)

set(lpdump_srcs
    "${partition_tools_dir}/lpdump.cc"
    "${partition_tools_dir}/lpdump_host.cc"
    ${dynamic_partitions_device_info_srcs}
)

set(lpadd_srcs
    "${partition_tools_dir}/lpadd.cc"
)

set(lpflash_srcs
    "${partition_tools_dir}/lpflash.cc"
)

add_executable(lpmake ${lpmake_srcs})
target_compile_options(lpmake PRIVATE ${cflags})
target_include_directories(lpmake PRIVATE 
    ${libbase_headers}
    ${liblog_headers}
    ${liblp_headers}
)
target_link_libraries(lpmake PRIVATE
    base
    cutils_sockets
    log
    lp
)

add_executable(lpadd ${lpadd_srcs})
target_compile_options(lpadd PRIVATE ${cflags})
target_include_directories(lpadd PRIVATE 
    ${libbase_headers}
    ${liblog_headers}
    ${libsparse_headers}
    ${liblp_headers}
)
target_link_libraries(lpadd PRIVATE
    base
    cutils_sockets
    log
    lp
    sparse
)

add_executable(lpflash ${lpflash_srcs})
target_compile_options(lpflash PRIVATE ${cflags})
target_include_directories(lpflash PRIVATE 
    ${libbase_headers}
    ${liblog_headers}
    ${liblp_headers}
)
target_link_libraries(lpflash PRIVATE
    base
    cutils_sockets
    log
    lp
)

add_executable(lpunpack ${lpunpack_srcs})
target_compile_options(lpunpack PRIVATE ${cflags})
target_include_directories(lpunpack PRIVATE 
    ${libbase_headers}
    ${liblog_headers}
    ${libsparse_headers}
    ${liblp_headers}
)
target_link_libraries(lpunpack PRIVATE
    base
    cutils_sockets
    log
    lp
    sparse
)

add_executable(lpdump ${lpdump_srcs})
add_dependencies(lpdump gen_dynamic_partitions_device_info_srcs
        gen_snapshot_protos_srcs gen_update_metadata_srcs)
target_compile_options(lpdump PRIVATE ${cflags})
target_include_directories(lpdump PRIVATE
    ${libbase_headers}
    ${liblog_headers}
    ${liblp_headers}
    ${fs_mgr_headers}
    ${update_engine_dir}
    "${libsnapshot_srcs_dir}/android/snapshot"
)
target_link_libraries(lpdump PRIVATE
    base
    cutils_sockets
    log
    lp
    protobuf-cpp-full
    cutils
    # fs_mgr
    snapshot
)