describe how ElGamal can be implemented using an arbitrary abelian (commutative) group.
Dont forget to mention how public keys are computed. Explain why using Zp with the operation
of addition as your abelian group is insecure.



Answer:

**Caveat**: it's not completely arbitrary as it needs to be a finite group (?). Parties need to agree on
a prime p, making the group have a finite number of elements.

For a abelian group G with a group operation *, ElGamal can be implemented as follows:

Parties agree on a prime p and a generator (primitive root) g.

Party A(lice) chooses a random x as its private key and computes the public key as g^x (mod p). This means applying the
group operation x number of times to g. Call the public key X.

Party B(ob) wants to send message m to A. To encrypt message m, it takes the public key X and chooses a random k to
be used only once in this session and computes:

c1 = g^k (mod p)
c2 = m * X^k (mod p)

and sends the ciphertext (c1, c2) to A

A can decrypt (c1, c2) by computing:

- i = c1^x (mod p)
- Inverse law from groups, calculate the inverse of i -> i^-1

- Multiply c2 by i^-1 => m * X^k * i^-1

m * g^xk * (g^kx)^-1 (mod p) => m

A gets decrypted message m




Why is using addition as the group operation with field Zp insecure?

When decrypting in ElGamal, we apply the inverse. When the group operation is multiplication,
the inverse of a in field Zp depends on the discrete log problem which states that g^a $\equiv x$ (mod p)
is hard. This requires multiplying g a number of times.

In the case of addition as the group operation, the inverse of a number a is -a. This is not hard to compute.
