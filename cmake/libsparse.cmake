set(target_name "sparse")
set(libsparse_dir "${CMAKE_SOURCE_DIR}/src/core/libsparse")

set(cflags
    "-Wall"
)

set(libsparse_srcs
        "${libsparse_dir}/backed_block.cpp"
        "${libsparse_dir}/output_file.cpp"
        "${libsparse_dir}/sparse.cpp"
        "${libsparse_dir}/sparse_crc32.cpp"
        "${libsparse_dir}/sparse_err.cpp"
        "${libsparse_dir}/sparse_read.cpp"
)


add_library(${target_name} STATIC ${libsparse_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${libbase_headers}
    ${libsparse_headers}
)
target_link_libraries(${target_name} PUBLIC
    zlib
    base
)