use crate::libsign::utils::error;
use crate::libsign::utils::ffi;
use crate::libsign::utils::config::{MLEN, 
                                    CRYPTO_BYTES,
                                    CRYPTO_PUBLICKEYBYTES, 
                                    CRYPTO_SECRETKEYBYTES};


#[repr(C)]
#[derive(Debug,Clone)]
pub struct Message([u8; MLEN]);

#[repr(C)]
#[derive(Debug,Clone)]
pub struct Signed_message{
    pub smlen: usize,
    pub signed_message_data:[u8; MLEN+CRYPTO_BYTES],
}



#[repr(C)]
#[derive(Debug,Clone)]
pub struct Public_key([u8; CRYPTO_PUBLICKEYBYTES]);

#[repr(C)]
#[derive(Debug,Clone)]
pub struct Secret_key([u8; CRYPTO_SECRETKEYBYTES]);

#[repr(C)]
#[derive(Debug,Clone)]
pub struct Key_pair{
    pub public_key: Public_key, 
    pub secret_key: Secret_key,
}

pub fn get_key_pair() -> Result<Key_pair, error::Error>{
    let mut pk_data: [u8; CRYPTO_PUBLICKEYBYTES] = [0; CRYPTO_PUBLICKEYBYTES];
    let mut sk_data: [u8; CRYPTO_SECRETKEYBYTES] = [0; CRYPTO_SECRETKEYBYTES];
    
    unsafe{
        ffi::pqcrystals_dilithium2_ref_keypair(pk_data.as_mut_ptr(), sk_data.as_mut_ptr());
    }
    let pk = Public_key(pk_data);
    let sk = Secret_key(sk_data);
    Ok(Key_pair{public_key: pk, secret_key: sk})  
}

pub fn generate_message() -> Result<Message, error::Error>{
    let mut m: [u8; MLEN] = [0; MLEN]; 
    unsafe{
        ffi::randombytes(m.as_mut_ptr(), MLEN);
    }
    Ok(Message(m))
}

pub fn sign(mut m: Message, mut sk: Secret_key) -> Result<Signed_message, error::Error>{
    let mut sm: [u8; MLEN+CRYPTO_BYTES] = [0; MLEN+CRYPTO_BYTES];
    let mut smlen: usize = 0;

    unsafe{
        ffi::pqcrystals_dilithium2_ref(sm.as_mut_ptr(), 
                            &mut smlen as *mut usize, 
                            m.0.as_mut_ptr(), 
                            MLEN, 
                            sk.0.as_mut_ptr());
        // println!("smlen:{:?}",smlen);
    }
    Ok(Signed_message{smlen:smlen, signed_message_data:sm})
}

pub fn verify_sign(m: Message, mut sm: Signed_message, mut pk: Public_key) -> Result<i32, error::Error>{
    let result;
    let mut m2: [u8; MLEN+CRYPTO_BYTES] = [0; MLEN+CRYPTO_BYTES];

    let mut mlen: usize = 0;
    let smlen: usize = sm.smlen.clone();

    unsafe{
        result = ffi::pqcrystals_dilithium2_ref_open(m2.as_mut_ptr(), 
                                        &mut mlen as *mut usize,
                                        sm.signed_message_data.as_mut_ptr(), 
                                        smlen, 
                                        pk.0.as_mut_ptr())
    }


    if result == 1{
        return Err(error::Error::FailedVerification)
    }

    if mlen != MLEN{
        println!("mlen:{:?} MLEN:{:?}", mlen, MLEN);
        return Err(error::Error::InvalidMessageLength)
    }

    for j in 0..mlen {
        if m.0[j] != m2[j]{
            println!("m.0[j]:{:?} m2[j]:{:?}", m.0[j], m2[j]);
            return Err(error::Error::InvalidMatch)
        }
    }

    Ok(result)
}


  