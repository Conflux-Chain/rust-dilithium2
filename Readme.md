## Introduction
The tool utilizes Rust to encapsulate the Dilithium2 algorithm. Developers can use it for signing and verifying message.

Dilithium2 is a post-quantum digital signature algorithm that provides strong security against attacks from both classical and quantum computers. It is designed to be resistant to various cryptographic attacks, including those based on the hardness of lattice problems. 

In this tool, Dilithium2 algorithm is implemented in C from NIST(https://csrc.nist.gov/Projects/post-quantum-cryptography/selected-algorithms-2022).

## Preliminaries
rust 1.75.0

## Functions

- libsign::sign::generate_message()
- libsign::sign::get_key_pair()
- libsign::sign::sign(mut m: Message, mut sk: Secret_key)
- libsign::sign::verify_sign(m: Message, mut sm: Signed_message, mut pk: Public_key)


