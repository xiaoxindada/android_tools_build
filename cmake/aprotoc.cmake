set(target_name "aprotoc")

set(aprotoc_srcs
    "${protobuf_dir}/src/google/protobuf/compiler/main.cc"
)

add_executable(${target_name} ${aprotoc_srcs})
target_compile_options(${target_name} PRIVATE ${protobuf_cflags} "-DHAVE_ZLIB=1")
target_link_options(${target_name} PRIVATE "-static")
target_include_directories(${target_name} PUBLIC
    ${protobuf_dir}
    ${protobuf_headers}
    ${absl_headers}
)
target_link_libraries(${target_name} PRIVATE
    absl_log_initialize
    protoc
    zlib
)