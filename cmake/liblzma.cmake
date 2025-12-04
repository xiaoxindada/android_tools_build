set(target_name "lzma")

set(lzma_dir "${CMAKE_SOURCE_DIR}/src/lzma")

set(liblzma_srcs
 "${lzma_dir}/C/7zAlloc.c"
        "${lzma_dir}/C/7zArcIn.c"
        "${lzma_dir}/C/7zBuf2.c"
        "${lzma_dir}/C/7zBuf.c"
        "${lzma_dir}/C/7zCrc.c"
        "${lzma_dir}/C/7zCrcOpt.c"
        "${lzma_dir}/C/7zDec.c"
        "${lzma_dir}/C/7zFile.c"
        "${lzma_dir}/C/7zStream.c"
        "${lzma_dir}/C/Aes.c"
        "${lzma_dir}/C/AesOpt.c"
        "${lzma_dir}/C/Alloc.c"
        "${lzma_dir}/C/Bcj2.c"
        "${lzma_dir}/C/Bra86.c"
        "${lzma_dir}/C/Bra.c"
        "${lzma_dir}/C/BraIA64.c"
        "${lzma_dir}/C/CpuArch.c"
        "${lzma_dir}/C/Delta.c"
        "${lzma_dir}/C/LzFind.c"
        "${lzma_dir}/C/Lzma2Dec.c"
        "${lzma_dir}/C/Lzma2Enc.c"
        "${lzma_dir}/C/Lzma86Dec.c"
        "${lzma_dir}/C/Lzma86Enc.c"
        "${lzma_dir}/C/LzmaDec.c"
        "${lzma_dir}/C/LzmaEnc.c"
        "${lzma_dir}/C/LzmaLib.c"
        "${lzma_dir}/C/Ppmd7.c"
        "${lzma_dir}/C/Ppmd7Dec.c"
        "${lzma_dir}/C/Ppmd7Enc.c"
        "${lzma_dir}/C/Sha256.c"
        "${lzma_dir}/C/Sha256Opt.c"
        "${lzma_dir}/C/Sort.c"
        "${lzma_dir}/C/Xz.c"
        "${lzma_dir}/C/XzCrc64.c"
        "${lzma_dir}/C/XzCrc64Opt.c"
        "${lzma_dir}/C/XzDec.c"
        "${lzma_dir}/C/XzEnc.c"
        "${lzma_dir}/C/XzIn.c"
)

set(cflags 
        "-DZ7_ST"
        "-Wall"
        "-Wno-empty-body"
        "-Wno-enum-conversion"
        "-Wno-logical-op-parentheses"
        "-Wno-self-assign"
)

add_library(${target_name} STATIC ${liblzma_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC 
        ${liblzma_headers}
)