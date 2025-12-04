set(target "xz")
set(xz_embedded_dir "${CMAKE_SOURCE_DIR}/src/xz-embedded")

set(libxz_srcs
    "${xz_embedded_dir}/linux/lib/xz/xz_crc32.c"
    "${xz_embedded_dir}/linux/lib/xz/xz_dec_bcj.c"
    "${xz_embedded_dir}/linux/lib/xz/xz_dec_lzma2.c"
    "${xz_embedded_dir}/linux/lib/xz/xz_dec_stream.c"
)

set(cflags
        "-DXZ_DEC_X86"
        "-DXZ_DEC_ARM"
        "-DXZ_DEC_ARMTHUMB"
        "-Wall"
)

add_library(${target} STATIC ${libxz_srcs})
target_compile_options(${target} PRIVATE ${cflags})
target_include_directories(${target} PUBLIC 
    ${libxz_headers}
)