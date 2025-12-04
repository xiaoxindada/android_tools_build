set(target_name "event")

set(libevent_dir "${CMAKE_SOURCE_DIR}/src/libevent")

set(cflags
        "-D_DEFAULT_SOURCE"
        "-Wno-strict-aliasing"
        "-Wno-unused-parameter"
        "-Wno-tautological-compare"
        "-UEVENT__HAVE_SYS_SYSCTL_H"
)

set(libevent_srcs
        # core
        "${libevent_dir}/buffer.c"
        "${libevent_dir}/bufferevent.c"
        "${libevent_dir}/bufferevent_filter.c"
        "${libevent_dir}/bufferevent_pair.c"
        "${libevent_dir}/bufferevent_ratelim.c"
        "${libevent_dir}/bufferevent_sock.c"
        "${libevent_dir}/event.c"
        "${libevent_dir}/evmap.c"
        "${libevent_dir}/evthread.c"
        "${libevent_dir}/evthread_pthread.c"
        "${libevent_dir}/evutil.c"
        "${libevent_dir}/evutil_rand.c"
        "${libevent_dir}/evutil_time.c"
        "${libevent_dir}/listener.c"
        "${libevent_dir}/log.c"
        "${libevent_dir}/signal.c"
        "${libevent_dir}/strlcpy.c"

        # extra
        "${libevent_dir}/evdns.c"
        "${libevent_dir}/event_tagging.c"
        "${libevent_dir}/evrpc.c"
        "${libevent_dir}/http.c"

        "${libevent_dir}/poll.c"
        "${libevent_dir}/select.c"
)

if (CMAKE_SYSTEM_NAME EQUAL "Android")
    list(APPEND libevent_srcs "${libevent_dir}/epoll.c")
    list(APPEND cflags "-D_GNU_SOURCE=1")
else()
    list(APPEND libevent_srcs "${libevent_dir}/epoll.c")
    list(APPEND cflags
        "-D_GNU_SOURCE=1"
        "-DANDROID_HOST_MUSL=1"
        "-DEVENT__HAVE_ARC4RANDOM=1"
    )
endif()

add_library(${target_name} STATIC ${libevent_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${libevent_dir}
    ${libevent_headers}
)