
fn main() {

    // Example 1
    let mut test_str_encode: String = String::from("abcde");
    let mut test_str_add: String = String::from("jjh");
    let mut test_offset: i32 = -5;

    println!("Converting '{}' with offset {} and additional string '{}'", test_str_encode, test_offset, test_str_add);    
    let res = encode(&test_str_encode, test_offset, &test_str_add).unwrap();
    println!("Result Encoding: {}", res);

    println!("Decoding '{}' with offset {}", res, test_offset);
    let res = decode(&res, test_offset).unwrap();
    println!("Result Decoding '{}'", res);



    // Example 2
    let mut test_str_encode: String = String::from("xyz");
    let mut test_str_add: String = String::from("a");
    let mut test_offset: i32 = 3;

    println!("Converting '{}' with offset {} and additional string '{}'", test_str_encode, test_offset, test_str_add);    
    let res = encode(&test_str_encode, test_offset, &test_str_add).unwrap();
    println!("Result Encoding: {}", res);

    println!("Decoding '{}' with offset {}", res, test_offset);
    let res = decode(&res, test_offset).unwrap();
    println!("Result Decoding '{}'", res);
}

/// str_to_encode and str_add are NON MUTABLE references. We dont allow to change the strings itself
fn encode(str_to_encode: &String, offset: i32, str_add: &String) -> Option<String> {

    const ALLOWED_CHARS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    
    // Check if the strings are empty
    if str_to_encode.is_empty() {
        println!("The string to encode is empty.");
        return None;
    }
    if str_add.is_empty() {
        println!("The additional string is empty.");
        return None;
    }

    // Make sure the str_to_encode only contains valid characters
    // CHATGPT proposes to use chars().all() with a lambda function. Looks nicer than a loop
    // borrow c in contains() with &c
    if !(str_to_encode.chars().all(|c| ALLOWED_CHARS.contains(&c))) {
        println!("The string to encode contains invalid characters. Only a-z allowed.");
        return None;
    }
    if !(str_add.chars().all(|c| ALLOWED_CHARS.contains(&c))) {
        println!("The additional string contains invalid characters. Only a-z allowed.");
        return None;
    }

    // Convert the strings to a vector of u8. NOTE: This "consumes" str_to_encode, it is no longer available within the scope of encode
    let str_to_encode_vector: Vec<u8> = str_to_encode.as_bytes().to_vec();
    let str_add_vector: Vec<u8> = str_add.as_bytes().to_vec();
    
    // insert the str_add_vector values into str_to_encode_vector. Here we need a mutable vector
    let mut str_to_encode_vector_with_add: Vec<u8> = Vec::new();
    // Make sure the str_add_vector is cycled
    let mut ct: usize = 0;
    for v in 0..str_to_encode_vector.len() {

        // push the original and one of the str_add_vector values
        str_to_encode_vector_with_add.push(str_to_encode_vector[v]);
        
        // Push one of the str_add_vector elements, cycle if needed. Do not add after the last element of str_to_encode_vector
        if v < str_to_encode_vector.len() - 1 {
            str_to_encode_vector_with_add.push(str_add_vector[ct % str_add_vector.len()]);
            ct += 1;
        }
    }

    // Do the ROT transformation, wrap around the designated char range
    let mut encoded_vector: Vec<u8> = Vec::new();
    for v in str_to_encode_vector_with_add {
        let new_v: u8 = wrap_ascii(v as i32, offset, ALLOWED_CHARS[0] as i32, ALLOWED_CHARS[ALLOWED_CHARS.len()-1] as i32);
        encoded_vector.push(new_v);
    }

    // Convert the encoded_vector back to a string
    let encoded_string: String = String::from_utf8(encoded_vector).expect("Failed to convert encoded_vector to string.");

    return Some(encoded_string);
    
}

/// str_to_decode is a NON MUTABLE reference. We dont allow to change the string itself
fn decode(str_to_decode: &String, offset: i32) -> Option<String> {
    const ALLOWED_CHARS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    
    // Check if the strings are empty
    if str_to_decode.is_empty() {
        println!("The string to decode is empty.");
        return None;
    }
    // Check for invalid chars
    if !(str_to_decode.chars().all(|c| ALLOWED_CHARS.contains(&c))) {
        println!("The string to decode contains invalid characters. Only a-z allowed.");
        return None;
    }

    // Convert the string to a vector of u8
    let str_to_decode_vector_full: Vec<u8> = str_to_decode.as_bytes().to_vec();

    // Remove every second element and apply ROT transformation
    let mut str_to_decode_vector: Vec<u8> = Vec::new();
    for i in 0..str_to_decode_vector_full.len() {
        if i % 2 == 0 {
            let new_value: u8 = wrap_ascii(str_to_decode_vector_full[i] as i32, -offset, ALLOWED_CHARS[0] as i32, ALLOWED_CHARS[ALLOWED_CHARS.len()-1] as i32);
            str_to_decode_vector.push(new_value);
        }
    }

    // Convert the str_to_decode_vector back to a string
    let decoded_string: String = String::from_utf8(str_to_decode_vector).expect("Failed to convert str_to_decode_vector to string.");

    return Some(decoded_string);
}

/// Given a lower and upper bound, wrap around the result of start + offset in that range
/// Since the result is used as an ascii byte, return u8
fn wrap_ascii(start: i32, offset: i32, lower_bound: i32, upper_bound: i32) -> u8 {
    
    let range = upper_bound - lower_bound + 1;

    let start_with_offset = start + offset;
    match start_with_offset {
        // If the start_with_offset is lower than lower bound, wrap around to the upper bound 
        _ if start_with_offset < lower_bound => {
            let diff = lower_bound - start_with_offset - 1;
            let diff_mod_range = diff % range; // make sure we only wrap around the excess of the range
            return (upper_bound - diff_mod_range) as u8;
        },
        _ if start_with_offset > upper_bound => {
            let diff = start_with_offset - upper_bound - 1;
            let diff_mod_range = diff % range;
            return (lower_bound + diff_mod_range) as u8;
        },
        _ => return start_with_offset as u8, // If the offset does not move us outside of the range, just return the result
    }
}