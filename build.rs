fn main() {
    let sources = [
        "depends/dilithium2/sign.c",
        "depends/dilithium2/packing.c",
        "depends/dilithium2/polyvec.c",
        "depends/dilithium2/poly.c",
        "depends/dilithium2/ntt.c",
        "depends/dilithium2/reduce.c",
        "depends/dilithium2/rounding.c",
        "depends/dilithium2/fips202.c",
        "depends/dilithium2/symmetric-shake.c",
        "depends/dilithium2/randombytes.c",
    ];
    let _headers = [
        "depends/dilithium2/config.h",
        "depends/dilithium2/params.h",
        "depends/dilithium2/api.h",
        "depends/dilithium2/sign.h",
        "depends/dilithium2/packing.h",
        "depends/dilithium2/polyvec.h",
        "depends/dilithium2/poly.h",
        "depends/dilithium2/ntt.h",
        "depends/dilithium2/reduce.h",
        "depends/dilithium2/rounding.h",
        "depends/dilithium2/symmetric.h",
        "depends/dilithium2/randombytes.h",
        "depends/dilithium2/fips202.h",
    ];

    let mut build = cc::Build::new();

    build
        .shared_flag(true)
        .pic(true)
        .define("DILITHIUM_MODE", Some("2"))
        .files(&sources);
    //.file("depends/dilithium2/symmetric-shake.c");

    //build.include("depends/dilithium2");
    //for header in &headers {
    //build.include(header);
    //}

    build.compile("libpqcrystals_dilithium2_ref.so");
    // println!("Hello, world!");

    // libpqcrystals_dilithium2_ref.so: $(SOURCES) $(HEADERS) symmetric-shake.c
    // $(CC) -shared -fPIC $(CFLAGS) -DDILITHIUM_MODE=2 \
    //   -o $@ $(SOURCES) symmetric-shake.c
    println!("cargo:rerun-if-changed=src/hello.c");
}
