use std::vec;
use rand::{self, Rng};


pub fn generate_pbox(size: usize) -> Result<Vec<u64>, String> {
    let mut result: Vec<u64> = vec![];
    for i in 0..size {
        result.push(i as u64);
    }
    let mut current_index = result.len();
    while 0 != current_index {
        let random_index = rand::thread_rng().gen_range(0..size);
        current_index -= 1;
        result.swap(random_index, current_index);
    }
    Ok(result)
}

pub fn encrypt_pbox(str: String, p_box: Vec<u64>) -> Result<Vec<u64>, String> {
    if str.is_empty() {
        return Err(String::from("Input string is empty"));
    }
    let mut result: Vec<u64> = vec![];
    let str_bytes = str.clone().into_bytes();
    for value in str_bytes.clone() {
        result.push(value as u64);
    }
    for i in 0..result.len() {
        result[i] = str_bytes[p_box[i] as usize] as u64;
    }
    Ok(result)
}

pub fn decrypt_pbox(input: Vec<u64>, p_box: Vec<u64>) -> Result<String, String> {
    if input.is_empty() {
        return Err(String::from("Input is empty"));
    }
    let mut result = String::new();
    let mut bytes: Vec<u8> = vec![];
    let pbox_inverse = pbox_inverse(p_box).unwrap();
    for i in 0..input.len() {
        bytes.push(input[pbox_inverse[i] as usize] as u8)
    }
    result = String::from_utf8(bytes).unwrap();
    Ok(result)
}

pub fn pbox_inverse(input: Vec<u64>) -> Result<Vec<u64>, String> {
    if input.is_empty() {
        return Err(String::from("P-box is empty"));
    }
    let mut result: Vec<u64> = vec![];
    let size = input.len();
    for i in 0..size {
        result.push(i as u64);
    }
    for (index, value) in input.iter().enumerate() {
        result[*value as usize] = index as u64;
    }
    Ok(result)
}