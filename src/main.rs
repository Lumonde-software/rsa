use std::{str::FromStr, char::from_u32};

fn main() {
    let (n, e, d) = key_gen();

    // for x in 100000..100010 {
    //     println!("x: {}", x);
    //     let y = encrypt(x, e, n);
    //     println!("y: {}", y);
    //     let ret = decrypt(y, d, n);
    //     println!("ret: {}", ret);
    //     assert!(x < n);
    // }

    let message_encrypted = send(e, n);
    receive(d, n, message_encrypted);
}

fn pow_loop(base: u64, exp: u64, n: u64) -> u64 {
    if exp == 1 { return base; };
    if exp % 2 == 1 {
        return pow_loop(base*base % n, (exp-1)/2, n) * base % n;
    } else {
        return pow_loop(base*base % n, exp/2, n) % n;
    }
}

fn send(e: u64, n: u64) -> Vec<u64> {
    // let message = "Hello, World!";
    let message = "こんにちは、世界！";
    let message_uint = str_to_uint(String::from_str(message).unwrap());
    let mut message_encrypted: Vec<u64> = Vec::new();
    for c in &message_uint {
        message_encrypted.push(encrypt(*c, e, n));
    }
    println!("---------- send ----------");
    println!("message: {}", message);
    println!("message_uint: {:?}", message_uint);
    println!("message_encrypted: {:?}\n", message_encrypted);
    message_encrypted
}

fn receive(d: u64, n: u64, message_encrypted: Vec<u64>) {
    let mut message_decrypted: Vec<u64> = Vec::new();
    for code in & message_encrypted {
        message_decrypted.push(decrypt(*code, d, n));
    }
    let message_restored = uint_to_str(&message_decrypted);
    println!("---------- receive ----------");
    println!("message_decrypted: {:?}", message_decrypted);
    println!("message_restored: {:?}", message_restored);
}

fn str_to_uint(message: String) -> Vec<u64> {
    let mut res = Vec::new();
    for c in message.chars() {
        res.push(c as u64);
    }

    res
}

fn uint_to_str(message_uint: &Vec<u64>) -> String {
    message_uint.iter().filter_map(|&code| from_u32(code as u32)).collect()
}

fn calc_d(e: u64, phi: u64) -> u64 {
    let e = e as i32;
    let phi = phi as i32;
    let (mut x0, mut y0, mut z0) = (1_i32, 0_i32, phi);
    let (mut x1, mut y1, mut z1) = (0_i32, 1_i32, e);
    loop {
        let q: i32 = z0 / z1;
        if z0 - q*z1 == 1 {
            return ((y0-q*y1 + phi) % phi) as u64;
        }
        (x0, x1) = (x1, x0-q*x1);
        (y0, y1) = (y1, y0-q*y1);
        (z0, z1) = (z1, z0-q*z1);
    }
}

fn key_gen() -> (u64, u64, u64) {
    let p = 991;
    let q = 997;
    let n = p*q;
    let phi = (p-1)*(q-1);
    let e = 65537;
    let d = calc_d(e, phi);
    // assert_eq!((e*d)%phi, 1);
    (n, e, d)
}

fn encrypt(x: u64, e:u64, n:u64) -> u64 {
    pow_loop(x, e, n)
}

fn decrypt(y: u64, d: u64, n: u64) -> u64 {
    pow_loop(y, d, n)
}