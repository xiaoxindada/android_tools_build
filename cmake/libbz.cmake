set(target_name "bz")

set(bzip2_dir "${CMAKE_SOURCE_DIR}/src/bzip2")

set(cflags
    "-DUSE_MMAP"
    "-Wno-unused-parameter"
)

set(libbz_srcs
    "${bzip2_dir}/blocksort.c"
    "${bzip2_dir}/bzlib.c"
    "${bzip2_dir}/compress.c"
    "${bzip2_dir}/crctable.c"
    "${bzip2_dir}/decompress.c"
    "${bzip2_dir}/huffman.c"
    "${bzip2_dir}/randtable.c"
)

add_library(${target_name} STATIC ${libbz_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${bzip2_dir}
)