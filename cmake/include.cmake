set(core_headers "${CMAKE_SOURCE_DIR}/src/system/core/include" CACHE STRING "" FORCE)
set(fmtlib_headers "${CMAKE_SOURCE_DIR}/src/external/fmtlib/include" CACHE STRING "" FORCE)
set(zlib_headers "${CMAKE_SOURCE_DIR}/src/external/zlib" CACHE STRING "" FORCE)
set(libbase_headers "${CMAKE_SOURCE_DIR}/src/system/libbase/include" CACHE STRING "" FORCE)
set(liblz4_headers "${CMAKE_SOURCE_DIR}/src/external/lz4/lib" CACHE STRING "" FORCE)
set(liblzma_headers "${CMAKE_SOURCE_DIR}/src/external/lzma/C" CACHE STRING "" FORCE)
set(libzstd_headers 
    "${CMAKE_SOURCE_DIR}/src/external/zstd/lib" 
    "${CMAKE_SOURCE_DIR}/src/external/zstd/lib/common"
CACHE STRING "" FORCE)
set(avb_headers "${CMAKE_SOURCE_DIR}/src/external/avb" CACHE STRING "" FORCE)
set(libfec_headers 
    "${CMAKE_SOURCE_DIR}/src/system/extras/libfec/include"
    "${CMAKE_SOURCE_DIR}/src/system/extras/libfec"
    "${CMAKE_SOURCE_DIR}/src/external/fec"
 CACHE STRING "" FORCE)
set(libcutils_headers "${CMAKE_SOURCE_DIR}/src/system/core/libcutils/include" CACHE STRING "" FORCE)
set(libutils_headers "${CMAKE_SOURCE_DIR}/src/system/core/libutils/include" CACHE STRING "" FORCE)
set(libchrome_headers "${CMAKE_SOURCE_DIR}/src/external/libchrome" CACHE STRING "" FORCE)
set(boringssl_headers "${CMAKE_SOURCE_DIR}/src/external/boringssl/include" CACHE STRING "" FORCE)

set(liblog_headers
    "${CMAKE_SOURCE_DIR}/src/system/logging/liblog/include"
    "${CMAKE_SOURCE_DIR}/src/system/logging/liblog/include_vndk"
    CACHE STRING "" FORCE
)
set(liblogwrap_headers "${CMAKE_SOURCE_DIR}/src/system/logging/logwrapper/include" CACHE STRING "" FORCE)
set(libselinux_headers 
    "${CMAKE_SOURCE_DIR}/src/external/selinux/libselinux/include" 
    "${CMAKE_SOURCE_DIR}/src/external/selinux/libsepol/include" 
CACHE STRING "" FORCE)
set(libpcre2_headers "${CMAKE_SOURCE_DIR}/src/external/pcre/include" CACHE STRING "" FORCE)
set(libprocessgroup_headers
    "${CMAKE_SOURCE_DIR}/src/system/core/libprocessgroup"
    "${CMAKE_SOURCE_DIR}/src/system/core/libprocessgroup/include"
    "${CMAKE_SOURCE_DIR}/src/system/core/libprocessgroup/cgrouprc/include"
    CACHE STRING "" FORCE)
set(libprocessgroup_util_headers "${CMAKE_SOURCE_DIR}/src/system/core/libprocessgroup/util/include" CACHE STRING "" FORCE)
set(libsparse_headers "${CMAKE_SOURCE_DIR}/src/system/core/libsparse/include" CACHE STRING "" FORCE)
set(libgtest_prod_headers "${CMAKE_SOURCE_DIR}/src/external/googletest/googletest/include" CACHE STRING "" FORCE)
set(libgmock_headers "${CMAKE_SOURCE_DIR}/src/external/googletest/googlemock/include" CACHE STRING "" FORCE)
set(libfscrypt_headers "${CMAKE_SOURCE_DIR}/src/system/extras/libfscrypt/include" CACHE STRING "" FORCE)
set(libmodpb64_headers "${CMAKE_SOURCE_DIR}/src/external/modp_b64" CACHE STRING "" FORCE)
set(libevent_headers 
    "${CMAKE_SOURCE_DIR}/src/external/libevent/include"
    "${CMAKE_SOURCE_DIR}/src/external/libevent/compat"
    CACHE STRING "" FORCE
)
set(e2fsprogs_lib_headers 
    "${CMAKE_SOURCE_DIR}/src/external/e2fsprogs/e2fsck"
    "${CMAKE_SOURCE_DIR}/src/external/e2fsprogs/lib"
    "${CMAKE_SOURCE_DIR}/src/external/e2fsprogs/lib/blkid"
    "${CMAKE_SOURCE_DIR}/src/external/e2fsprogs/lib/e2p"
    "${CMAKE_SOURCE_DIR}/src/external/e2fsprogs/lib/et"
    "${CMAKE_SOURCE_DIR}/src/external/e2fsprogs/lib/ext2fs"
    "${CMAKE_SOURCE_DIR}/src/external/e2fsprogs/lib/ss"
    "${CMAKE_SOURCE_DIR}/src/external/e2fsprogs/lib/support"
    "${CMAKE_SOURCE_DIR}/src/external/e2fsprogs/lib/uuid"
CACHE STRING "" FORCE)
set(libext4_utils_headers 
    "${CMAKE_SOURCE_DIR}/src/system/extras/ext4_utils"
    "${CMAKE_SOURCE_DIR}/src/system/extras/ext4_utils/include"
    CACHE STRING "" FORCE)
