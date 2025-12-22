
set(update_engine_dir "${CMAKE_SOURCE_DIR}/src/update_engine")

set(update_metadata_protos_srcs
    "${update_engine_dir}/update_engine/update_metadata.pb.cc"
    "${update_engine_dir}/update_engine/update_metadata.pb.h"
)
set(lz4diff_protos_srcs
    "${update_engine_dir}/lz4diff/lz4diff.pb.cc"
    "${update_engine_dir}/lz4diff/lz4diff.pb.h"
)

add_custom_command(
    OUTPUT
        ${update_metadata_protos_srcs}
    COMMAND
        ${prebuilt_protoc} update_metadata.proto -I${update_engine_dir}/update_engine --cpp_out=${update_engine_dir}/update_engine
    DEPENDS
        ${update_engine_dir}/update_engine/update_metadata.proto
)
add_custom_target(gen_update_metadata_srcs ALL DEPENDS ${update_metadata_protos_srcs})

add_custom_command(
    OUTPUT
        ${lz4diff_protos_srcs}
    COMMAND
        ${prebuilt_protoc} lz4diff.proto -I${update_engine_dir}/lz4diff --cpp_out=${update_engine_dir}/lz4diff
    DEPENDS
        ${update_engine_dir}/lz4diff/lz4diff.proto
)
add_custom_target(gen_lz4diff_protos_srcs ALL DEPENDS ${lz4diff_protos_srcs})

add_library(update_metadata-protos STATIC ${update_metadata_protos_srcs})
target_compile_options(update_metadata-protos PRIVATE "-Wall")
target_include_directories(update_metadata-protos PUBLIC 
    ${protobuf_headers}
    ${absl_headers}
)
target_link_libraries(update_metadata-protos PUBLIC 
    protobuf-cpp-full
)

add_library(lz4diff-protos STATIC ${lz4diff_protos_srcs})
target_compile_options(lz4diff-protos PRIVATE ${cflags} ${cppflags})
target_include_directories(lz4diff-protos PUBLIC 
    ${protobuf_headers}
    ${absl_headers}
)

set(cflags
        "-DBASE_VER=576279"
        "-DUSE_HWID_OVERRIDE=0"
        "-D_FILE_OFFSET_BITS=64"
        "-D_POSIX_C_SOURCE=199309L"
        "-Wa,--noexecstack"
        "-Wall"
        "-Wextra"
        "-Wformat=2"
        "-Wno-psabi"
        "-Wno-unused-parameter"
        "-ffunction-sections"
        "-fstack-protector-strong"
        "-fvisibility=hidden"
        "-g3"
        "-Wno-macro-redefined"
)

set(cppflags
        "-Wnon-virtual-dtor"
        "-fno-strict-aliasing"
)

set(common_libs
        base
        brillo-stream
        brillo
        chrome
)

set(common_headers
    ${libbrillo_headers}
    ${libchrome_headers}
    ${libbase_headers}
    ${libgtest_prod_headers}
    ${update_engine_dir}
    ${protobuf_headers}
    ${absl_headers}
    "${update_engine_dir}/client_library/include"
    "${update_engine_dir}/aosp"
    "${update_engine_dir}/common"
    "${update_engine_dir}/payload_consumer"
    "${update_engine_dir}/.."
)

if(BUILD_ANDROID)
    list(APPEND cflags "-DUSE_FEC=0")
else()
     list(APPEND cflags "-DUSE_FEC=1")
endif()

set(libpayload_extent_utils_srcs
    "${update_engine_dir}/payload_generator/extent_utils.cc"
)

set(libpayload_extent_ranges_srcs
    "${update_engine_dir}/payload_generator/extent_ranges.cc"
)

set(libcow_operation_convert_srcs
    "${update_engine_dir}/common/cow_operation_convert.cc"
)
set(libbcow_size_estimator_srcs
    "${update_engine_dir}/payload_generator/cow_size_estimator.cc"
)

set(liblz4diff_srcs
    ${lz4diff_protos_srcs}
    "${update_engine_dir}/lz4diff/lz4diff.cc"
    "${update_engine_dir}/lz4diff/lz4diff_compress.cc"
)

