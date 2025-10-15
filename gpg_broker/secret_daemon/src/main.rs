use rpassword;
use secrecy::SecretString;

fn main() {
    println!("Enter passkey: ");
    let password = SecretString::new(rpassword::read_password().unwrap().into());

    println!("you entered: {:?}", password);

}
