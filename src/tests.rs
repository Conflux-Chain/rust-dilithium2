use super::sign;

#[test]
fn test_sign() {
    let message = sign::generate_message().unwrap();

    let key_pair = sign::get_key_pair().unwrap();

    let signed_message = sign::sign(&message, &key_pair.secret_key).unwrap();

    sign::verify_sign(&message, &signed_message, &key_pair.public_key).unwrap();
}
