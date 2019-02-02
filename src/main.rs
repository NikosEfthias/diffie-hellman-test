extern crate bigint;
extern crate rand;
use bigint::uint::U512;
use rand::prelude::*;
fn main()
{
    let self_secret: u32 = thread_rng().gen_range(1, 30);
    let public_mod = 67;
    let pub_base: U512 = U512::from(73);
    let pub_key: U512 = pub_base.pow(U512::from(self_secret)) % U512::from(public_mod);
    println!("pub key is {}", pub_key);
    print!("Please provide public of the other party: ");
    use std::io::Write;
    let _ = std::io::stdout().flush();
    let mut other_pub_key = String::new();
    let _ = std::io::stdin().read_line(&mut other_pub_key);
    let other_pub_key: u64 = other_pub_key.trim_end().parse().unwrap();
    println!(
        "secret is {}",
        U512::from(other_pub_key).pow(U512::from(self_secret)) % U512::from(public_mod)
    );
    use std::io::Read;
    let _ = std::io::stdin().read(&mut [0; 1]);
}
