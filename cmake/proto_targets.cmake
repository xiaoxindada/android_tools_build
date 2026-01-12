set(partition_tools_dir "${CMAKE_SOURCE_DIR}/src/extras/partition_tools")
set(fs_mgr_dir "${CMAKE_SOURCE_DIR}/src/core/fs_mgr")
set(libsnapshot_srcs_dir "${fs_mgr_dir}/libsnapshot")
set(puffin_dir "${CMAKE_SOURCE_DIR}/src/puffin")
set(update_engine_dir "${CMAKE_SOURCE_DIR}/src/update_engine")

set(puffin_protos_srcs
    "${puffin_dir}/src/puffin.pb.cc"
    "${puffin_dir}/src/puffin.pb.h"
)

set(update_metadata_protos_srcs
    "${update_engine_dir}/update_engine/update_metadata.pb.cc"
    "${update_engine_dir}/update_engine/update_metadata.pb.h"
)

set(lz4diff_protos_srcs
    "${update_engine_dir}/lz4diff/lz4diff.pb.cc"
    "${update_engine_dir}/lz4diff/lz4diff.pb.h"
)

set(dynamic_partitions_device_info_srcs
    "${partition_tools_dir}/dynamic_partitions_device_info.pb.cc"
    "${partition_tools_dir}/dynamic_partitions_device_info.pb.h"
)

set(snapshot_protos_srcs
    "${libsnapshot_srcs_dir}/android/snapshot/snapshot.pb.cc"
    "${libsnapshot_srcs_dir}/android/snapshot/snapshot.pb.h"
)

add_custom_command(
    OUTPUT
        ${dynamic_partitions_device_info_srcs}
    COMMAND
        ${prebuilt_protoc} dynamic_partitions_device_info.proto -I${partition_tools_dir} --cpp_out=${partition_tools_dir} 
    DEPENDS
        ${partition_tools_dir}/dynamic_partitions_device_info.proto
)
add_custom_target(gen_dynamic_partitions_device_info_srcs ALL DEPENDS ${dynamic_partitions_device_info_srcs})

add_custom_command(
    OUTPUT
        ${snapshot_protos_srcs}
    COMMAND
        ${prebuilt_protoc} snapshot.proto -I${libsnapshot_srcs_dir}/android/snapshot --cpp_out=${libsnapshot_srcs_dir}/android/snapshot
    DEPENDS
        ${libsnapshot_srcs_dir}/android/snapshot/snapshot.proto
)
add_custom_target(gen_snapshot_protos_srcs ALL DEPENDS ${snapshot_protos_srcs})

add_custom_command(
    OUTPUT
        ${puffin_protos_srcs}
    COMMAND
        ${prebuilt_protoc} puffin.proto -I${puffin_dir}/src --cpp_out=${puffin_dir}/src
    DEPENDS
        ${puffin_dir}/src/puffin.proto
)
add_custom_target(gen_puffin_protos_srcs ALL DEPENDS ${puffin_protos_srcs})

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
target_compile_options(lz4diff-protos PRIVATE "-Wall")
target_include_directories(lz4diff-protos PUBLIC 
    ${protobuf_headers}
    ${absl_headers}
)