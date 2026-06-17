use secp256k1::hashes::{Hash, sha256};
use secp256k1::{PublicKey, Scalar, Secp256k1, SecretKey, Signing, Verification, constants, rand};

fn main() {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::rng());

    let msg1 = sha256::Hash::hash(b"Hello World!").to_byte_array();
    let msg2 = sha256::Hash::hash(b"Hello again!").to_byte_array();

    // k = nonce, R = k*G, x = private key, P = public key
    // a signature is the pair (R, s) where s = k + H(P, k, msg)*x
    let (nonce, _) = secp.generate_keypair(&mut rand::rng());
    let (r1, s1) = sign_with_nonce(&secp, &secret_key, &nonce, &msg1);
    let (r2, s2) = sign_with_nonce(&secp, &secret_key, &nonce, &msg2);

    assert!(
        verify(&secp, &public_key, &msg1, &r1, &s1),
        "sig1 must verify"
    );
    assert!(
        verify(&secp, &public_key, &msg2, &r2, &s2),
        "sig2 must verify"
    );
    assert_eq!(r1, r2, "nonce R must match");

    // private key = (s1 - s2) * (H1 - H2)^-1
    // the key can be extracted from the 2 sigs using the same nonce because
    // call H1 = H(P, k, msg1) and H2 = H(P, k, msg2)
    // s1 = k + H1 * x
    // s2 = k + H2 * x
    // then
    // (s1 - s2) = (k + H1 * x) - (k + H2 * x)
    // (s1 - s2) = x(H1 - H2)
    // x = (s1 - s2) * (H1 - H2)^-1
    let msg1: [u8; 32] = msg1.try_into().unwrap();
    let msg2: [u8; 32] = msg2.try_into().unwrap();

    let e1 =
        Scalar::from_be_bytes(challenge(&r1.serialize(), &public_key.serialize(), &msg1)).unwrap();
    let e2 = SecretKey::from_byte_array(challenge(&r2.serialize(), &public_key.serialize(), &msg2))
        .unwrap();

    let sub_s = s2.negate().add_tweak(&Scalar::from(s1)).unwrap();
    let sub_e = e2.negate().add_tweak(&e1).unwrap();
    let mul_inverse = mul_inverse(sub_e);

    let extracted_key = sub_s.mul_tweak(&Scalar::from(mul_inverse)).unwrap();
    assert_eq!(extracted_key.secret_bytes(), secret_key.secret_bytes());
    println!("extracted key {:?}", extracted_key.secret_bytes())
}

fn sign_with_nonce<C: Signing>(
    secp: &Secp256k1<C>,
    sk: &SecretKey,
    k: &SecretKey,
    msg: &[u8],
) -> (PublicKey, SecretKey) {
    let p = PublicKey::from_secret_key(secp, sk);
    let r = PublicKey::from_secret_key(secp, k);

    // e = H(R || P || m)  as a scalar mod n.
    let e = Scalar::from_be_bytes(challenge(&r.serialize(), &p.serialize(), msg))
        .expect("challenge >= curve order (negligible)");

    // s = k + e*d
    let e_d = sk.mul_tweak(&e).expect("e*d");
    let s = k.add_tweak(&Scalar::from(e_d)).expect("k + e*d");

    (r, s)
}

/// Verify a Schnorr signature:  s*G == R + e*P.
fn verify<C: Signing + Verification>(
    secp: &Secp256k1<C>,
    p: &PublicKey,
    msg: &[u8],
    r: &PublicKey,
    s: &SecretKey,
) -> bool {
    let e = Scalar::from_be_bytes(challenge(&r.serialize(), &p.serialize(), msg)).unwrap();
    let s_g = PublicKey::from_secret_key(secp, s); // s*G
    let e_p = p.mul_tweak(secp, &e).expect("e*P"); // e*P
    let rhs = r.combine(&e_p).expect("R + e*P"); // R + e*P
    s_g == rhs
}

fn mul_inverse(a: SecretKey) -> SecretKey {
    let mut exp = constants::CURVE_ORDER;
    exp[31] -= 2;

    let mut one = [0u8; 32];
    one[31] = 1;
    let mut result = SecretKey::from_byte_array(one).unwrap();

    for byte in exp {
        for i in (0..8).rev() {
            result = result.mul_tweak(&Scalar::from(result)).expect("square");
            if (byte >> i) & 1 == 1 {
                result = result.mul_tweak(&Scalar::from(a)).expect("multiply");
            }
        }
    }
    result
}

fn challenge(r: &[u8; 33], p: &[u8; 33], msg: &[u8]) -> [u8; 32] {
    let mut data = Vec::with_capacity(33 + 33 + msg.len());
    data.extend_from_slice(r);
    data.extend_from_slice(p);
    data.extend_from_slice(msg);
    sha256::Hash::hash(&data).to_byte_array()
}
