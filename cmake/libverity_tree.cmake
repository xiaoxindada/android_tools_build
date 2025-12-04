set(target_name "verity_tree")
set(libverity_tree_dir "${CMAKE_SOURCE_DIR}/src/extras/verity")

set(cflags
        "-D_FILE_OFFSET_BITS=64"
        "-Wall"
)

set(libverity_tree_srcs
        "${libverity_tree_dir}/build_verity_tree.cpp"
        "${libverity_tree_dir}/build_verity_tree_utils.cpp"
        "${libverity_tree_dir}/hash_tree_builder.cpp"
)


add_library(${target_name} STATIC ${libverity_tree_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${libverity_tree_dir}
    ${boringssl_headers}
    ${libbase_headers}
    ${libsparse_headers}
    ${libverity_tree_headers}
)
target_link_libraries(${target_name} PUBLIC
    sparse
    zlib
    crypto
    base
)
