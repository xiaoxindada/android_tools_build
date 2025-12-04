set(protobuf_dir "${CMAKE_SOURCE_DIR}/src/protobuf")

set(protobuf_cflags
        "-Wall"
        "-Wno-missing-field-initializers"
        "-Wno-unused-function"
        "-Wno-unused-parameter"
        "-Wno-error=user-defined-warnings"
        "-Wno-deprecated-enum-enum-conversion"
)

if(NOT CMAKE_SYSTEM_NAME STREQUAL "Android")
    list(APPEND protobuf_cflags "-frtti")
endif()

set(protobuf_cpp_lite_srcs
        "${protobuf_dir}/src/google/protobuf/any_lite.cc"
        "${protobuf_dir}/src/google/protobuf/arena.cc"
        "${protobuf_dir}/src/google/protobuf/arenastring.cc"
        "${protobuf_dir}/src/google/protobuf/arenaz_sampler.cc"
        "${protobuf_dir}/src/google/protobuf/extension_set.cc"
        "${protobuf_dir}/src/google/protobuf/generated_enum_util.cc"
        "${protobuf_dir}/src/google/protobuf/generated_message_tctable_lite.cc"
        "${protobuf_dir}/src/google/protobuf/generated_message_util.cc"
        "${protobuf_dir}/src/google/protobuf/implicit_weak_message.cc"
        "${protobuf_dir}/src/google/protobuf/inlined_string_field.cc"
        "${protobuf_dir}/src/google/protobuf/io/coded_stream.cc"
        "${protobuf_dir}/src/google/protobuf/io/io_win32.cc"
        "${protobuf_dir}/src/google/protobuf/io/strtod.cc"
        "${protobuf_dir}/src/google/protobuf/io/zero_copy_stream.cc"
        "${protobuf_dir}/src/google/protobuf/io/zero_copy_stream_impl.cc"
        "${protobuf_dir}/src/google/protobuf/io/zero_copy_stream_impl_lite.cc"
        "${protobuf_dir}/src/google/protobuf/map.cc"
        "${protobuf_dir}/src/google/protobuf/message_lite.cc"
        "${protobuf_dir}/src/google/protobuf/parse_context.cc"
        "${protobuf_dir}/src/google/protobuf/repeated_field.cc"
        "${protobuf_dir}/src/google/protobuf/repeated_ptr_field.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/bytestream.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/common.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/int128.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/status.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/statusor.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/stringpiece.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/stringprintf.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/structurally_valid.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/strutil.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/time.cc"
        "${protobuf_dir}/src/google/protobuf/wire_format_lite.cc"
)

set(protobuf_cpp_full_srcs
        "${protobuf_dir}/src/google/protobuf/any.cc"
        "${protobuf_dir}/src/google/protobuf/any.pb.cc"
        "${protobuf_dir}/src/google/protobuf/api.pb.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/importer.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/parser.cc"
        "${protobuf_dir}/src/google/protobuf/descriptor.cc"
        "${protobuf_dir}/src/google/protobuf/descriptor.pb.cc"
        "${protobuf_dir}/src/google/protobuf/descriptor_database.cc"
        "${protobuf_dir}/src/google/protobuf/duration.pb.cc"
        "${protobuf_dir}/src/google/protobuf/dynamic_message.cc"
        "${protobuf_dir}/src/google/protobuf/empty.pb.cc"
        "${protobuf_dir}/src/google/protobuf/extension_set_heavy.cc"
        "${protobuf_dir}/src/google/protobuf/field_mask.pb.cc"
        "${protobuf_dir}/src/google/protobuf/generated_message_bases.cc"
        "${protobuf_dir}/src/google/protobuf/generated_message_reflection.cc"
        "${protobuf_dir}/src/google/protobuf/generated_message_tctable_full.cc"
        "${protobuf_dir}/src/google/protobuf/io/gzip_stream.cc"
        "${protobuf_dir}/src/google/protobuf/io/printer.cc"
        "${protobuf_dir}/src/google/protobuf/io/tokenizer.cc"
        "${protobuf_dir}/src/google/protobuf/map_field.cc"
        "${protobuf_dir}/src/google/protobuf/message.cc"
        "${protobuf_dir}/src/google/protobuf/reflection_ops.cc"
        "${protobuf_dir}/src/google/protobuf/service.cc"
        "${protobuf_dir}/src/google/protobuf/source_context.pb.cc"
        "${protobuf_dir}/src/google/protobuf/struct.pb.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/substitute.cc"
        "${protobuf_dir}/src/google/protobuf/text_format.cc"
        "${protobuf_dir}/src/google/protobuf/timestamp.pb.cc"
        "${protobuf_dir}/src/google/protobuf/type.pb.cc"
        "${protobuf_dir}/src/google/protobuf/unknown_field_set.cc"
        "${protobuf_dir}/src/google/protobuf/util/delimited_message_util.cc"
        "${protobuf_dir}/src/google/protobuf/util/field_comparator.cc"
        "${protobuf_dir}/src/google/protobuf/util/field_mask_util.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/datapiece.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/default_value_objectwriter.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/error_listener.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/field_mask_utility.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/json_escaping.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/json_objectwriter.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/json_stream_parser.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/object_writer.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/proto_writer.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/protostream_objectsource.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/protostream_objectwriter.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/type_info.cc"
        "${protobuf_dir}/src/google/protobuf/util/internal/utility.cc"
        "${protobuf_dir}/src/google/protobuf/util/json_util.cc"
        "${protobuf_dir}/src/google/protobuf/util/message_differencer.cc"
        "${protobuf_dir}/src/google/protobuf/util/time_util.cc"
        "${protobuf_dir}/src/google/protobuf/util/type_resolver_util.cc"
        "${protobuf_dir}/src/google/protobuf/wire_format.cc"
        "${protobuf_dir}/src/google/protobuf/wrappers.pb.cc"
)

