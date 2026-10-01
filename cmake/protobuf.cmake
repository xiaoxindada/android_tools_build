set(protobuf_dir "${CMAKE_SOURCE_DIR}/src/protobuf")

set(protobuf_cflags
        "-Wall"
        "-Wno-missing-field-initializers"
        "-Wno-unused-function"
        "-Wno-unused-parameter"
        "-Wno-error=user-defined-warnings"
        "-Wno-deprecated-enum-enum-conversion"
        "-Wno-unused-private-field"
)

if(NOT CMAKE_SYSTEM_NAME STREQUAL "Android")
    list(APPEND protobuf_cflags "-frtti")
endif()

set(absl_libs 
    # lite
    # "absl_base_dynamic_annotations"
    # "absl_container_btree"
    # "absl_container_flat_hash_map"
    # "absl_container_flat_hash_set"
    # "absl_log_absl_check"
    # "absl_log_absl_log"
    # "absl_strings"
    # "absl_strings_cord"
    # "absl_strings_str_format"
    # "absl_synchronization"
    # "absl_types_optional"
    # "absl_types_span"

    # full
    # "absl_cleanup"
    # "absl_functional_function_ref"
    # "absl_log_die_if_null"
    # "absl_time"
    # "absl_types_variant"

    absl_base
    absl_log
    absl_strings
    absl_synchronization
    # abseil-cpp 新版本已把 low_level_hash 合并进 absl_hash(internal/hash.cc)，
    # 不再有独立的 absl_low_level_hash 目标；此处保留会退化成 -labsl_low_level_hash 链接失败。
    absl_hash
    absl_raw_hash_set
    absl_status
    absl_statusor
    absl_cleanup
    absl_log_internal_check_op
    absl_time
)

set(protobuf_cpp_lite_srcs
        "${protobuf_dir}/src/google/protobuf/any_lite.cc"
        "${protobuf_dir}/src/google/protobuf/arena.cc"
        "${protobuf_dir}/src/google/protobuf/arena_align.cc"
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
        "${protobuf_dir}/src/google/protobuf/io/zero_copy_sink.cc"
        "${protobuf_dir}/src/google/protobuf/io/zero_copy_stream.cc"
        "${protobuf_dir}/src/google/protobuf/io/zero_copy_stream_impl.cc"
        "${protobuf_dir}/src/google/protobuf/io/zero_copy_stream_impl_lite.cc"
        "${protobuf_dir}/src/google/protobuf/map.cc"
        "${protobuf_dir}/src/google/protobuf/message_lite.cc"
        "${protobuf_dir}/src/google/protobuf/micro_string.cc"
        "${protobuf_dir}/src/google/protobuf/parse_context.cc"
        "${protobuf_dir}/src/google/protobuf/port.cc"
        "${protobuf_dir}/src/google/protobuf/raw_ptr.cc"
        "${protobuf_dir}/src/google/protobuf/repeated_field.cc"
        "${protobuf_dir}/src/google/protobuf/repeated_ptr_field.cc"
        "${protobuf_dir}/src/google/protobuf/stubs/common.cc"
        "${protobuf_dir}/src/google/protobuf/wire_format_lite.cc"
        "${protobuf_dir}/third_party/utf8_range/naive.c"
        "${protobuf_dir}/third_party/utf8_range/range2-neon.c"
        "${protobuf_dir}/third_party/utf8_range/range2-sse.c"
        "${protobuf_dir}/third_party/utf8_range/utf8_range.c"
)

