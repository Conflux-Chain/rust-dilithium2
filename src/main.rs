mod libsign;

fn main() {

    println!("cargo:rerun-if-changed=src/hello.c");
}

#[cfg(test)]
mod test{
    use crate::libsign;

    #[test]
    fn test_sign(){
        let message = libsign::sign::generate_message().unwrap();
        println!("message:{:?}", message);

        let key_pair = libsign::sign::get_key_pair().unwrap();
        println!("public_key:{:?}", key_pair.public_key);
        println!("secret_key:{:?}", key_pair.secret_key);

        let signed_message = libsign::sign::sign(message.clone(), key_pair.secret_key.clone()).unwrap();
        println!("signed_message:{:?}", signed_message);
        
        let result = libsign::sign::verify_sign(message.clone(), 
                                signed_message, 
                                key_pair.public_key.clone());
        println!("result:{:?}", result);

    }
}
