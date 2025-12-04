set(target_name "bsdiff")

set(libbsdiff_dir "${CMAKE_SOURCE_DIR}/src/bsdiff")

set(cflags
        "-D_FILE_OFFSET_BITS=64"
        "-Wall"
        "-Wextra"
        "-Wno-unused-parameter"
)

set(libbsdiff_srcs
        "${libbsdiff_dir}/brotli_compressor.cc"
        "${libbsdiff_dir}/bsdiff.cc"
        "${libbsdiff_dir}/bz2_compressor.cc"
        "${libbsdiff_dir}/compressor_buffer.cc"
        "${libbsdiff_dir}/diff_encoder.cc"
        "${libbsdiff_dir}/endsley_patch_writer.cc"
        "${libbsdiff_dir}/logging.cc"
        "${libbsdiff_dir}/patch_writer.cc"
        "${libbsdiff_dir}/patch_writer_factory.cc"
        "${libbsdiff_dir}/split_patch_writer.cc"
        "${libbsdiff_dir}/suffix_array_index.cc"
)


set(libbspatch_srcs
        "${libbsdiff_dir}/brotli_decompressor.cc"
        "${libbsdiff_dir}/bspatch.cc"
        "${libbsdiff_dir}/bz2_decompressor.cc"
        "${libbsdiff_dir}/buffer_file.cc"
        "${libbsdiff_dir}/decompressor_interface.cc"
        "${libbsdiff_dir}/extents.cc"
        "${libbsdiff_dir}/extents_file.cc"
        "${libbsdiff_dir}/file.cc"
        "${libbsdiff_dir}/logging.cc"
        "${libbsdiff_dir}/memory_file.cc"
        "${libbsdiff_dir}/patch_reader.cc"
        "${libbsdiff_dir}/sink_file.cc"
        "${libbsdiff_dir}/utils.cc"
)

add_library(bspatch STATIC ${libbspatch_srcs})
target_compile_options(bspatch PRIVATE ${cflags})
target_include_directories(bspatch PUBLIC
    "${libbsdiff_dir}/.."
    "${libbsdiff_dir}/include"
    ${libdivsufsort_headers}
    ${libbrotli_headers}
    ${libbz_headers}
)
target_link_libraries(bspatch PUBLIC 
    bz
    brotli
)

add_library(${target_name} STATIC ${libbsdiff_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    "${libbsdiff_dir}/.."
    "${libbsdiff_dir}/include"
    ${libdivsufsort_headers}
    ${libbrotli_headers}
    ${libbz_headers}
)
target_link_libraries(${target_name} PUBLIC 
    bz
    brotli
    divsufsort64
    divsufsort
)