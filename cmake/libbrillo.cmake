set(libbrillo_dir "${CMAKE_SOURCE_DIR}/src/libbrillo")

set(libbrillo_core_sources
    "${libbrillo_dir}/brillo/backoff_entry.cc"
    "${libbrillo_dir}/brillo/data_encoding.cc"
    "${libbrillo_dir}/brillo/errors/error.cc"
    "${libbrillo_dir}/brillo/errors/error_codes.cc"
    "${libbrillo_dir}/brillo/flag_helper.cc"
    "${libbrillo_dir}/brillo/key_value_store.cc"
    "${libbrillo_dir}/brillo/message_loops/base_message_loop.cc"
    "${libbrillo_dir}/brillo/message_loops/message_loop.cc"
    "${libbrillo_dir}/brillo/message_loops/message_loop_utils.cc"
    "${libbrillo_dir}/brillo/mime_utils.cc"
    "${libbrillo_dir}/brillo/osrelease_reader.cc"
    "${libbrillo_dir}/brillo/process.cc"
    "${libbrillo_dir}/brillo/process_information.cc"
    "${libbrillo_dir}/brillo/secure_blob.cc"
    "${libbrillo_dir}/brillo/strings/string_utils.cc"
    "${libbrillo_dir}/brillo/syslog_logging.cc"
    "${libbrillo_dir}/brillo/type_name_undecorate.cc"
    "${libbrillo_dir}/brillo/url_utils.cc"
    "${libbrillo_dir}/brillo/userdb_utils.cc"
    "${libbrillo_dir}/brillo/value_conversion.cc"
)

set(libbrillo_linux_sources
    # "${libbrillo_dir}/brillo/asynchronous_signal_handler.cc"
    "${libbrillo_dir}/brillo/daemons/daemon.cc"
    # "${libbrillo_dir}/brillo/file_utils.cc"
    "${libbrillo_dir}/brillo/process_reaper.cc"
)

set(libbrillo_stream_sources
    "${libbrillo_dir}/brillo/streams/file_stream.cc"
    "${libbrillo_dir}/brillo/streams/input_stream_set.cc"
    "${libbrillo_dir}/brillo/streams/memory_containers.cc"
    "${libbrillo_dir}/brillo/streams/memory_stream.cc"
    "${libbrillo_dir}/brillo/streams/openssl_stream_bio.cc"
    "${libbrillo_dir}/brillo/streams/stream.cc"
    "${libbrillo_dir}/brillo/streams/stream_errors.cc"
    "${libbrillo_dir}/brillo/streams/stream_utils.cc"
    "${libbrillo_dir}/brillo/streams/tls_stream.cc"
)

set(libbrillo_CFLAGS
    "-std=c++23"
    "-Wall"
    "-Wno-non-virtual-dtor"
    "-Wno-unused-parameter"
    "-Wno-unused-variable"
)

if(CMAKE_SYSTEM_NAME STREQUAL "Android")
    list(APPEND libbrillo_core_sources ${libbrillo_linux_sources})
else()
    list(APPEND libbrillo_core_sources ${libbrillo_linux_sources})
    list(APPEND libbrillo_CFLAGS "-D__ANDROID_HOST__")
endif()

add_library(brillo STATIC ${libbrillo_core_sources})
target_compile_options(brillo PRIVATE ${libbrillo_CFLAGS})
target_include_directories(brillo PUBLIC
    ${libgtest_prod_headers}
    ${libbrillo_headers}
    ${libchrome_headers}
    ${libmodpb64_headers}
)
target_link_libraries(brillo PUBLIC
    modpb64
    chrome
    gtest
)

add_library(brillo-stream STATIC ${libbrillo_stream_sources})
target_compile_options(brillo-stream PRIVATE ${libbrillo_CFLAGS})
target_include_directories(brillo-stream PUBLIC
    ${libbrillo_dir}
    ${libgtest_prod_headers}
    ${libchrome_headers}
    ${libmodpb64_headers}
)
target_link_libraries(brillo-stream PUBLIC
        brillo
        crypto
        ssl
)