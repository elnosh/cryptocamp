fn main() {
    let result = exp_alg(3, 218, 1000);
    println!("result {}", result);

    let result = exp_alg(8, 248, 5600);
    println!("result {}", result)
}

fn exp_alg(base: u64, exp: u64, mod_n: u64) -> u64 {
    let powers = powers_mod(base, exp, mod_n);
    let mut e = exp;

    let mut total: u64 = 1;
    for pow in powers.iter() {
        total = total * pow.pow((e & 1) as u32) % mod_n;
        e >>= 1;
    }
    total
}

fn powers_mod(base: u64, exp: u64, mod_n: u64) -> Vec<u64> {
    let mut e = exp;
    let mut pows: Vec<u64> = vec![base];

    let mut i: usize = 0;
    while e != 0 {
        pows.push(pows[i].pow(2) % mod_n);
        i += 1;
        e >>= 1;
    }

    pows
}

// for a set mod p where p is prime, we can calculate the modular inverse for
// an element a by doing a^p-2. From fermat little theorem we know that
// a^p-1 ≡ 1 (mod p) so => a * a^p-2 ≡ 1 (mod p) i.e a^p-2 is the modular
// inverse of a
fn mod_inverse(element: u64, mod_n: u64) -> u64 {
    exp_alg(element, mod_n - 2, mod_n)
}
