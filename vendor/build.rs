fn main() {
    cc::Build::new()
        .file("vendor/codec/EndianPortable.c")
        .file("vendor/codec/ALACBitUtilities.c")
        .file("vendor/codec/ALACDecoder.cpp")
        .file("vendor/codec/ALACEncoder.cpp")
        .file("vendor/codec/ag_dec.c")
        .file("vendor/codec/ag_enc.c")
        .file("vendor/codec/dp_dec.c")
        .file("vendor/codec/dp_enc.c")
        .file("vendor/codec/matrix_dec.c")
        .file("vendor/codec/matrix_enc.c")
        .compile("alac");
}