set(protoc_srcs
        "${protobuf_dir}/src/google/protobuf/compiler/code_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/command_line_interface.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/enum.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/enum_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/extension.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/file.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/helpers.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/map_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/message.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/message_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/padding_optimizer.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/parse_function_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/primitive_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/service.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/string_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_doc_comment.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_enum.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_enum_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_field_base.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_helpers.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_map_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_message.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_message_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_primitive_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_reflection_class.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_repeated_enum_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_repeated_message_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_repeated_primitive_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_source_generator_base.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/csharp_wrapper_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/context.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/doc_comment.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/enum.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/enum_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/enum_field_lite.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/enum_lite.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/extension.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/extension_lite.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/file.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/generator_factory.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/helpers.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/kotlin_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/map_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/map_field_lite.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/message.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/message_builder.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/message_builder_lite.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/message_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/message_field_lite.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/message_lite.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/name_resolver.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/primitive_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/primitive_field_lite.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/service.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/shared_code_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/string_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/string_field_lite.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_enum.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_enum_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_extension.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_file.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_helpers.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_map_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_message.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_message_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_oneof.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/objectivec_primitive_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/php/php_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/plugin.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/plugin.pb.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/python/generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/python/helpers.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/python/pyi_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/ruby/ruby_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/subprocess.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/zip_writer.cc"
)

add_library(protobuf-cpp-lite STATIC ${protobuf_cpp_lite_srcs})
target_compile_options(protobuf-cpp-lite PRIVATE ${protobuf_cflags})
target_include_directories(protobuf-cpp-lite PUBLIC
    ${protobuf_dir}
    ${protobuf_headers}
)
target_link_libraries(protobuf-cpp-lite PUBLIC
    log
)

add_library(protobuf-cpp-full STATIC 
    ${protobuf_cpp_lite_srcs} 
    ${protobuf_cpp_full_srcs}
)
target_compile_options(protobuf-cpp-full PRIVATE ${protobuf_cflags} "-DHAVE_ZLIB=1")
target_include_directories(protobuf-cpp-full PUBLIC
    ${protobuf_dir}
    ${protobuf_headers}
)
target_link_libraries(protobuf-cpp-full PUBLIC
    log
    zlib
)

add_library(protoc STATIC
    ${protobuf_cpp_lite_srcs} 
    ${protobuf_cpp_full_srcs} 
    ${protoc_srcs}
 )
target_compile_options(protoc PRIVATE ${protobuf_cflags} "-DHAVE_ZLIB=1")
target_include_directories(protoc PUBLIC
    ${protobuf_dir}
    ${protobuf_headers}
)
target_link_libraries(protoc PUBLIC
    log
    zlib
)