set(libcrypto_utils_headers 
    "${CMAKE_SOURCE_DIR}/src/system/core/libcrypto_utils" 
    "${CMAKE_SOURCE_DIR}/src/system/core/libcrypto_utils/include"
    CACHE STRING "" FORCE)
set(libsquashfs_utils_headers 
    "${CMAKE_SOURCE_DIR}/src/system/extras/squashfs_utils"
    CACHE STRING "" FORCE)
set(squashfs_headers "${CMAKE_SOURCE_DIR}/src/external/squashfs-tools/squashfs-tools" CACHE STRING "" FORCE)
set(bootimg_headers "${CMAKE_SOURCE_DIR}/src/system/tools/mkbootimg/include/bootimg" CACHE STRING "" FORCE)
set(libdivsufsort_headers 
    "${CMAKE_SOURCE_DIR}/src/external/libdivsufsort/include"
    "${CMAKE_SOURCE_DIR}/src/external/libdivsufsort/android_include"
    CACHE STRING "" FORCE)
set(libbrotli_headers "${CMAKE_SOURCE_DIR}/src/external/brotli/c/include" CACHE STRING "" FORCE)
set(libbrillo_headers "${CMAKE_SOURCE_DIR}/src/external/libbrillo" CACHE STRING "" FORCE)
set(libbz_headers "${CMAKE_SOURCE_DIR}/src/external/bzip2" CACHE STRING "" FORCE)
set(libxz_headers 
    "${CMAKE_SOURCE_DIR}/src/external/xz-embedded/userspace"
    "${CMAKE_SOURCE_DIR}/src/external/xz-embedded/linux/include/linux"
    CACHE STRING "" FORCE)
set(libsparse_headers "${CMAKE_SOURCE_DIR}/src/system/core/libsparse/include" CACHE STRING "" FORCE)
set(protobuf_headers 
    "${CMAKE_SOURCE_DIR}/src/external/protobuf/src"
    "${CMAKE_SOURCE_DIR}/src/external/protobuf/third_party"
    "${CMAKE_SOURCE_DIR}/src/external/protobuf/third_party/utf8_range"
    "${CMAKE_SOURCE_DIR}/src/external/abseil-cpp"
    CACHE STRING "" FORCE)
set(absl_headers
    "${CMAKE_SOURCE_DIR}/src/external/abseil-cpp"
    CACHE STRING "" FORCE)
set(libzucchini_headers
    "${CMAKE_SOURCE_DIR}/src/external/zucchini/aosp/include/components"
    "${CMAKE_SOURCE_DIR}/src/external/zucchini/aosp/include"
    CACHE STRING "" FORCE)
set(libbsdiff_headers "${CMAKE_SOURCE_DIR}/src/external/bsdiff/include" CACHE STRING "" FORCE)
set(libjsoncpp_headers "${CMAKE_SOURCE_DIR}/src/external/jsoncpp/include" CACHE STRING "" FORCE)
set(libgsi_headers
    "${CMAKE_SOURCE_DIR}/src/system/gsid"
    "${CMAKE_SOURCE_DIR}/src/system/gsid/include"
    CACHE STRING "" FORCE)
set(libavb_headers 
    "${CMAKE_SOURCE_DIR}/src/external/avb"
    "${CMAKE_SOURCE_DIR}/src/external/avb/libavb"
    CACHE STRING "" FORCE)
set(gsid_headers "${CMAKE_SOURCE_DIR}/src/system/gsid/include" CACHE STRING "" FORCE)
set(puffin_headers "${CMAKE_SOURCE_DIR}/src/external/puffin/src/include" CACHE STRING "" FORCE)
set(update_engine_headers "${CMAKE_SOURCE_DIR}/src/system/update_engine" CACHE STRING "" FORCE)
set(fs_mgr_headers
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/libstorage_literals"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/include"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/libsnapshot"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/libsnapshot/include"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/libsnapshot/snapuserd/include"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/libsnapshot/libsnapshot_cow"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/liblp/include"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/libfstab/include"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/libfs_avb/include"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/libdm/include"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/libfiemap/include"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/libdm/flags/include"
    "${CMAKE_SOURCE_DIR}/src/system/fs/fs_mgr/libsnapshot/flags/include"
    CACHE STRING "" FORCE)
set(libziparchive_headers 
    "${CMAKE_SOURCE_DIR}/src/system/libziparchive/include"
    "${CMAKE_SOURCE_DIR}/src/system/libziparchive/incfs_support/include"
    CACHE STRING "" FORCE
)
set(libverity_tree_headers "${CMAKE_SOURCE_DIR}/src/system/extras/verity/include")
set(liberofs_headers 
    "${CMAKE_SOURCE_DIR}/src/external/erofs-utils/include"
    "${CMAKE_SOURCE_DIR}/src/external/erofs-utils/lib"
    CACHE STRING "" FORCE
)
set(libgflags_headers
    "${CMAKE_SOURCE_DIR}/src/external/gflags"
    "${CMAKE_SOURCE_DIR}/src/external/gflags/android"
    CACHE STRING "" FORCE
)