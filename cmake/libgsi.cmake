set(target_name "gsi")

set(libgsi_dir "${CMAKE_SOURCE_DIR}/src/gsid")


set(libgsi_srcs
    "${libgsi_dir}/libgsi.cpp"
)

add_library(${target_name} STATIC ${libgsi_srcs})
target_compile_options(${target_name} PRIVATE "-Wall")
target_include_directories(${target_name} PUBLIC
    ${libbase_headers}
    ${libgsi_headers}
)
target_link_libraries(${target_name} PUBLIC
        base
)