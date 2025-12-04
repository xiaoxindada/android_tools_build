set(target_name "fec_rs")

set(libfec_rs_dir "${CMAKE_SOURCE_DIR}/src/fec")

set(cflags
    "-Wall"
)

set(libfec_rs_srcs
    "${libfec_rs_dir}/encode_rs_char.c"
    "${libfec_rs_dir}/decode_rs_char.c"
    "${libfec_rs_dir}/init_rs_char.c"
)

add_library(${target_name} STATIC ${libfec_rs_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
   ${libfec_headers}
)
