fn main()
{
    println!("Welcome to forot!");

    let input = "The use of letter frequencies and frequency analysis plays \
                 a fundamental role in cryptograms and several word puzzle \
                 games.";
    let shift = forot::generate_random_number();

    let encrypted = forot::encrypt(input, shift);
    let decrypted = forot::decrypt(&encrypted, shift);

    let guess_decrypted = forot::auto_decrypt(&encrypted);

    println!("Input: {}", input);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
    println!("Guess decrypted: {}", guess_decrypted);
}
