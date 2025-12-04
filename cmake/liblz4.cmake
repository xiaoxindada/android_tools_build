set(target_name "lz4")

set(liblz4_srcs_dir "${CMAKE_SOURCE_DIR}/src/lz4/lib")

set(liblz4_srcs
        "${liblz4_srcs_dir}/lz4.c"
        "${liblz4_srcs_dir}/lz4hc.c"
        "${liblz4_srcs_dir}/lz4frame.c"
        "${liblz4_srcs_dir}/xxhash.c"
)

set(cflags 
    "-Wall"
)

add_library(${target_name} STATIC ${liblz4_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC ${liblz4_headers})
