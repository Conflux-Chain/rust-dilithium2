use super::sign;

#[test]
fn test_sign() {
    let message = sign::generate_message().unwrap();

    let key_pair = sign::get_key_pair().unwrap();

    let signed_message = sign::sign(message.clone(), key_pair.secret_key.clone()).unwrap();

    sign::verify_sign(message.clone(), signed_message, key_pair.public_key.clone()).unwrap();
}
