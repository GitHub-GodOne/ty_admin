/// 密码加密工具类
///
/// 实现与Java版本相同的DES加密算法
/// Java代码参考: CrmebUtil.encryptPassword()
use base64::{Engine as _, engine::general_purpose};
use des::Des;
use des::cipher::{BlockEncrypt, BlockDecrypt, KeyInit, generic_array::GenericArray};

/// DES加密密码
///
/// # 参数
/// * `pwd` - 原始密码
/// * `key` - 加密密钥（通常是账号）
///
/// # 返回
/// Base64编码的加密结果
///
/// # 示例
/// ```
/// let encrypted = encrypt_password("123456", "admin");
/// ```
pub fn encrypt_password(pwd: &str, key: &str) -> String {
    let des_key = get_des_secret_key(key);
    let cipher = Des::new(&des_key);

    // 将密码转换为字节并进行PKCS7填充
    let pwd_bytes = pwd.as_bytes();
    let padded = pkcs7_pad(pwd_bytes);

    // 加密每个8字节块
    let mut encrypted = Vec::new();
    for chunk in padded.chunks(8) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted.extend_from_slice(&block);
    }

    // Base64编码
    general_purpose::STANDARD.encode(&encrypted)
}

/// DES解密密码
///
/// # 参数
/// * `encrypted_pwd` - Base64编码的加密密码
/// * `key` - 解密密钥（通常是账号）
///
/// # 返回
/// 解密后的原始密码
pub fn decrypt_password(encrypted_pwd: &str, key: &str) -> Result<String, String> {
    let des_key = get_des_secret_key(key);
    let cipher = Des::new(&des_key);

    // Base64解码
    let encrypted_bytes = general_purpose::STANDARD
        .decode(encrypted_pwd)
        .map_err(|e| format!("Base64解码失败: {}", e))?;

    // 解密每个8字节块
    let mut decrypted = Vec::new();
    for chunk in encrypted_bytes.chunks(8) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted.extend_from_slice(&block);
    }

    // 移除PKCS7填充
    let unpadded = pkcs7_unpad(&decrypted)
        .map_err(|e| format!("移除填充失败: {}", e))?;

    // 转换为字符串
    String::from_utf8(unpadded.to_vec())
        .map_err(|e| format!("UTF-8转换失败: {}", e))
}

/// 获取DES加密密钥
///
/// 与Java版本保持一致的密钥生成逻辑
/// Java代码: CrmebUtil.getDESSercretKey()
///
/// # 逻辑
/// 1. 创建8字节数组
/// 2. 将key的UTF-8字节复制到数组
/// 3. 如果key长度不足8字节，剩余位置填充0x01
fn get_des_secret_key(key: &str) -> GenericArray<u8, des::cipher::typenum::U8> {
    let mut result = [0x01u8; 8];
    let key_bytes = key.as_bytes();

    for i in 0..8 {
        if i < key_bytes.len() {
            result[i] = key_bytes[i];
        }
    }

    GenericArray::clone_from_slice(&result)
}

/// PKCS7填充
///
/// 将数据填充到8字节的倍数
fn pkcs7_pad(data: &[u8]) -> Vec<u8> {
    let block_size = 8;
    let padding_len = block_size - (data.len() % block_size);
    let mut padded = data.to_vec();
    padded.extend(vec![padding_len as u8; padding_len]);
    padded
}

/// 移除PKCS7填充
fn pkcs7_unpad(data: &[u8]) -> Result<&[u8], String> {
    if data.is_empty() {
        return Err("数据为空".to_string());
    }

    let padding_len = data[data.len() - 1] as usize;

    if padding_len == 0 || padding_len > 8 {
        return Err("无效的填充长度".to_string());
    }

    if data.len() < padding_len {
        return Err("数据长度小于填充长度".to_string());
    }

    // 验证填充是否正确
    for i in 0..padding_len {
        if data[data.len() - 1 - i] != padding_len as u8 {
            return Err("填充验证失败".to_string());
        }
    }

    Ok(&data[..data.len() - padding_len])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let password = "123456";
        let account = "admin";

        let encrypted = encrypt_password(password, account);
        println!("加密结果: {}", encrypted);

        let decrypted = decrypt_password(&encrypted, account).unwrap();
        assert_eq!(password, decrypted);
    }

    #[test]
    fn test_get_des_secret_key() {
        let key = "admin";
        let secret_key = get_des_secret_key(key);

        // 验证前5个字节是"admin"的UTF-8编码
        assert_eq!(secret_key[0], b'a');
        assert_eq!(secret_key[1], b'd');
        assert_eq!(secret_key[2], b'm');
        assert_eq!(secret_key[3], b'i');
        assert_eq!(secret_key[4], b'n');
        // 验证剩余字节是0x01
        assert_eq!(secret_key[5], 0x01);
        assert_eq!(secret_key[6], 0x01);
        assert_eq!(secret_key[7], 0x01);
    }

    #[test]
    fn test_pkcs7_padding() {
        let data = b"hello";
        let padded = pkcs7_pad(data);

        // "hello"是5字节，需要填充3字节到8字节
        assert_eq!(padded.len(), 8);
        assert_eq!(padded[5], 3);
        assert_eq!(padded[6], 3);
        assert_eq!(padded[7], 3);

        let unpadded = pkcs7_unpad(&padded).unwrap();
        assert_eq!(unpadded, b"hello");
    }
}
