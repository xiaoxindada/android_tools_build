set(target_name "modpb64")

set(libmodpb64_srcs_dir "${CMAKE_SOURCE_DIR}/src/modp_b64")

set(libmodpb64_srcs 
    "${libmodpb64_srcs_dir}/modp_b64.cc"
)
set(cflags
    "-Wall"
)

add_library(${target_name} STATIC ${libmodpb64_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC 
    ${libmodpb64_srcs_dir}
    "${libmodpb64_srcs_dir}/modp_b64"
)