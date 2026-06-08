### Exercise 1
Let G be an abelian group with an element, g ∈ G, of prime order n. A
Schnorr signature of a message, m, by a private key, x ∈ Z/nZ, is a pair (s, R) such that
s = k + H(R, m) · x ∈ Z/nZ and R = g k ∈ G is the nonce (k is a random element of Z/nZ
generated for this signature). Intuitively, this works because s commits to the message using
a hash, uses the private key, but it does not reveal the private key because of the random
value k. A Schnorr signature can be verified by checking the equation g^s == R · X H(R,m),
where X = g x is the signer’s public key.

Use proof by reduction to show that if we assume that the discrete log problem is hard in G,
then we can conclude that it is hard to forge two Schnorr signatures for the same nonce and
key, that is, if we are given a public key X = g x and nonce R (without being given x or k) it
is hard to compute two different messages m1 and m2 and values s1 and s2 such that (s1 , R)
and (s2 , R) are valid signatures for X of m1 and m2 , respectively.


Answer:

To prove by reduction: if forging two Schnorr signatures is easy, then the DL problem is easy.

To create a valid Schnorr signature we need to generate a (s, R) pair such that g^s == R · X^H(R,m)
where s = k + H(R, m) · x

The Discrete Log (DL) assumption states that given X where X = g^x for a Group G and generator g, it is hard
to compute x.

R, X and H(R, m) are publicly known. Forging a Schnorr signature would require generating s with knowledge
of private key x which is not publicly known. To know x, we would need to compute the discrete logarithm of
X to the base g. Thus, if it's easy to forge a Schnorr signature, then the DL problem is easy.



### Exercise 2

Use proof by reduction and the definitions above to informally show that
(a) If the DDH assumption holds, then the CDH assumption holds.

Computation Diffie-Hellman (CDH) assumption states that it is "hard" to compute Z if given X and Y where
X = g^x, Y = g^Y and Z = g^xy

More formally:

Adv (cdh) (λ) := Pr[CDH (λ) = true]

Adv = advantage an adversary has
Pr = probability function
And we say that the assumption does holds if the value of `Adv` is "negligible".

Decisional Diffie-Hellman assumption states that it is "hard" to distinguish between a Diffie-Hellman shared
secret (i.e Z) and a truly random group element.

Proof:
Assume Computational Diffie-Hellman assumption does not hold and it is easy to compute Z if given X and Y.
If we can easily compute the shared secret Z if given X and Y then we would easily be able to distinguish it
from a truly random group element.

Using the more formal definition from above:
If `Pr[CDH (λ) = true]` is a non-negligible value and we can compute the Diffie-Hellman shared key then we can
distinguish it from a truly random group element.


(b) If The CDH assumption holds, then the DL assumption holds.
In order to solve this part, you may use two facts that we will prove later (though it is
possible to do this exercise without):
    (1) If you have two independent events A and B, then Pr [A and B] = Pr [A] · Pr [B].
    (2) If F (λ) is not negligible, then F (λ)2 is also not negligible.


The Discrete Log (DL) as stated above or can also be stated more formally as:
Adv (dl) (λ) := Pr[DL (λ) = true]

And we say that the DL assumption holds if `Adv` from the probability function is negligible.

Proof:

Assume that the DL assumption does not hold and we can "easily" compute x from a given X where X = g^x for group G.

Then the Computational Diffie-Hellman (CDH) assumption does not hold because given two public keys X and Y
we can compute their private keys x and y such that X = g^x and Y = g^y and derive the shared Diffie-Hellman
key Z = g^xy

We do not need to compute both x and y, we can only compute one of x or y and use the counterpart
public key to derive the shared key Z.

