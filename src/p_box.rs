use std::vec;


pub fn generate_pbox(key: String, size: usize) -> Result<Vec<u64>, String> {
    if key.is_empty() {
        return Err(String::from("Key is empty"));
    }
    if size <= 0 {
        return Err(String::from("Size is lower/equal zero"));
    }
    let mut result: Vec<u64> = vec![];
    for i in 0..size {
        result.push(i as u64);
    }
    let key_len = key.len();
    for i in 0..result.len() {
        let swap_index = (result[i] as usize ^ key.as_bytes()[i % key_len] as usize) % size;
        (result[i], result[swap_index]) = (result[swap_index], result[i])
    }
    Ok(result)
}

pub fn encrypt_pbox(input_str: String, p_box: Vec<u64>) -> Result<Vec<u64>, String> {
    if input_str.is_empty() {
        return Err(String::from("Input string is empty"));
    }
    if p_box.is_empty() {
        return Err(String::from("P-box is empty"));
    }
    let mut result: Vec<u64> = vec![];
    let str_bytes = input_str.clone().into_bytes();
    for i in 0..p_box.len() {
        let temp = str_bytes[p_box[i] as usize] as u64;
        result.push(temp);
    }
    Ok(result)
}

pub fn decrypt_pbox(input: Vec<u64>, p_box: Vec<u64>) -> Result<String, String> {
    if input.is_empty() {
        return Err(String::from("Input is empty"));
    }
    if p_box.is_empty() {
        return Err(String::from("P-box is empty"));
    }
    let mut result = String::new();
    let mut bytes: Vec<u8> = vec![];
    let pbox_inverse = invers_pbox(p_box).unwrap();
    for i in 0..input.len() {
        bytes.push(input[pbox_inverse[i] as usize] as u8);
    }
    result = String::from_utf8(bytes).unwrap();
    Ok(result)
}

pub fn invers_pbox(input: Vec<u64>) -> Result<Vec<u64>, String> {
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