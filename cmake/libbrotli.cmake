set(target_name "brotli")

set(brotli_dir "${CMAKE_SOURCE_DIR}/src/brotli")

file(GLOB common_srcs "${brotli_dir}/c/common/*.c")
file(GLOB dec_srcs "${brotli_dir}/c/dec/*.c")
file(GLOB enc_srcs "${brotli_dir}/c/enc/*.c")

add_library(${target_name} STATIC 
    ${common_srcs}
    ${dec_srcs}
    ${enc_srcs}
)
target_compile_options(${target_name} PRIVATE "-Wall")
target_include_directories(${target_name} PUBLIC ${libbrotli_headers})