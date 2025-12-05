
set(puffin_dir "${CMAKE_SOURCE_DIR}/src/puffin")

set(common_headers
    ${libchrome_dir}
    ${puffin_dir}
    ${libzucchini_headers}
    ${libbrotli_headers}
    ${protobuf_headers}
    ${absl_headers}
    "${puffin_dir}/src/include"
)

set(cflags
        "-DUSE_BRILLO=1"
        "-D_FILE_OFFSET_BITS=64"
        "-Wall"
        "-Wextra"
        "-Wimplicit-fallthrough"
)

set(puffin_protos_srcs
    "${puffin_dir}/src/puffin.pb.cc"
    "${puffin_dir}/src/puffin.pb.h"
)

set(libpuffdiff_srcs
        "${puffin_dir}/src/file_stream.cc"
        "${puffin_dir}/src/puffdiff.cc"
        "${puffin_dir}/src/utils.cc"
)

set(libpuffpatch_srcs
        ${puffin_protos_srcs}
        "${puffin_dir}/src/bit_reader.cc"
        "${puffin_dir}/src/bit_writer.cc"
        "${puffin_dir}/src/brotli_util.cc"
        "${puffin_dir}/src/huffer.cc"
        "${puffin_dir}/src/huffman_table.cc"
        "${puffin_dir}/src/memory_stream.cc"
        "${puffin_dir}/src/puff_reader.cc"
        "${puffin_dir}/src/puff_writer.cc"
        "${puffin_dir}/src/puffer.cc"
        "${puffin_dir}/src/puffin_stream.cc"
        "${puffin_dir}/src/puffpatch.cc"
)

add_custom_command(
    OUTPUT
        ${puffin_protos_srcs}
    COMMAND
        ${prebuilt_protoc} puffin.proto -I${puffin_dir}/src --cpp_out=${puffin_dir}/src
    DEPENDS
        ${puffin_dir}/src/puffin.proto
)
add_custom_target(gen_puffin_protos_srcs ALL DEPENDS ${puffin_protos_srcs})

add_library(puffpatch STATIC ${libpuffpatch_srcs})
target_compile_options(puffpatch PRIVATE ${cflags})
target_include_directories(puffpatch PUBLIC
    ${common_headers}
)
target_link_libraries(puffpatch PUBLIC
    bsdiff
    zucchini
    chrome
    brotli
)

add_library(puffdiff STATIC ${libpuffdiff_srcs})
target_compile_options(puffdiff PRIVATE ${cflags})
target_include_directories(puffdiff PUBLIC
    ${common_headers}
)
target_link_libraries(puffdiff PUBLIC
    bsdiff
    zucchini
    puffpatch
    chrome
    protobuf-cpp-lite
    brotli
    zlib
)