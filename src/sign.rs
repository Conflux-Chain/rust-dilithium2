use super::utils::config::{CRYPTO_BYTES, CRYPTO_PUBLICKEYBYTES, CRYPTO_SECRETKEYBYTES, MLEN};
use super::utils::error;
use super::utils::ffi;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Message{
    pub data: Vec<u8>,
}
//([u8; MLEN]);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SignedMessage {
    pub smlen: usize,
    pub data: Vec<u8>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PublicKey{
    pub data: Vec<u8>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SecretKey{
    data: Vec<u8>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct KeyPair {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

pub fn get_key_pair() -> Result<KeyPair, error::Error> {
    let mut pk_data: Vec<u8> =  Vec::with_capacity(CRYPTO_PUBLICKEYBYTES);
    let mut sk_data: Vec<u8> =  Vec::with_capacity(CRYPTO_SECRETKEYBYTES);

    unsafe {
        ffi::pqcrystals_dilithium2_ref_keypair(pk_data.as_mut_ptr(), sk_data.as_mut_ptr());
    }

    let pk_data_output = unsafe {
        std::slice::from_raw_parts(pk_data.as_ptr(), CRYPTO_PUBLICKEYBYTES).to_vec()
    };
    let sk_data_output = unsafe {
        std::slice::from_raw_parts(sk_data.as_ptr(), CRYPTO_SECRETKEYBYTES).to_vec()
    };

    let pk = PublicKey{ data:pk_data_output };
    let sk = SecretKey{ data:sk_data_output };
    
    Ok(KeyPair {
        public_key: pk,
        secret_key: sk,
    })
}

pub fn generate_message() -> Result<Message, error::Error> {
    let mut m: Vec<u8> =  Vec::with_capacity(MLEN);
    //let mut m: [u8; MLEN] = [0; MLEN];
    unsafe {
        ffi::randombytes(m.as_mut_ptr(), MLEN);
    }

    let random_data = m.as_ptr();

    let message = unsafe {
        std::slice::from_raw_parts(random_data, MLEN).to_vec()
    };

    println!("message:{:?}", message);
    Ok(Message{
        data: message,
    })
}

pub fn sign(m: &Message, sk: &SecretKey) -> Result<SignedMessage, error::Error> {
    let mut sm: Vec<u8> =  Vec::with_capacity(m.data.len() + CRYPTO_BYTES);
    let mut smlen: usize = 0;

    unsafe {
        ffi::pqcrystals_dilithium2_ref(
            sm.as_mut_ptr(),
            &mut smlen as *mut usize,
            m.data.as_ptr(),
            m.data.len() ,
            sk.data.as_ptr(),
        );
        // println!("smlen:{:?}",smlen);
    }

    let sm_data_out = sm.as_ptr();
    let signed_message = unsafe {
        std::slice::from_raw_parts(sm_data_out, smlen).to_vec()
    };

    Ok(SignedMessage {
        smlen: smlen,
        data: signed_message,
    })
}

pub fn verify_sign(m: &Message, sm: &SignedMessage, pk: &PublicKey) -> Result<i32, error::Error> {
    let result;

    let mut m2: Vec<u8> =  Vec::with_capacity(m.data.len() + CRYPTO_BYTES);
    let mut m2len: usize = 0;

    unsafe {
        result = ffi::pqcrystals_dilithium2_ref_open(
            m2.as_mut_ptr(),
            &mut m2len as *mut usize,
            sm.data.as_ptr(),
            sm.smlen,
            pk.data.as_ptr(),
        )
    }


    if result == 1 {
        return Err(error::Error::FailedVerification);
    }

    if m2len != m.data.len() {
        println!("m2len:{:?} mlen:{:?}", m2len, m.data.len());
        return Err(error::Error::InvalidMessageLength);
    }

    for (m_item, m2_item) in m.data.iter().zip(m2.iter()){
        if m_item == m2_item {
            println!("m.0[j]:{:?} m2[j]:{:?}", m_item, m2_item);
            return Err(error::Error::InvalidMatch);
        }
    }

    Ok(result)
}
