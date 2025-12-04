set(libcutils_dir "${CMAKE_SOURCE_DIR}/src/core/libcutils")

set(libcutils_flags
    "-Wno-exit-time-destructors"
)

set(libcutils_sockets_srcs
    "${libcutils_dir}/sockets.cpp"
)

if(NOT WIN32)
    if(CMAKE_SYSTEM_NAME STREQUAL "Android")
        list(APPEND libcutils_sockets_srcs
            "${libcutils_dir}/android_get_control_file.cpp"
            "${libcutils_dir}/socket_inaddr_any_server_unix.cpp"
            "${libcutils_dir}/socket_local_client_unix.cpp"
            "${libcutils_dir}/socket_local_server_unix.cpp"
            "${libcutils_dir}/socket_network_client_unix.cpp"
            "${libcutils_dir}/sockets_unix.cpp"
        )
    else()
        list(APPEND libcutils_sockets_srcs
            "${libcutils_dir}/socket_inaddr_any_server_unix.cpp"
            "${libcutils_dir}/socket_local_client_unix.cpp"
            "${libcutils_dir}/socket_local_server_unix.cpp"
            "${libcutils_dir}/socket_network_client_unix.cpp"
            "${libcutils_dir}/sockets_unix.cpp"
        )
    endif()
else()
    list(APPEND libcutils_sockets_srcs
        "${libcutils_dir}/socket_inaddr_any_server_windows.cpp"
        "${libcutils_dir}/socket_network_client_windows.cpp"
        "${libcutils_dir}/sockets_windows.cpp"
    )
    list(APPEND libcutils_sockets_flags "-D_GNU_SOURCE")
endif()

set(libcutils_srcs
    "${libcutils_dir}/config_utils.cpp"
    "${libcutils_dir}/iosched_policy.cpp"
    "${libcutils_dir}/load_file.cpp"
    "${libcutils_dir}/native_handle.cpp"
    "${libcutils_dir}/properties.cpp"
    "${libcutils_dir}/record_stream.cpp"
    "${libcutils_dir}/strlcpy.c"
)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    list(APPEND libcutils_srcs
        "${libcutils_dir}/canned_fs_config.cpp"
        "${libcutils_dir}/fs_config.cpp"
    )
endif()


# # HOST
# list(APPEND libcutils_srcs
#     "${libcutils_dir}/trace-host.cpp"
#     "${libcutils_dir}/ashmem-host.cpp"
# )


if(NOT WIN32)
    list(APPEND libcutils_srcs
        "${libcutils_dir}/fs.cpp"
        "${libcutils_dir}/hashmap.cpp"
        "${libcutils_dir}/multiuser.cpp"
        "${libcutils_dir}/str_parms.cpp"
    )
endif()

if (BUILD_ANDROID)
    list(APPEND libcutils_srcs
        "${libcutils_dir}/android_reboot.cpp"
        "${libcutils_dir}/ashmem-dev.cpp"
        "${libcutils_dir}/klog.cpp"
        "${libcutils_dir}/partition_utils.cpp"
        "${libcutils_dir}/qtaguid.cpp"
        "${libcutils_dir}/trace-dev.cpp"
        "${libcutils_dir}/uevent.cpp"
    )
endif()

add_library(cutils_sockets STATIC ${libcutils_sockets_srcs})
target_include_directories(cutils_sockets PUBLIC
    ${libbase_headers}
    ${libcutils_headers}
    ${liblog_headers}
)
target_link_directories(cutils_sockets PUBLIC
    base
    log
)
target_compile_options(cutils_sockets PRIVATE ${libcutils_flags})
if(WIN32)
    target_link_libraries(cutils_sockets PRIVATE ws2_32)
endif()

add_library(cutils STATIC ${libcutils_srcs})
target_compile_options(cutils PRIVATE ${libcutils_flags})
target_include_directories(cutils PUBLIC
    ${libprocessgroup_headers}
    ${libcutils_headers}
    ${libbase_headers}
    ${liblog_headers}
    ${core_headers}
)
target_link_directories(cutils PUBLIC
    processgroup
    base
    log
)