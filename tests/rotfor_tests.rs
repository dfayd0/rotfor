use rotfor::{
    auto_decrypt,
    decrypt,
    encrypt,
    rotx,
};

#[test]
fn it_works()
{
    let result = rotx("toto", 0);
    assert_eq!(result, "toto".to_owned());
}

#[test]
fn rot_shift_1()
{
    let result = rotx("toto", 1);
    assert_eq!(result, "upup".to_owned());
}

#[test]
fn rot_shift_13()
{
    let result = rotx("toto", 13);
    assert_eq!(result, "gbgb".to_owned());
}

#[test]
fn rot_shift_26()
{
    let result = rotx("toto", 26);
    assert_eq!(result, "toto".to_owned());
}

pub fn generate_random_string() -> String
{
    use rand::Rng;
    let mut rng = rand::rng();
    let length = rng.random_range(1..100);
    let s: String = (0..length)
        .map(|_| rng.random_range(b'a'..=b'z') as char)
        .collect();
    s
}
pub fn _generate_random_number() -> u8
{
    use rand::Rng;
    let mut rng = rand::rng();
    rng.random_range(1..100)
}

#[test]
fn encrypt_decrypt()
{
    let input = generate_random_string();
    let shift = 13;
    let encrypted = encrypt(&input, shift);
    let decrypted = decrypt(&encrypted, shift);
    assert_eq!(input, decrypted);
}

#[test]
fn auto_decrypt_test_small_sentence()
{
    let input = "The best english sentence!".to_owned();
    let shift = 13;
    let encrypted = encrypt(&input, shift);
    let decrypted = auto_decrypt(&encrypted);
    assert_eq!(input, decrypted);
}

#[test]
fn auto_decrypt_bulk_test()
{
    let test_cases = vec![
        ("The best english sentence!", 13),
        ("Another test sentence.", 5),
        ("Rust is amazing!", 7),
        ("Testing ROT ciphers.", 12),
        ("Hello, World!", 13),
    ];

    for (input, shift) in test_cases {
        let encrypted = encrypt(input, shift);
        let decrypted = auto_decrypt(&encrypted);
        assert_eq!(input.to_string(), decrypted, "Failed on input: {}", input);
    }
}

#[test]
fn auto_decrypt_test_random_string()
{
    let input = generate_random_string();
    let shift = 13;
    let encrypted = encrypt(&input, shift);
    let decrypted = auto_decrypt(&encrypted);
    assert_eq!(input, decrypted);
}