set(protobuf_cpp_full_srcs
        ${protobuf_cpp_lite_srcs}
        "${protobuf_dir}/src/google/protobuf/any.cc"
        "${protobuf_dir}/src/google/protobuf/any.pb.cc"
        "${protobuf_dir}/src/google/protobuf/api.pb.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/importer.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/parser.cc"
        "${protobuf_dir}/src/google/protobuf/cpp_features.pb.cc"
        "${protobuf_dir}/src/google/protobuf/descriptor.cc"
        "${protobuf_dir}/src/google/protobuf/descriptor.pb.cc"
        "${protobuf_dir}/src/google/protobuf/descriptor_database.cc"
        "${protobuf_dir}/src/google/protobuf/duration.pb.cc"
        "${protobuf_dir}/src/google/protobuf/dynamic_message.cc"
        "${protobuf_dir}/src/google/protobuf/empty.pb.cc"
        "${protobuf_dir}/src/google/protobuf/extension_set_heavy.cc"
        "${protobuf_dir}/src/google/protobuf/feature_resolver.cc"
        "${protobuf_dir}/src/google/protobuf/field_mask.pb.cc"
        "${protobuf_dir}/src/google/protobuf/generated_message_bases.cc"
        "${protobuf_dir}/src/google/protobuf/generated_message_reflection.cc"
        "${protobuf_dir}/src/google/protobuf/generated_message_tctable_full.cc"
        "${protobuf_dir}/src/google/protobuf/generated_message_tctable_gen.cc"
        "${protobuf_dir}/src/google/protobuf/internal_feature_helper.cc"
        "${protobuf_dir}/src/google/protobuf/io/gzip_stream.cc"
        "${protobuf_dir}/src/google/protobuf/io/printer.cc"
        "${protobuf_dir}/src/google/protobuf/io/strtod.cc"
        "${protobuf_dir}/src/google/protobuf/io/tokenizer.cc"
        "${protobuf_dir}/src/google/protobuf/json/internal/lexer.cc"
        "${protobuf_dir}/src/google/protobuf/json/internal/message_path.cc"
        "${protobuf_dir}/src/google/protobuf/json/internal/parser.cc"
        "${protobuf_dir}/src/google/protobuf/json/internal/unparser.cc"
        "${protobuf_dir}/src/google/protobuf/json/internal/untyped_message.cc"
        "${protobuf_dir}/src/google/protobuf/json/internal/writer.cc"
        "${protobuf_dir}/src/google/protobuf/json/internal/zero_copy_buffered_stream.cc"
        "${protobuf_dir}/src/google/protobuf/json/json.cc"
        "${protobuf_dir}/src/google/protobuf/map_field.cc"
        "${protobuf_dir}/src/google/protobuf/message.cc"
        "${protobuf_dir}/src/google/protobuf/reflection_mode.cc"
        "${protobuf_dir}/src/google/protobuf/reflection_ops.cc"
        "${protobuf_dir}/src/google/protobuf/service.cc"
        "${protobuf_dir}/src/google/protobuf/source_context.pb.cc"
        "${protobuf_dir}/src/google/protobuf/struct.pb.cc"
        "${protobuf_dir}/src/google/protobuf/text_format.cc"
        "${protobuf_dir}/src/google/protobuf/timestamp.pb.cc"
        "${protobuf_dir}/src/google/protobuf/type.pb.cc"
        "${protobuf_dir}/src/google/protobuf/unknown_field_set.cc"
        "${protobuf_dir}/src/google/protobuf/util/delimited_message_util.cc"
        "${protobuf_dir}/src/google/protobuf/util/field_comparator.cc"
        "${protobuf_dir}/src/google/protobuf/util/field_mask_util.cc"
        "${protobuf_dir}/src/google/protobuf/util/message_differencer.cc"
        "${protobuf_dir}/src/google/protobuf/util/time_util.cc"
        "${protobuf_dir}/src/google/protobuf/util/type_resolver_util.cc"
        "${protobuf_dir}/src/google/protobuf/wire_format.cc"
        "${protobuf_dir}/src/google/protobuf/wrappers.pb.cc"
)

