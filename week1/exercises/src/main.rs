fn main() {
    let result = exp_alg(3, 218, 1000);
    println!("result {}", result);

    let elgamal = Elgamal::new(467, 2);
    let ciphertext = elgamal.encrypt(224, 331, 197);
    println!("ciphertext {:?}", ciphertext);

    let message = elgamal.decrypt(ciphertext, 153);
    println!("decrypted message {:?}", message);
}

fn exp_alg(base: u128, exp: u128, mod_n: u128) -> u128 {
    let powers = powers_mod(base, exp, mod_n);
    let mut e = exp;

    let mut total: u128 = 1;
    for pow in powers.iter() {
        total = total * pow.pow((e & 1) as u32) % mod_n;
        e >>= 1;
    }
    total
}

fn powers_mod(base: u128, exp: u128, mod_n: u128) -> Vec<u128> {
    let mut e = exp;
    let mut pows: Vec<u128> = vec![base];

    let mut i: usize = 0;
    while e != 0 {
        pows.push(pows[i].pow(2) % mod_n);
        i += 1;
        e >>= 1;
    }

    pows
}

// for a set mod p where p is prime, we can calculate the multiplicative
// inverse for an element a by doing a^p-2. From fermat little theorem
// we know that a^p-1 ≡ 1 (mod p) so => a * a^p-2 ≡ 1 (mod p) i.e a^p-2
// is the modular inverse of a
fn mul_inverse(element: u128, mod_n: u128) -> u128 {
    exp_alg(element, mod_n - 2, mod_n)
}

// NOTE: should use something bigger than u128 but this captures the idea of the algorithm
struct Elgamal {
    p: u128,
    g: u128,
}

impl Elgamal {
    fn new(p: u128, g: u128) -> Elgamal {
        Elgamal { p, g }
    }

    fn encrypt(&self, pubkey: u128, message: u128, nonce: u128) -> (u128, u128) {
        let c1 = exp_alg(self.g, nonce, self.p);
        let c2 = (message * exp_alg(pubkey, nonce, self.p)) % self.p;
        (c1, c2)
    }

    fn decrypt(&self, ciphertext: (u128, u128), private_key: u128) -> u128 {
        let x = exp_alg(ciphertext.0, private_key, self.p);
        let x_mul_inverse = mul_inverse(x, self.p);
        (ciphertext.1 * x_mul_inverse) % self.p
    }
}

#[cfg(test)]
mod tests {
    use crate::{Elgamal, exp_alg};

    #[test]
    fn test_elgamal() {
        struct TestCase {
            p: u128,
            g: u128,
            x: u128,
            k: u128,
            m: u128,
            c1: u128,
            c2: u128,
        }

        let test_cases = vec![
            TestCase {
                p: 71,
                g: 33,
                x: 62,
                k: 31,
                m: 15,
                c1: 62,
                c2: 18,
            },
            TestCase {
                p: 23,
                g: 11,
                x: 6,
                k: 3,
                m: 10,
                c1: 20,
                c2: 22,
            },
            TestCase {
                p: 809,
                g: 3,
                x: 68,
                k: 89,
                m: 100,
                c1: 345,
                c2: 517,
            },
            TestCase {
                p: 17,
                g: 6,
                x: 5,
                k: 10,
                m: 13,
                c1: 15,
                c2: 9,
            },
        ];

        for case in test_cases {
            let elgamal = Elgamal::new(case.p, case.g);
            let pubkey = exp_alg(case.g, case.x, case.p);
            let ciphertext = elgamal.encrypt(pubkey, case.m, case.k);
            assert_eq!(ciphertext, (case.c1, case.c2));

            let decrypted_message = elgamal.decrypt(ciphertext, case.x);
            assert_eq!(decrypted_message, case.m);
        }
    }
}
