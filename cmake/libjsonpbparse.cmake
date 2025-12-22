set(target_name "jsonpbparse")

set(jsonpbparse_dir "${CMAKE_SOURCE_DIR}/src/extras/libjsonpb/parse")

set(libjsonpbparse_srcs
    "${jsonpbparse_dir}/jsonpb.cpp"
)

set(cflags 
       "-Wall"
        "-Wno-unused-parameter"
)

add_library(${target_name} STATIC ${libjsonpbparse_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
        ${libjsonpbparse_headers}
)
target_link_libraries(${target_name} PUBLIC
        base
        protobuf-cpp-full
)