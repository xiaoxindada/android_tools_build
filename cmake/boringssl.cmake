
set(boringssl_dir "${CMAKE_SOURCE_DIR}/src/boringssl")
set(BORINGSSL_ROOT "${boringssl_dir}/")
include("${boringssl_dir}/android-sources.cmake")

set(ldflags "-pthread")

set(common_cflags
        "-std=c++17"
        "-Wall"
        "-DBORINGSSL_IMPLEMENTATION"
        "-DBORINGSSL_ANDROID_SYSTEM"
)

if (WIN32)
    list(APPEND ldflags "-ws2_32")
    list(REMOVE_ITEM android_cflags "-DBORINGSSL_FIPS")
    list(APPEND crypto_sources ${crypto_sources_nasm})
else()
    enable_language(ASM)
    list(APPEND crypto_sources ${crypto_sources_asm})
    if(CMAKE_SYSTEM_NAME STREQUAL "Android")
        list(APPEND "-Wl-Bsymbolic")
        if(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
            list(APPEND ldflags "-Wl,-Bsymbolic")
            list(APPEND common_cflags ${android_cflags})
            list(APPEND ldflags "-Wl--dynamic-list=${boringssl_dir}/src/crypto/fipsmodule/fips_shared.lds")
        endif()
    endif()
endif()

add_library(crypto ${crypto_sources})
target_link_options(crypto PRIVATE ${ldflags})
target_compile_options(crypto PRIVATE
    ${common_flags}
)

target_include_directories(crypto PUBLIC ${boringssl_headers})

add_library(ssl ${ssl_sources})
target_compile_options(ssl PRIVATE
    ${common_flags}
)
target_link_options(ssl PRIVATE ${ldflags})
target_include_directories(ssl PUBLIC ${boringssl_headers})