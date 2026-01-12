set(fs_mgr_dir "${CMAKE_SOURCE_DIR}/src/core/fs_mgr")
set(libfiemap_srcs_dir "${CMAKE_SOURCE_DIR}/src/core/fs_mgr/libfiemap")
set(libsnapshot_srcs_dir "${fs_mgr_dir}/libsnapshot")
set(libsnapshot_cow_srcs_dir "${fs_mgr_dir}/libsnapshot/libsnapshot_cow")
set(libdm_srcs_dir "${fs_mgr_dir}/libdm")
set(liblp_srcs_dir "${fs_mgr_dir}/liblp")
set(libfstab_srcs_dir "${fs_mgr_dir}/libfstab")
set(libfs_avb_srcs_dir "${fs_mgr_dir}/libfs_avb")
set(update_engine_dir "${CMAKE_SOURCE_DIR}/src/update_engine")

set(cflags
        "-D_FILE_OFFSET_BITS=64"
        "-Wall"
        "-Wno-inconsistent-missing-override"
        "-DALLOW_ADBD_DISABLE_VERITY=1"
)

set(libfiemap_srcs
        "${libfiemap_srcs_dir}/fiemap_writer.cpp"
        "${libfiemap_srcs_dir}/fiemap_status.cpp"
        "${libfiemap_srcs_dir}/image_manager.cpp"
        "${libfiemap_srcs_dir}/metadata.cpp"
        "${libfiemap_srcs_dir}/split_fiemap_writer.cpp"
        "${libfiemap_srcs_dir}/utility.cpp"
)

set(libfiemap_binder_srcs
    "${libfiemap_srcs_dir}/binder.cpp"
)

set(libfs_mgr_binder_srcs 
        "${fs_mgr_dir}/blockdev.cpp"
        "${fs_mgr_dir}/file_wait.cpp"
        "${fs_mgr_dir}/fs_mgr.cpp"
        "${fs_mgr_dir}/fs_mgr_format.cpp"
        "${fs_mgr_dir}/fs_mgr_dm_linear.cpp"
        "${fs_mgr_dir}/fs_mgr_roots.cpp"
        "${fs_mgr_dir}/fs_mgr_overlayfs_control.cpp"
        "${fs_mgr_dir}/fs_mgr_overlayfs_mount.cpp"
        "${fs_mgr_dir}/fs_mgr_vendor_overlay.cpp"
       ${libfiemap_srcs}
       ${libfiemap_binder_srcs}
)

set(libdm_srcs
        "${libdm_srcs_dir}/dm_table.cpp"
        "${libdm_srcs_dir}/dm_target.cpp"
        "${libdm_srcs_dir}/dm.cpp"
        "${libdm_srcs_dir}/loop_control.cpp"
        "${libdm_srcs_dir}/utility.cpp"
)

set(libfstab_srcs 
        "${libfstab_srcs_dir}/fstab.cpp"
        "${libfstab_srcs_dir}/boot_config.cpp"
        "${libfstab_srcs_dir}/slotselect.cpp"
)

set(libfs_avb_srcs 
        "${libfs_avb_srcs_dir}/avb_ops.cpp"
        "${libfs_avb_srcs_dir}/avb_util.cpp"
        "${libfs_avb_srcs_dir}/fs_avb.cpp"
        "${libfs_avb_srcs_dir}/fs_avb_util.cpp"
        "${libfs_avb_srcs_dir}/types.cpp"
        "${libfs_avb_srcs_dir}/util.cpp"
)

set(liblp_srcs
        "${liblp_srcs_dir}/builder.cpp"
        "${liblp_srcs_dir}/super_layout_builder.cpp"
        "${liblp_srcs_dir}/images.cpp"
        "${liblp_srcs_dir}/partition_opener.cpp"
        "${liblp_srcs_dir}/property_fetcher.cpp"
        "${liblp_srcs_dir}/reader.cpp"
        "${liblp_srcs_dir}/utility.cpp"
        "${liblp_srcs_dir}/writer.cpp"
)

set(libsnapshot_capabilities_sources
        "${libsnapshot_srcs_dir}/capabilities.cpp"
 )

set(libsnapshot_srcs
        # ${libsnapshot_capabilities_sources}
        ${snapshot_protos_srcs}
        "${libsnapshot_srcs_dir}/device_info.cpp"
        "${libsnapshot_srcs_dir}/snapshot.cpp"
        "${libsnapshot_srcs_dir}/snapshot_stats.cpp"
        "${libsnapshot_srcs_dir}/snapshot_stub.cpp"
        "${libsnapshot_srcs_dir}/snapshot_metadata_updater.cpp"
        "${libsnapshot_srcs_dir}/partition_cow_creator.cpp"
        "${libsnapshot_srcs_dir}/return.cpp"
        # "${libsnapshot_srcs_dir}/utility.cpp"
        "${libsnapshot_srcs_dir}/scratch_super.cpp"
)

