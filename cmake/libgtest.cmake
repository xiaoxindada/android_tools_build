set(target_name "gtest")

set(libgtest_srcs_dir "${CMAKE_SOURCE_DIR}/src/googletest/googletest")

set(libgtest_srcs
        "${libgtest_srcs_dir}/src/gtest-all.cc"
)

set(cflags 
    "-Wall"
    "-Wno-unused-private-field"
)

add_library(${target_name} STATIC ${libgtest_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${libgtest_srcs_dir}
    ${libgtest_prod_headers}
)