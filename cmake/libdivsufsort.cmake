set(target_name "divsufsort")

set(libdivsufsort_dir "${CMAKE_SOURCE_DIR}/src/libdivsufsort")

set(cflags
        "-Wall"
        "-Wextra"
        "-DHAVE_CONFIG_H=1"
)

set(libdivsufsort_srcs
        "${libdivsufsort_dir}/lib/divsufsort.c"
        "${libdivsufsort_dir}/lib/sssort.c"
        "${libdivsufsort_dir}/lib/trsort.c"
        "${libdivsufsort_dir}/lib/utils.c"
)

add_library(${target_name} STATIC ${libdivsufsort_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${libdivsufsort_headers}
)

add_library("${target_name}64" STATIC ${libdivsufsort_srcs})
target_compile_options("${target_name}64" PRIVATE ${cflags} "-DBUILD_DIVSUFSORT64")
target_include_directories("${target_name}64" PUBLIC
    ${libdivsufsort_headers}
)