set(protoc_srcs
        "${protobuf_dir}/src/google/protobuf/compiler/code_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/code_generator_lite.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/command_line_interface.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/enum.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/extension.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/field_chunk.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/field_generators/cord_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/field_generators/enum_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/field_generators/map_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/field_generators/message_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/field_generators/primitive_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/field_generators/string_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/field_generators/string_view_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/file.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/helpers.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/ifndef_guard.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/message.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/message_layout_helper.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/namespace_printer.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/parse_function_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/service.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/cpp/tracker.cc"
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
        "${protobuf_dir}/src/google/protobuf/compiler/csharp/names.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/context.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/doc_comment.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/field_common.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/file.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/enum.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/enum_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/extension.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/generator_factory.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/make_field_gens.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/map_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/message.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/message_builder.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/message_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/primitive_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/service.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/full/string_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/helpers.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/internal_helpers.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/java_features.pb.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/lite/enum.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/lite/enum_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/lite/extension.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/lite/generator_factory.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/lite/make_field_gens.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/lite/map_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/lite/message.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/lite/message_builder.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/lite/message_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/lite/primitive_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/lite/string_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/message_serialization.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/name_resolver.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/names.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/java/shared_code_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/kotlin/field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/kotlin/file.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/kotlin/generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/kotlin/message.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/enum.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/enum_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/extension.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/file.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/helpers.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/import_writer.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/line_consumer.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/map_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/message.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/message_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/names.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/oneof.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/primitive_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/objectivec/tf_decode_data.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/php/names.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/php/php_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/plugin.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/plugin.pb.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/python/generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/python/helpers.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/python/pyi_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/retention.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/ruby/ruby_generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/accessors/accessor_case.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/accessors/accessors.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/accessors/default_value.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/accessors/map.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/accessors/repeated_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/accessors/singular_cord.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/accessors/singular_message.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/accessors/singular_scalar.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/accessors/singular_string.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/accessors/unsupported_field.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/accessors/with_presence.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/context.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/crate_mapping.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/enum.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/generator.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/message.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/naming.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/oneof.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/relative_path.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/rust_field_type.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/rust_keywords.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/rust/upb_helpers.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/subprocess.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/versions.cc"
        "${protobuf_dir}/src/google/protobuf/compiler/zip_writer.cc"
        "${protobuf_dir}/src/google/protobuf/testing/file.cc"
)

set(upb_srcs
        "${protobuf_dir}/upb/base/status.c"
        "${protobuf_dir}/upb/hash/common.c"
        "${protobuf_dir}/upb/mem/alloc.c"
        "${protobuf_dir}/upb/mem/arena.c"
        "${protobuf_dir}/upb/message/accessors.c"
        "${protobuf_dir}/upb/message/array.c"
        "${protobuf_dir}/upb/message/compat.c"
        "${protobuf_dir}/upb/message/copy.c"
        "${protobuf_dir}/upb/message/internal/extension.c"
        "${protobuf_dir}/upb/message/internal/iterator.c"
        "${protobuf_dir}/upb/message/internal/message.c"
        "${protobuf_dir}/upb/message/map.c"
        "${protobuf_dir}/upb/message/map_sorter.c"
        "${protobuf_dir}/upb/message/message.c"
        "${protobuf_dir}/upb/mini_descriptor/build_enum.c"
        "${protobuf_dir}/upb/mini_descriptor/decode.c"
        "${protobuf_dir}/upb/mini_descriptor/internal/base92.c"
        "${protobuf_dir}/upb/mini_descriptor/internal/encode.c"
        "${protobuf_dir}/upb/mini_descriptor/link.c"
        "${protobuf_dir}/upb/mini_table/extension_registry.c"
        "${protobuf_dir}/upb/mini_table/internal/message.c"
        "${protobuf_dir}/upb/mini_table/message.c"
        "${protobuf_dir}/upb/reflection/cmake/google/protobuf/descriptor.upb_minitable.c"
        "${protobuf_dir}/upb/reflection/def_pool.c"
        "${protobuf_dir}/upb/reflection/def_type.c"
        "${protobuf_dir}/upb/reflection/desc_state.c"
        "${protobuf_dir}/upb/reflection/enum_def.c"
        "${protobuf_dir}/upb/reflection/enum_reserved_range.c"
        "${protobuf_dir}/upb/reflection/enum_value_def.c"
        "${protobuf_dir}/upb/reflection/extension_range.c"
        "${protobuf_dir}/upb/reflection/field_def.c"
        "${protobuf_dir}/upb/reflection/file_def.c"
        "${protobuf_dir}/upb/reflection/internal/def_builder.c"
        "${protobuf_dir}/upb/reflection/internal/strdup2.c"
        "${protobuf_dir}/upb/reflection/message.c"
        "${protobuf_dir}/upb/reflection/message_def.c"
        "${protobuf_dir}/upb/reflection/message_reserved_range.c"
        "${protobuf_dir}/upb/reflection/method_def.c"
        "${protobuf_dir}/upb/reflection/oneof_def.c"
        "${protobuf_dir}/upb/reflection/service_def.c"
        "${protobuf_dir}/upb/util/def_to_proto.c"
        "${protobuf_dir}/upb/wire/decode.c"
        "${protobuf_dir}/upb/wire/encode.c"
        "${protobuf_dir}/upb/wire/eps_copy_input_stream.c"
        "${protobuf_dir}/upb/wire/internal/decoder.c"
        "${protobuf_dir}/upb/wire/reader.c"
)