set(liblz4patch_srcs
    ${lz4diff_protos_srcs}
    "${update_engine_dir}/lz4diff/lz4patch.cc"
    "${update_engine_dir}/lz4diff/lz4diff_compress.cc"
)

set(libpayload_consumer_srcs
        "${update_engine_dir}/aosp/platform_constants_android.cc"
        "${update_engine_dir}/common/action_processor.cc"
        "${update_engine_dir}/common/boot_control_stub.cc"
        "${update_engine_dir}/common/clock.cc"
        "${update_engine_dir}/common/constants.cc"
        "${update_engine_dir}/common/cpu_limiter.cc"
        "${update_engine_dir}/common/dynamic_partition_control_stub.cc"
        "${update_engine_dir}/common/error_code_utils.cc"
        "${update_engine_dir}/common/file_fetcher.cc"
        "${update_engine_dir}/common/hash_calculator.cc"
        "${update_engine_dir}/common/http_common.cc"
        "${update_engine_dir}/common/http_fetcher.cc"
        "${update_engine_dir}/common/hwid_override.cc"
        "${update_engine_dir}/common/multi_range_http_fetcher.cc"
        "${update_engine_dir}/common/prefs.cc"
        "${update_engine_dir}/common/subprocess.cc"
        "${update_engine_dir}/common/terminator.cc"
        "${update_engine_dir}/common/utils.cc"
        "${update_engine_dir}/payload_consumer/bzip_extent_writer.cc"
        "${update_engine_dir}/payload_consumer/cached_file_descriptor.cc"
        "${update_engine_dir}/payload_consumer/certificate_parser_android.cc"
        "${update_engine_dir}/payload_consumer/cow_writer_file_descriptor.cc"
        "${update_engine_dir}/payload_consumer/delta_performer.cc"
        "${update_engine_dir}/payload_consumer/extent_reader.cc"
        "${update_engine_dir}/payload_consumer/extent_writer.cc"
        "${update_engine_dir}/payload_consumer/file_descriptor.cc"
        "${update_engine_dir}/payload_consumer/file_descriptor_utils.cc"
        "${update_engine_dir}/payload_consumer/file_writer.cc"
        "${update_engine_dir}/payload_consumer/filesystem_verifier_action.cc"
        "${update_engine_dir}/payload_consumer/install_operation_executor.cc"
        "${update_engine_dir}/payload_consumer/install_plan.cc"
        "${update_engine_dir}/payload_consumer/mount_history.cc"
        "${update_engine_dir}/payload_consumer/payload_constants.cc"
        "${update_engine_dir}/payload_consumer/payload_metadata.cc"
        "${update_engine_dir}/payload_consumer/payload_verifier.cc"
        "${update_engine_dir}/payload_consumer/partition_writer.cc"
        "${update_engine_dir}/payload_consumer/partition_writer_factory_android.cc"
        "${update_engine_dir}/payload_consumer/vabc_partition_writer.cc"
        "${update_engine_dir}/payload_consumer/xor_extent_writer.cc"
        "${update_engine_dir}/payload_consumer/block_extent_writer.cc"
        "${update_engine_dir}/payload_consumer/snapshot_extent_writer.cc"
        "${update_engine_dir}/payload_consumer/postinstall_runner_action.cc"
        "${update_engine_dir}/payload_consumer/verified_source_fd.cc"
        "${update_engine_dir}/payload_consumer/verity_writer_android.cc"
        "${update_engine_dir}/payload_consumer/xz_extent_writer.cc"
        "${update_engine_dir}/payload_consumer/fec_file_descriptor.cc"
        "${update_engine_dir}/payload_consumer/partition_update_generator_android.cc"
        "${update_engine_dir}/update_status_utils.cc"
)

