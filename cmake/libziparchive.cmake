set(target_name "ziparchive")

set(libziparchive_dir "${CMAKE_SOURCE_DIR}/src/libziparchive")

set(cflags
    # ZLIB_CONST turns on const for input buffers which is pretty standard.
    "-DZLIB_CONST"
    "-D_FILE_OFFSET_BITS=64"
)

set(cppflags
    "-std=c++2a"
    # Incorrectly warns when C++11 empty brace {} initializer is used.
    # https:#gcc.gnu.org/bugzilla/show_bug.cgi?id=61489
    "-Wno-missing-field-initializers"
    "-Wconversion"
    "-Wno-sign-conversion"
    "-DZIPARCHIVE_DISABLE_CALLBACK_API=1"
    "-DINCFS_SUPPORT_DISABLED=1"
)

if(NOT WIN32)
    list(APPEND cppflags "-Wold-style-cast")
endif()

set(libziparchive_srcs
    "${libziparchive_dir}/zip_archive.cc"
    "${libziparchive_dir}/zip_archive_stream_entry.cc"
    "${libziparchive_dir}/zip_cd_entry_map.cc"
    "${libziparchive_dir}/zip_error.cpp"
    "${libziparchive_dir}/zip_writer.cc"
)

add_library(${target_name} STATIC ${libziparchive_srcs})
target_compile_options(${target_name} PRIVATE ${cflags} ${cppflags})
target_include_directories(${target_name} PUBLIC
    ${libgtest_prod_headers}
    ${libbase_headers}
    ${liblog_headers}
    ${libziparchive_headers}
)
target_link_libraries(${target_name} PUBLIC
    base
    log
    zlib
)