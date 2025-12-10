set(target_name "pcre2")

set(cflags
    "-DHAVE_CONFIG_H"
    "-Wall"
    "-DPCRE2_CODE_UNIT_WIDTH=8"
)

set(pcre_dir "${CMAKE_SOURCE_DIR}/src/pcre")

set(target_srcs 
        "${pcre_dir}/src/pcre2_auto_possess.c"
        "${pcre_dir}/src/pcre2_chartables.c"
        "${pcre_dir}/src/pcre2_chkdint.c"
        "${pcre_dir}/src/pcre2_compile.c"
        "${pcre_dir}/src/pcre2_compile_class.c"
        "${pcre_dir}/src/pcre2_config.c"
        "${pcre_dir}/src/pcre2_context.c"
        "${pcre_dir}/src/pcre2_convert.c"
        "${pcre_dir}/src/pcre2_dfa_match.c"
        "${pcre_dir}/src/pcre2_error.c"
        "${pcre_dir}/src/pcre2_extuni.c"
        "${pcre_dir}/src/pcre2_find_bracket.c"
        "${pcre_dir}/src/pcre2_jit_compile.c"
        "${pcre_dir}/src/pcre2_maketables.c"
        "${pcre_dir}/src/pcre2_match_data.c"
        "${pcre_dir}/src/pcre2_match.c"
        "${pcre_dir}/src/pcre2_newline.c"
        "${pcre_dir}/src/pcre2_ord2utf.c"
        "${pcre_dir}/src/pcre2_pattern_info.c"
        "${pcre_dir}/src/pcre2_script_run.c"
        "${pcre_dir}/src/pcre2_serialize.c"
        "${pcre_dir}/src/pcre2_string_utils.c"
        "${pcre_dir}/src/pcre2_study.c"
        "${pcre_dir}/src/pcre2_substitute.c"
        "${pcre_dir}/src/pcre2_substring.c"
        "${pcre_dir}/src/pcre2_tables.c"
        "${pcre_dir}/src/pcre2_ucd.c"
        "${pcre_dir}/src/pcre2_valid_utf.c"
        "${pcre_dir}/src/pcre2_xclass.c"
)

add_library(${target_name} STATIC ${target_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC ${libpcre2_headers})