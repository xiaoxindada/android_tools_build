set(target_name "gflags")

set(libgflags_dir "${CMAKE_SOURCE_DIR}/src/gflags")

set(cflags
   "-D__STDC_FORMAT_MACROS"
        "-DHAVE_INTTYPES_H"
        "-DHAVE_SYS_STAT_H"
        "-DHAVE_PTHREAD"
        "-Wall"
        "-Wno-cast-function-type-mismatch"
        "-Wno-implicit-fallthrough"
)

set(libgflags_srcs
    "${libgflags_dir}/src/gflags.cc"
    "${libgflags_dir}/src/gflags_completions.cc"
    "${libgflags_dir}/src/gflags_reporting.cc"
)

# if(NOT CMAKE_SYSTEM_NAME STREQUAL "Android")
#     list(APPEND libgflags_srcs "${libgflags_dir}/epoll.c")
#     list(APPEND cflags "-D_GNU_SOURCE=1")
# endif()

add_library(${target_name} STATIC ${libgflags_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${libgflags_headers}
)