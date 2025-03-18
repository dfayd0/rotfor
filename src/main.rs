fn main()
{
    println!("Welcome to rotfor!");

    let input = "The use of letter frequencies and frequency analysis plays \
                 a fundamental role in cryptograms and several word puzzle \
                 games.";
    let shift = rotfor::generate_random_number();

    let encrypted = rotfor::encrypt(input, shift);
    let decrypted = rotfor::decrypt(&encrypted, shift);

    let guess_decrypted = rotfor::auto_decrypt(&encrypted);

    println!("\nInput: {}", input);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
    println!("Guess decrypted: {}", guess_decrypted);
}
