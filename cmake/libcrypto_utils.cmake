set(target_name "crypto_utils")

set(libcrypto_utils_dir "${CMAKE_SOURCE_DIR}/src/core/libcrypto_utils")

set(cflags
    "-Wall"
    "-Wextra"
)

set(libcrypto_utils_srcs
    "${libcrypto_utils_dir}/android_pubkey.cpp"
)

add_library(${target_name} STATIC ${libcrypto_utils_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${boringssl_headers}
    ${libcrypto_utils_headers}
)
target_link_libraries(${target_name} PUBLIC 
    crypto
)