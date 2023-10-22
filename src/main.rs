use s_box::*;

mod tests;
mod s_box;
mod p_box;

fn main() {
    let input_string = "겡𘚟𒤩".to_string();
    let vec_result = encrypt_sbox(input_string.clone()).unwrap();
    for item in &vec_result {
        print!("{}, ", item);
    }
    println!();
    let decrypt = decrypt_sbox(vec_result.clone(), RIJNDAEL_AES_SBOX).unwrap();
    println!("Decrypted: {}", decrypt);
    if input_string == decrypt {
        println!("Yes");
    }
}
