use super::sign;

#[test]
fn test_sign() {
    let message = sign::generate_message().unwrap();
    println!("message:{:?}", message);
    
    let key_pair = sign::get_key_pair().unwrap();
    //println!("key_pair:{:?}", key_pair);

    let signed_message = sign::sign(&message, &key_pair.secret_key).unwrap();
    //println!("signed_message:{:?}", signed_message);

    let result = sign::verify_sign(&message, &signed_message, &key_pair.public_key).unwrap();
    print!("result:{:?}", result);
}
