use std::str::FromStr;

fn main() {
    // let (n, e, d) = key_gen();
    // let message_encrypted = send(e, n);
    let (n, e, d) = (4119524081, 65537, 3373923173);
    let message_encrypted = "4134a026a62b5d0bd83a1c323aeed9fb47c88dba6cbf1a842b4675341bb836a77257163823fb5ace0441090ab9a6624e4134a026a62b5d0bd83a1c323292b2dd7bd6d18633ad55d43292b2dd33ad55d43da32f7b3292b2dd7bd6d1866240e1ac3292b2dd7bd6d186bf4f9eea23fb5ace0441090acd4e9c353292b2dd7bd6d186374d3a8f3292b2dd7bd6d186ec24adca3292b2dd894c163a7bd6d1863aeed9fbe91ad3687bd6d1863aeed9fb4887dbeb068d11a83292b2dd7bd6d186a85d96db2b467534621a8ddf85c00e8a3292b2dd7bd6d186e0c4bc303292b2dd7bd6d1866956b9da2b4675348c21ce5eb9a6624e3aeed9fbb9a6624e8c21ce5e3aeed9fb4887dbeb068d11a83292b2dd7bd6d186d437b20430e5e75c89bd5ecac4705e332b46753439875335c4705e333aeed9fb4887dbeb068d11a82b467534b9a6624ebf4f9eeaccd0706685c00e8a92d60ac13292b2dd7bd6d186a85d96db30e5e75c1bb836a712274acd2b467534d437b2040955022130e5e75c47c88dba6956b9da3292b2dd7bd6d186811cf1913aeed9fb4d466e7833ad55d446a272c6d1cfc7793da32f7b2b467534bf4f9eea8029eee83292b2dd7bd6d18685c00e8a2b46753439875335068d11a846a272c63987533592d60ac13292b2dd7bd6d18685c00e8a3292b2dd7bd6d18633ad55d43292b2dd33ad55d463d8efbc3292b2dd7bd6d186374d3a8fb03440eb88034757e91ad3683292b2dd7bd6d18605654d2e3292b2dd33ad55d4cd4e9c353292b2dd33ad55d4a85d96db3292b2dd7bd6d1868d2739163292b2dd7bd6d1866956b9da3292b2dd33ad55d463d8efbc3292b2dd7bd6d1863da32f7b3292b2dd7bd6d186374d3a8f3292b2dd33ad55d499d3a6fa2b467534d437b204cd4e9c352b4675348029eee8374d3a8f3aeed9fb894c163a85c00e8a3292b2dd7bd6d186d437b2043aeed9fbc4705e33080047983aeed9fb63d8efbcc4705e333292b2dd7bd6d186374d3a8f3292b2dd7bd6d1866cbf1a843292b2dd7bd6d186095502212b4675348029eee8b122cf9346a272c6d1cfc77963d8efbc46a272c6e0c4bc30483dfc903aeed9fb47c88dba6cbf1a842b4675341bb836a7725716383292b2dd7bd6d186d437b204b03440eb1b1b1d73894c163a3292b2dd7bd6d186621a8ddf3292b2dd7bd6d18685c00e8a3292b2dd7bd6d18633ad55d43292b2dd33ad55d463d8efbc3292b2dd894c163a33ad55d43aeed9fb47c88dba6cbf1a842b4675341bb836a7725716383292b2dd7bd6d186374d3a8f3292b2dd5354c00b546af8b03292b2dd33ad55d41b1b1d733292b2dd33ad55d4880347573292b2dd5354c00b42ca17d630e5e75c0d50cbe9c587255e2b4675348c21ce5ee0c4bc303292b2dd33ad55d499d3a6fa2b467534d437b2040955022130e5e75c1bb836a712274acd3292b2dd7bd6d18685c00e8a3292b2dd7bd6d186e0c4bc303292b2dd33ad55d463d8efbc3aeed9fbd1cfc779080047982b4675340441090a1bb836a73292b2dd7bd6d186374d3a8f3292b2dd7bd6d1866cbf1a843292b2dd7bd6d1868d2739163aeed9fb48a0a01b894c163a2b467534b9a6624e2126e0bc3292b2dd7bd6d18642ca17d62b4675348029eee8b122cf9346a272c6d1cfc77963d8efbc3292b2dd7bd6d1864887dbeb3292b2dd33ad55d4a85d96db3292b2dd7bd6d186095502213292b2dd33ad55d433ad55d43292b2dd7bd6d186d437b2043292b2dd7bd6d18685c00e8a3292b2dd7bd6d18633ad55d43292b2dd33ad55d463d8efbc3292b2dd894c163a33ad55d4".to_string();
    receive(d, n, &message_encrypted);
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

fn receive(d: u64, n: u64, message_encrypted:&str) {
    let mut message_decrypted: Vec<u32> = Vec::new();
    let mut mm: Vec<String> = Vec::new();
    for chunk in message_encrypted.chars().collect::<Vec<_>>().chunks(8) {
        let chunk_str: String = chunk.iter().collect();
        assert_eq!(chunk_str.len(), 8);
        let chunk_u64 = match u64::from_str_radix(&chunk_str, 16) {
            Ok(result) => result,
            Err(_) => 0,
        };
        message_decrypted.push(decrypt(chunk_u64, d, n));
        mm.push(chunk_str);
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

fn uint_to_str(message_uint: &Vec<u32>) -> String {
    let message_u8: Vec<u8> = message_uint.iter().map(|&code| code as u8).collect();
    let res = String::from_utf8_lossy(&message_u8).to_string();

    res
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
    assert_eq!((e*d)%phi, 1);
    (n, e, d)
}

fn encrypt(x: u64, e:u64, n:u64) -> u64 {
    pow_loop(x, e, n)
}

fn decrypt(y: u64, d: u64, n: u64) -> u32 {
    pow_loop(y, d, n) as u32
}