set(libsnapshot_cow_srcs
        "${libsnapshot_cow_srcs_dir}/cow_compress.cpp"
        "${libsnapshot_cow_srcs_dir}/cow_decompress.cpp"
        "${libsnapshot_cow_srcs_dir}/cow_format.cpp"
        "${libsnapshot_cow_srcs_dir}/cow_reader.cpp"
        "${libsnapshot_cow_srcs_dir}/parser_v2.cpp"
        "${libsnapshot_cow_srcs_dir}/parser_v3.cpp"
        "${libsnapshot_cow_srcs_dir}/snapshot_reader.cpp"
        "${libsnapshot_cow_srcs_dir}/writer_base.cpp"
        "${libsnapshot_cow_srcs_dir}/writer_v2.cpp"
        "${libsnapshot_cow_srcs_dir}/writer_v3.cpp"
)

add_library(dm STATIC ${libdm_srcs})
target_compile_options(dm PRIVATE ${cflags})
target_include_directories(dm PUBLIC
        ${fs_mgr_headers}
        ${libbase_headers}
        ${liblog_headers}
        ${e2fsprogs_lib_headers}
)
target_link_libraries(dm PUBLIC
        base
        log
        ext2_uuid
)

add_library(fstab STATIC ${libfstab_srcs})
target_compile_options(fstab PRIVATE ${cflags})
target_include_directories(fstab PUBLIC
        ${fs_mgr_headers}
        ${libbase_headers}
        ${libgsi_headers}
)
target_link_libraries(fstab PUBLIC
        base
        gsi
)

add_library(fs_avb STATIC ${libfs_avb_srcs})
target_compile_options(fs_avb PRIVATE ${cflags})
target_include_directories(fs_avb PUBLIC
        ${fs_mgr_headers}
        ${libavb_headers}
        ${libbase_headers}
        ${libgsi_headers}
        ${boringssl_headers}
)
target_link_libraries(fs_avb PUBLIC
        avb
        dm
        gsi
        fstab
        base
        crypto
)

add_library(lp STATIC ${liblp_srcs})
target_compile_options(lp PRIVATE ${cflags})
target_include_directories(lp PUBLIC 
        ${fs_mgr_headers}
        ${libbase_headers}
        ${libcutils_headers}
        ${boringssl_headers}
        ${liblog_headers}
        ${libsparse_headers}
        ${e2fsprogs_lib_headers}
        ${zlib_headers}
)
target_link_libraries(lp PUBLIC 
        crypto
        cutils
        base
        log
        crypto_utils
        sparse
        ext4_utils
        zlib
)

# add_library(fs_mgr_binder STATIC ${libfs_mgr_binder_srcs})
# target_compile_options(fs_mgr_binder PRIVATE ${cflags})
# target_include_directories(fs_mgr_binder PUBLIC
#         ${fs_mgr_headers}
#         ${libbase_headers}
#         ${liblog_headers}
# )
# target_link_libraries(fs_mgr_binder PUBLIC
#         base
#         crypto
#         crypto_utils
#         cutils
#         ext4_utils
#         fec
#         log
#         lp
#         selinux
#         avb
#         fs_avb
#         gsi
# )

add_library(snapshot STATIC ${libsnapshot_srcs})
target_compile_options(snapshot PRIVATE ${cflags})
target_include_directories(snapshot PUBLIC
        ${fs_mgr_headers}
        ${libbase_headers}
        ${liblog_headers}
        ${libchrome_headers}
        ${libcutils_headers}
        ${update_engine_dir}
        ${libext4_utils_headers}
        ${libselinux_headers}
)
target_link_libraries(snapshot PUBLIC
        base
        chrome
        cutils
        log
        dm
        fstab
        # snapshot_flags_cc_lib
        update_metadata-protos
        # fs_mgr_binder
        selinux
)

add_library(snapshot_cow STATIC ${libsnapshot_cow_srcs})
target_compile_options(snapshot_cow PRIVATE 
        ${cflags}
        # "-DLIBSNAPSHOT_USE_HAL"
)
target_include_directories(snapshot_cow PUBLIC
        ${fs_mgr_headers}
        ${libbase_headers}
        ${liblog_headers}
        ${libbrotli_headers}
        ${liblz4_headers}
        ${libzstd_headers}
        ${update_engine_headers}
        ${libselinux_headers}
)
target_link_libraries(snapshot_cow PUBLIC
        base
        log
        brotli
        zlib
        lz4
        zstd
)
