set(target_name "zstd")

set(libzstd_dir "${CMAKE_SOURCE_DIR}/src/zstd/lib")

set(cflags
    "-DZSTD_HAVE_WEAK_SYMBOLS=0"
    "-DZSTD_TRACE=0"
    "-DZSTD_DISABLE_ASM"
)

file(GLOB libzstd_srcs "${libzstd_dir}/*/*.c")

add_library(${target_name} STATIC ${libzstd_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${libzstd_headers}
)