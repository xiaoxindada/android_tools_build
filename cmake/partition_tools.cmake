set(partition_tools_dir "${CMAKE_SOURCE_DIR}/src/extras/partition_tools")

set(cflags
    "-Wextra"
    "-D_FILE_OFFSET_BITS=64"
)

set(dynamic_partitions_device_info_srcs
    "${partition_tools_dir}/dynamic_partitions_device_info.pb.cc"
    "${partition_tools_dir}/dynamic_partitions_device_info.pb.h"
)

add_custom_command(
    OUTPUT
        ${dynamic_partitions_device_info_srcs}
    COMMAND
        ${prebuilt_protoc} dynamic_partitions_device_info.proto -I${partition_tools_dir} --cpp_out=${partition_tools_dir} 
    DEPENDS
        ${partition_tools_dir}/dynamic_partitions_device_info.proto
)
add_custom_target(gen_dynamic_partitions_device_info_srcs ALL DEPENDS ${dynamic_partitions_device_info_srcs})


set(lpmake_srcs
    "${partition_tools_dir}/lpmake.cc"
)

set(lpunpack_srcs
    "${partition_tools_dir}/lpunpack.cc"
)

set(lpdump_srcs
    "${partition_tools_dir}/lpdump.cc"
    ${dynamic_partitions_device_info_srcs}
)

if(NOT CMAKE_SYSTEM_NAME STREQUAL "Android")
    list(APPEND lpdump_srcs "${partition_tools_dir}/lpdump_host.cc")
else()
    list(APPEND lpdump_srcs "${partition_tools_dir}/lpdump_target.cc")
endif()

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

if (NOT CMAKE_SYSTEM_NAME STREQUAL "Android")
add_executable(lpdump ${lpdump_srcs})
target_compile_options(lpdump PRIVATE ${cflags})
target_include_directories(lpdump PRIVATE
    ${libbase_headers}
    ${liblog_headers}
    ${liblp_headers}
    ${libjsonpbparse_headers}
)
target_link_libraries(lpdump PRIVATE
    base
    cutils_sockets
    log
    lp
    protobuf-cpp-full
    jsonpbparse
)
endif()