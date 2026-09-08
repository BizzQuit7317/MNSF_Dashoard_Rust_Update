use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};

use age::secrecy::SecretString;
use rpassword;

fn main() {
    let input_path = "secrets.txt";
    //let output_path = "secrets.txt.age";
    let passphrase = SecretString::from(
        rpassword::prompt_password("Passphrase: ").expect("Could not read passphrase!")
    );

    println!("Output file name: ");
    io::stdout().flush().expect("Could not flush stdout!");

    let mut output_path_buffer = String::new();
    io::stdin().read_line(&mut output_path_buffer).expect("Could not read line!");

    let output_path = format!("{}.txt.age", output_path_buffer.trim());

    let mut plaintext = File::open(input_path).expect("Could not find file!");
    let ciphertext_sink = BufWriter::new(File::create(output_path).expect("Could not create the output file!"));

    let encryptor =
        age::Encryptor::with_user_passphrase(SecretString::from(passphrase.to_owned()));

    let mut writer = encryptor.wrap_output(ciphertext_sink).expect("Could not wrap output!");
    io::copy(&mut plaintext, &mut writer).expect("Could not encrypt!");

    let mut inner = writer.finish().expect("Could not finish encryption!");
    inner.flush().expect("Could not flush!");

    println!("Finished!")
}