set(libpayload_generator_srcs
        "${update_engine_dir}/common/system_state.cc"
        "${update_engine_dir}/download_action.cc"
        "${update_engine_dir}/payload_generator/ab_generator.cc"
        "${update_engine_dir}/payload_generator/annotated_operation.cc"
        "${update_engine_dir}/payload_generator/blob_file_writer.cc"
        "${update_engine_dir}/payload_generator/block_mapping.cc"
        "${update_engine_dir}/payload_generator/boot_img_filesystem.cc"
        "${update_engine_dir}/payload_generator/bzip.cc"
        "${update_engine_dir}/payload_generator/deflate_utils.cc"
        "${update_engine_dir}/payload_generator/delta_diff_generator.cc"
        "${update_engine_dir}/payload_generator/delta_diff_utils.cc"
        "${update_engine_dir}/payload_generator/ext2_filesystem.cc"
        "${update_engine_dir}/payload_generator/erofs_filesystem.cc"
        "${update_engine_dir}/payload_generator/extent_ranges.cc"
        "${update_engine_dir}/payload_generator/full_update_generator.cc"
        "${update_engine_dir}/payload_generator/mapfile_filesystem.cc"
        "${update_engine_dir}/payload_generator/merge_sequence_generator.cc"
        "${update_engine_dir}/payload_generator/payload_file.cc"
        "${update_engine_dir}/payload_generator/payload_generation_config_android.cc"
        "${update_engine_dir}/payload_generator/payload_generation_config.cc"
        "${update_engine_dir}/payload_generator/payload_properties.cc"
        "${update_engine_dir}/payload_generator/payload_signer.cc"
        "${update_engine_dir}/payload_generator/raw_filesystem.cc"
        "${update_engine_dir}/payload_generator/squashfs_filesystem.cc"
        "${update_engine_dir}/payload_generator/xz_android.cc"
)

set(delta_generator_srcs
    "${update_engine_dir}/payload_generator/generate_delta_main.cc"
)

add_library(payload_extent_utils STATIC ${libpayload_extent_utils_srcs})
target_compile_options(payload_extent_utils PRIVATE ${cflags} ${cppflags})
target_include_directories(payload_extent_utils PUBLIC
    ${common_headers}
)
target_link_libraries(payload_extent_utils PUBLIC
    ${common_libs}
    update_metadata-protos
)
add_library(payload_extent_ranges STATIC ${libpayload_extent_ranges_srcs})
target_compile_options(payload_extent_ranges PRIVATE ${cflags} ${cppflags})
target_include_directories(payload_extent_ranges PUBLIC
    ${common_headers}
)
target_link_libraries(payload_extent_ranges PUBLIC
    ${common_libs}
    update_metadata-protos
)

add_library(cow_operation_convert STATIC ${libcow_operation_convert_srcs})
target_compile_options(cow_operation_convert PRIVATE ${cflags} ${cppflags})
target_include_directories(cow_operation_convert PUBLIC
    ${common_headers}
    ${fs_mgr_headers}
)
target_link_libraries(cow_operation_convert PUBLIC
    ${common_libs}
    update_metadata-protos
)

add_library(cow_size_estimator STATIC ${libbcow_size_estimator_srcs})
target_compile_options(cow_size_estimator PRIVATE ${cflags} ${cppflags})
target_include_directories(cow_size_estimator PUBLIC
    ${common_headers}
    ${fs_mgr_headers}
)
target_link_libraries(cow_size_estimator PUBLIC
    ${common_libs}
        update_metadata-protos
        base
        snapshot_cow
        cow_operation_convert
)

add_library(lz4diff STATIC ${liblz4diff_srcs})
target_compile_options(lz4diff PRIVATE ${cflags} ${cppflags})
target_include_directories(lz4diff PUBLIC
    ${common_headers}
    ${boringssl_headers}
    ${liblz4_headers}
    ${libbsdiff_headers}
    ${puffin_headers}
)
target_link_libraries(lz4diff PUBLIC
    update_metadata-protos
    lz4diff-protos
    ssl
    bsdiff
    puffdiff
    lz4
)

add_library(lz4patch STATIC ${liblz4patch_srcs})
target_compile_options(lz4patch PRIVATE ${cflags} ${cppflags})
target_include_directories(lz4patch PUBLIC
    ${common_headers}
    ${boringssl_headers}
    ${liblz4_headers}
    ${libbsdiff_headers}
    ${puffin_headers}
)
target_link_libraries(lz4patch PUBLIC
    update_metadata-protos
    lz4diff-protos
    ssl
    bsdiff
    puffdiff
    lz4
)

