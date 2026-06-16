### Exercise 1

Let n be a whole number. Using proof by contrapositive, show that
if n^2 is a multiple of 3, then n is a multiple of 3. State clearly what P and Q are, what
their negations are, and where in your argument you use the assumption that the conclusion
is false. (Hint: every whole number can be written as 3k, 3k + 1, or 3k + 2.) Optional fun
extension: Can you generalize the example above and the result and proof of this exercise?

Assumption (P) = n^2 is a multiple of 3
Conclusion (Q) = n is a multiple of 3

The statement P => Q is:
If n^2 is a multiple of 3, then n is a multiple of 3.


To prove by contrapositive, we prove that if n is not a multiple of 3, then n^2 is not a multiple of 3.
That is: ¬Q ⇒ ¬P

if n is not a multiple of 3 then n = 3k + 1 or n = 3k + 2 for some k

if n = 3k + 1:

n^2 = (3k + 1)^2

n^2 = 9k^2 + 6k + 1

and 9k^2 + 6k + 1 is not a multiple of 3

if n = 3k + 2:

n^2 = (3k + 2)^2

n^2 = 9k^2 + 12k + 4

and 9k^2 + 12k + 4 is not a multiple of 3

This proof that if n is not a multiple of 3 then n^2 is not a multiple of 3 proves the original statement
that if n^2 is a multiple of 3, then n is a multiple of 3.


-------------------------------------------------------------------------------

### Exercise 2

Warm-up reduction. Recall the Diffie-Hellman setup, where two parties have
public keys X = g^x and Y = g^y and their shared secret is Z = g^x·y . Suppose someone hands
you an efficient “magic box” that solves the discrete log problem: given any group element g^a
it returns a. Using this box as a subroutine, describe an efficient procedure that, given only
X = g^x and Y = g^y (but not x or y), computes the Diffie-Hellman shared secret Z = g^x·y .
Then state, in one sentence, which implication between assumptions you have just informally
proven via its contrapositive.


Using the given X = g^x, we can query the DL-solver and give X as input to get x as output.
With the output x from the DL-solver and the given Y = g^y we can perform Y^x which will
result in the shared secret Z = g^xy because (g^y)^x = g^xy.

It has been proven that if the Diffie-Hellman (DH) key exchange is hard, then the
DL is hard via the contrapositive which is that if the DL is easy, then the DH key exchange
is easy.


-------------------------------------------------------------------------------

### Exercise 3

Let G be an abelian group with an element, g ∈ G, of prime order n. A
Schnorr signature of a message, m, by a private key, x ∈ Z/nZ, is a pair (s, R) such that
s = k + H(R, m) · x ∈ Z/nZ and R = g^k ∈ G is the nonce (k is a random element of Z/nZ
generated for this signature). Intuitively, this works because s commits to the message using
a hash, uses the private key, but it does not reveal the private key because of the random
value k. A Schnorr signature can be verified by checking the equation g^s == R · X H(R,m),
where X = g^x is the signer’s public key.

Use proof by reduction to show that if we assume that the discrete log problem is hard in G,
then we can conclude that it is hard to forge two Schnorr signatures for the same nonce and
key, that is, if we are given a public key X = g^x and nonce R (without being given x or k) it
is hard to compute two different messages m1 and m2 and values s1 and s2 such that (s1 , R)
and (s2, R) are valid signatures for X of m1 and m2, respectively.

Answer:

The implication:

P = Discrete Log problem is hard
Q = Forging two Schnorr signatures for the same nonce and key is hard

P => Q

If the DLP is hard, then forging two Schnorr signatures for the same nonce and key is hard

To prove by reduction:
if forging two Schnorr signatures is easy, DLP is easy.

To create a valid Schnorr signature for a message m and nonce R we need to generate a (s, R) pair
such that g^s == R · X^H(R,m) where s = k + H(R, m) · x

If we can generate 2 Schnorr signatures for the same nonce and key, we have:
s1 = k + H(R, m1) * x and s2 = k + H(R, m2) * x

call H(R, m1) -> H1 and H(R, m2) -> H2

if subtract the two
s1 - s2 = k + H1 * x - k - H2 * x

s1 - s2 = x(H1 - H2)

x = s1 - s2 / H1 - H2
x = (s1 - s2) * (H1 - H2)^-1

where x is the private key i.e DLP is easy

-------------------------------------------------------------------------------

### Exercise 4

Use proof by reduction and the definitions above to informally show that

* (a) If the DDH assumption holds, then the CDH assumption holds.

Computation Diffie-Hellman (CDH) assumption states that it is "hard" to compute Z if given X and Y where
X = g^x, Y = g^Y and Z = g^xy

Decisional Diffie-Hellman (DDH) assumption states that it is "hard" to distinguish between a
Diffie-Hellman shared secret and a truly random group element.

Proof by reduction:
If CDH does not hold and there is a non-negligible advantage for an adversary program A to solve it, then DDH does not
hold and there is non-negligible advantage for a program A' to solve DDH.

suppose we have a program A with non-negligible advantage in CDH^A(λ), we can build a program A'
that takes inputs:

λ = Key size

Public Keys -> X = g^x and Y = g^y

Z0 = random group element

and performs the following:

Given public keys X and Y solve CDH by using program A with public keys and key size as inputs A(X, Y, λ)
that will return Z which is the shared secret between X and Y.

With non-negligible probability of calculating the shared secret Z (by using program A), our program A'
can compare the calculated Z with the random element Z0 given as input and produce the same Z0 as output.

This adversary program A' has a non-negligible advantage in DDH.


* (b) If the CDH assumption holds, then the DL assumption holds.

Proof by reduction:

If the DL does not hold and there is a non-negligible advantage for an adversary program A to solve it, then
CDH does not hold and there is a non-negligible advantage for a program A' to solve CDH.

Suppose we have a program A that takes as input a public key X and the key size λ and has a non-negligible
advantage in DL, we can build a program A' with a non-negligible advantage in solving CDH that takes two
public keys X and Y and key size λ.

Program A' will do the following:
use program A with public key X and key size as input A(X, λ) to get private x where X = g^x.
With knowledge of x, it can them compute Y^x and return the shared secret Z = g^xy.

Using program A we can build this program A' that has a non-negligible advantage in CDH to compute
the shared key between two public keys.

(Hint: for each part, start by writing “suppose A is a program with non-negligible advantage
in the easier-to-attack game,” then describe the program A′ you build and which game it
attacks. For (a), think about what an attacker that can compute Z lets you do in a game
that only asks you to distinguish Z from random.)

