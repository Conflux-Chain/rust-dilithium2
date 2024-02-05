use libc::{size_t, c_int};

extern "C" {
    pub fn pqcrystals_dilithium2_ref_keypair(pk: *mut u8, sk: *mut u8) -> c_int;
    pub fn pqcrystals_dilithium2_ref(sm: *mut u8,
                    smlen: *mut size_t,
                    m: *const u8, 
                    mlen: size_t, 
                    sk: *const u8);
    pub fn pqcrystals_dilithium2_ref_open(m: *mut u8, 
                    mlen: *mut size_t,
                    sm: *const u8,
                    smlen: size_t, 
                    pk: *const u8) -> c_int;
    pub fn randombytes(out: *mut u8, outlen: usize);
    //pub fn hello();
}

// int crypto_sign_keypair(uint8_t *pk, uint8_t *sk)
/* 
int crypto_sign(uint8_t *sm,
                size_t *smlen,
                const uint8_t *m,
                size_t mlen,
                const uint8_t *sk)
*/
/*
int crypto_sign_open(uint8_t *m,
                     size_t *mlen,
                     const uint8_t *sm,
                     size_t smlen,
                     const uint8_t *pk)
*/
// void randombytes(uint8_t *out, size_t outlen);