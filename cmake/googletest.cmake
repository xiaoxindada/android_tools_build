
set(libgtest_srcs_dir "${CMAKE_SOURCE_DIR}/src/googletest/googletest")
set(libgmock_srcs_dir "${CMAKE_SOURCE_DIR}/src/googletest/googlemock")

set(libgtest_srcs
        "${libgtest_srcs_dir}/src/gtest-all.cc"
)

set(libgmock_srcs
        "${libgmock_srcs_dir}/src/gmock-all.cc"
)

set(cflags
    "-std=c++14"
    "-Wall"
    "-frtti"
    "-Wno-unused-private-field"
)

add_library(gtest STATIC ${libgtest_srcs})
target_compile_options(gtest PRIVATE ${cflags})
target_include_directories(gtest PUBLIC
    ${libgtest_srcs_dir}
    ${libgtest_prod_headers}
)

add_library(gmock STATIC ${libgmock_srcs})
target_compile_options(gmock PRIVATE
     ${cflags}
    "-Wno-missing-field-initializers"
    "-Wno-sign-compare"
    "-Wno-unused-parameter"
    # We need to generate exception tables, otherwise any users of this
    # library that use testing::Throw will trigger undefined behaviour.
    "-fexceptions"
)
target_include_directories(gmock PUBLIC
    ${libgmock_srcs_dir}
    ${libgmock_headers}
    ${libgtest_prod_headers}
)