
fn main() {
    let sources = [
        "depend/dilithium2/sign.c",
        "depend/dilithium2/packing.c",
        "depend/dilithium2/polyvec.c",
        "depend/dilithium2/poly.c",
        "depend/dilithium2/ntt.c",
        "depend/dilithium2/reduce.c",
        "depend/dilithium2/rounding.c",
        "depend/dilithium2/fips202.c",
        "depend/dilithium2/symmetric-shake.c",
        "depend/dilithium2/randombytes.c"
    ];
    let headers = [
        "depend/dilithium2/config.h",
        "depend/dilithium2/params.h",
        "depend/dilithium2/api.h",
        "depend/dilithium2/sign.h",
        "depend/dilithium2/packing.h",
        "depend/dilithium2/polyvec.h",
        "depend/dilithium2/poly.h",
        "depend/dilithium2/ntt.h",
        "depend/dilithium2/reduce.h",
        "depend/dilithium2/rounding.h",
        "depend/dilithium2/symmetric.h",
        "depend/dilithium2/randombytes.h",
        "depend/dilithium2/fips202.h"
    ];

    let mut build = cc::Build::new();

    build.shared_flag(true)
        .pic(true)
        .define("DILITHIUM_MODE", Some("2"))
        .files(&sources);
        //.file("depend/dilithium2/symmetric-shake.c");

    //build.include("depend/dilithium2");
    //for header in &headers {
        //build.include(header);
    //}    
        
    build.compile("libpqcrystals_dilithium2_ref.so");
    println!("Hello, world!");

    // libpqcrystals_dilithium2_ref.so: $(SOURCES) $(HEADERS) symmetric-shake.c
	// $(CC) -shared -fPIC $(CFLAGS) -DDILITHIUM_MODE=2 \
	//   -o $@ $(SOURCES) symmetric-shake.c

}