add_library(payload_consumer STATIC ${libpayload_consumer_srcs})
target_compile_options(payload_consumer PRIVATE ${cflags} ${cppflags})
target_include_directories(payload_consumer PUBLIC
    ${common_headers}
    ${core_headers}
    ${libbase_headers}
    ${libbsdiff_headers}
    ${puffin_headers}
    ${libzucchini_headers}
    ${zlib_headers}
    ${libbz_headers}
    ${libxz_headers}
    ${libsnapshot_cow_headers}
    ${libfec_headers}
    ${boringssl_headers}
    ${libcrypto_utils_headers}
    ${libevent_headers}
    ${fs_mgr_headers}
)
target_link_libraries(payload_consumer PUBLIC
        protobuf-cpp-full
        xz
        bz
        bspatch
        brotli
        fec_rs
        puffpatch
        verity_tree
        snapshot_cow
        brotli
        zlib
        payload_extent_ranges
        payload_extent_utils
        cow_operation_convert
        lz4diff-protos
        lz4patch
        zstd
        base
        crypto
        fec
        lz4
        ziparchive
)

add_library(payload_generator STATIC ${libpayload_generator_srcs})
target_compile_options(payload_generator PRIVATE
        ${cflags} 
        ${cppflags}
        "-std=gnu++23"
        "-Wall"
        "-Wno-error=#warnings"
        "-Wno-ignored-qualifiers"
        "-Wno-pointer-arith"
        "-Wno-unused-parameter"
        "-Wno-unused-function"
        "-DHAVE_FALLOCATE"
        "-DHAVE_LINUX_TYPES_H"
        "-DHAVE_LIBSELINUX"
        "-DHAVE_LIBUUID"
        "-DLZ4_ENABLED"
        "-DLZ4HC_ENABLED"
        "-DWITH_ANDROID"
        "-DHAVE_MEMRCHR"
        "-DHAVE_SYS_IOCTL_H"
        "-DHAVE_LLISTXATTR"
        "-DHAVE_LGETXATTR"
        "-D_FILE_OFFSET_BITS=64"
        "-DEROFS_MAX_BLOCK_SIZE=16384"
        "-DHAVE_UTIMENSAT"
        "-DHAVE_UNISTD_H"
        "-DHAVE_SYSCONF"
        "-DEROFS_MT_ENABLED"
)
target_include_directories(payload_generator PUBLIC
    ${common_headers}
    ${core_headers}
    ${libbase_headers}
    ${libbsdiff_headers}
    ${puffin_headers}
    ${libzucchini_headers}
    ${zlib_headers}
    ${libbz_headers}
    ${libxz_headers}
    ${liblzma_headers}
    ${libsnapshot_cow_headers}
    ${libfec_headers}
    ${boringssl_headers}
    ${libcrypto_utils_headers}
    ${libevent_headers}
    ${e2fsprogs_lib_headers}
    ${liberofs_headers}
    ${libavb_headers}
    ${bootimg_headers}
    ${fs_mgr_headers}
)
target_link_libraries(payload_generator PUBLIC
        ${common_libs}
        protobuf-cpp-full
        avb
        brotli
        bsdiff
        divsufsort
        divsufsort64
        lzma
        payload_consumer
        puffdiff
        zucchini
        verity_tree
        update_metadata-protos
        payload_extent_utils
        cow_size_estimator
        erofs
        selinux
        lz4diff-protos
        lz4diff
        zstd
        
        base
        cutils
        log
        ext2fs
        lz4
)


add_executable(delta_generator ${delta_generator_srcs})
target_compile_options(delta_generator PRIVATE ${cflags} ${cppflags})
target_include_directories(delta_generator PRIVATE
    ${common_headers}
    ${core_headers}
    ${libbase_headers}
    ${libbsdiff_headers}
    ${puffin_headers}
    ${libzucchini_headers}
    ${zlib_headers}
    ${libbz_headers}
    ${libxz_headers}
    ${liblzma_headers}
    ${libsnapshot_cow_headers}
    ${libfec_headers}
    ${boringssl_headers}
    ${libcrypto_utils_headers}
    ${libevent_headers}
    ${e2fsprogs_lib_headers}
    ${liberofs_headers}
    ${libavb_headers}
    ${bootimg_headers}
    ${libgflags_headers}
    ${fs_mgr_headers}
)
target_link_libraries(delta_generator PRIVATE
    ${common_libs}
    payload_consumer
    payload_generator
    gflags
)
