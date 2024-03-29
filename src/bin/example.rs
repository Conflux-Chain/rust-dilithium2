use rust_dilithium2::sign;

fn main() {
    let message = sign::generate_message().unwrap();
    println!("message:{:x?}", message);

    let key_pair = sign::get_key_pair().unwrap();
    println!("public_key:{:?}", key_pair.public_key);
    println!("secret_key:{:?}", key_pair.secret_key);

    let signed_message = sign::sign(&message, &key_pair.secret_key).unwrap();
    println!("signed_message:{:?}", signed_message);

    let result = sign::verify_sign(&message, &signed_message, &key_pair.public_key);
    println!("result:{:?}", result);
}
