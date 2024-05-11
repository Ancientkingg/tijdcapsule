use crypto::{aes, blockmodes, buffer::{self, BufferResult, ReadBuffer, WriteBuffer}, symmetriccipher};
use base64::{engine::general_purpose, Engine as _};



pub fn encrypt(data: &str, key_str: &str, initial_vector: Option<&[u8]>) -> Result<String, symmetriccipher::SymmetricCipherError> {
    if key_str.len() != 32 {
        return Err(symmetriccipher::SymmetricCipherError::InvalidLength);
    }


    let iv = initial_vector.unwrap_or(b"0123456789012345");
    let key = key_str.as_bytes();

    let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(general_purpose::STANDARD.encode(&final_result))
}


pub fn decrypt(encrypted_str: &str, key_str: &str, initial_vector: Option<&[u8]>) -> Result<String, symmetriccipher::SymmetricCipherError> {
    if key_str.len() != 32 {
        return Err(symmetriccipher::SymmetricCipherError::InvalidLength);
    }

    
    let iv = initial_vector.unwrap_or(b"0123456789012345");
    let encrypted_data = general_purpose::STANDARD.decode(encrypted_str.as_bytes()).unwrap();
    let key = key_str.as_bytes();

    let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize256,
            key,
            iv,
            blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(&encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(std::str::from_utf8(&final_result).unwrap().to_string())
}


#[cfg(test)]
mod tests {
    use crate::utils::idgen::key::to_key;
    use crate::utils::{cipher::{decrypt, encrypt}, idgen::key::generate};

    #[test]
    fn cipher_encrypt() {
        let content = "Hello, World!";
        let (key_fragment, key) = generate();
        let iv = None;

        let encrypted = encrypt(content, &key, iv).unwrap();

        

        assert_eq!(content, decrypt(dbg!(&encrypted), &to_key(&key_fragment), iv).unwrap());
    }
}