set(upb_generator_srcs
        "${protobuf_dir}/upb_generator/c/names.cc"
        "${protobuf_dir}/upb_generator/c/names_internal.cc"
        "${protobuf_dir}/upb_generator/cmake/google/protobuf/compiler/plugin.upb_minitable.c"
        "${protobuf_dir}/upb_generator/common.cc"
        "${protobuf_dir}/upb_generator/common/cpp_to_upb_def.cc"
        "${protobuf_dir}/upb_generator/common/names.cc"
        "${protobuf_dir}/upb_generator/file_layout.cc"
        "${protobuf_dir}/upb_generator/minitable/names.cc"
        "${protobuf_dir}/upb_generator/minitable/names_internal.cc"
        "${protobuf_dir}/upb_generator/plugin.cc"
        "${protobuf_dir}/upb_generator/reflection/header.cc"
        "${protobuf_dir}/upb_generator/reflection/names.cc"
        "${protobuf_dir}/upb_generator/reflection/source.cc"
)

set(upb_bootstrap_include_dirs
        "${protobuf_dir}/upb/reflection/cmake"
        "${protobuf_dir}/upb_generator/cmake"
)

set(aprotoc_srcs
    "${protobuf_dir}/src/google/protobuf/compiler/main.cc"
)

add_library(protobuf-cpp-lite STATIC ${protobuf_cpp_lite_srcs})
target_compile_options(protobuf-cpp-lite PRIVATE ${protobuf_cflags})
target_include_directories(protobuf-cpp-lite PUBLIC
    ${protobuf_dir}
    ${protobuf_headers}
    ${absl_headers}
)
target_link_libraries(protobuf-cpp-lite PUBLIC
    ${absl_libs}
    log
)

add_library(protobuf-cpp-full STATIC ${protobuf_cpp_full_srcs})
target_compile_options(protobuf-cpp-full PRIVATE ${protobuf_cflags} "-DHAVE_ZLIB=1")
target_include_directories(protobuf-cpp-full PUBLIC
    ${protobuf_dir}
    ${protobuf_headers}
    ${absl_headers}
)
target_link_libraries(protobuf-cpp-full PUBLIC
    ${absl_libs}
    log
    zlib
)

add_library(protoc STATIC
    ${protobuf_cpp_lite_srcs} 
    ${protobuf_cpp_full_srcs} 
    ${protoc_srcs}
    ${upb_srcs}
    ${upb_generator_srcs}
 )
target_compile_options(protoc PRIVATE ${protobuf_cflags} "-DHAVE_ZLIB=1")
target_include_directories(protoc PUBLIC
    ${protobuf_dir}
    ${upb_bootstrap_include_dirs}
    ${protobuf_headers}
    ${absl_headers}
)
target_link_libraries(protoc PUBLIC
    ${absl_libs}
    log
    zlib
)

add_executable(aprotoc ${aprotoc_srcs})
target_compile_options(aprotoc PRIVATE ${protobuf_cflags} "-DHAVE_ZLIB=1")
target_link_options(aprotoc PRIVATE "-static")
target_include_directories(aprotoc PUBLIC
    ${protobuf_dir}
    ${protobuf_headers}
    ${absl_headers}
)
target_link_libraries(aprotoc PRIVATE
    absl_log_initialize
    protoc
    zlib
)