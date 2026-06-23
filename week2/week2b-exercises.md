### exercise 1

Warm-up. A single key k is drawn uniformly at random from {0, 1}^3 (so Ω
has 8 states, each of probability 1/8). Using only the three rules above, compute: (a) the
probability that k starts with a 0; (b) the probability that k has an even number of 1s; (c) the
probability that k = 010. For each, write down the event as a subset of Ω before computing
its probability, so that you are explicitly using rule 3.

$\Omega$ = { 000, 001, 010, 011, 100, 101, 110, 111 }

a) E = { 000, 001, 010, 011 }

Pr[E] = $\sum_{x\in_E}$ Pr[{x}] = 1/2

b) E = { 011, 110, 101 }

Pr[E] = $\sum_{x\in_E}$ Pr[{x}] = 3/8

c) E = { 010 }

Pr[E] = 1/8


### exercise 2

Suppose there is a room with 30 people, and assume that the probabilities of all
assignments of one of the 365 possible birthdays to each of the 30 people are equal (this is
of course not true, but assuming a uniform distribution will make things simpler). Compute
the probability that no two people share a birthday, and then compute the probability that at
least two people share a birthday.
(If instead of people and birthdays we think of inputs and hash values, this “birthday problem”
shows that finding hash collisions is easier than finding hash pre-images).

Answer:

- A room has one person and a 2nd one comes in, what is the probability that they do not share
a birthday? person 2 has to avoid person 1 birthday. The probability of that is 364/365
- A third person comes in, what is the probability all 3 do not share a birthday? Person 3 needs
to avoid the previous 2 people days so -> 363/365
- continue until 30...

Then the probability that no 2 people share a birthday is:

364/365 * 363/365 * 362/365 * ... 336/365 = 0.293684

The probability that at least 2 people share a birthday:

1 - 0.293684 = 0.706316


### exercise 3
Explain why the following is true using the definition of probability: ${\bigcup_{i=1}^n E_i}\leq \sum_{i=1}^n{E_i}.$ When are these two values equal? (Hint: first try the case where $n=2$).

In case this notation is not familiar to you, $\bigcup_{i=1}^n E_i = E_1\cup E_2\cup\cdots\cup E_n$ refers to the union of all of the sets $E_1$ through $E_n$, and $\sum_{i=1}^n{E_i} = {E_1} + {E_2} + \cdots + {E_n}$ refers to the sum of all of the probabilities ${E_1}$ through ${E_n}$. You can think of both of these as essentially for loops.

(This inequality is called the \emph{union bound}, and it is one of the most-used tools in all of cryptography: it lets us upper-bound the probability that \emph{anything} goes wrong by the sum of the probabilities of each individual bad event.)

By looking at the statement, the left side will always be <= than right side because by taking the union of events, we "deduplicate"
elements that could be repeated across different events (assuming they are not mutually exclusive). The right side
is the summation of the probability of each event.

Showing why it's true by way of example:

E = {1, 2, 3, 4, 8, 10, 11, 32}

E1 = {1, 2, 3, 8}

E2 = {1, 2, 32}

$\sum_{i=1}^n$ Pr[Ei] = 7/8

E1 U E2 = {1, 2, 3, 8, 32}

Pr[E1 U E2] = 5/8


### exercise 4

Warm-up for the law of total probability. A fair coin is flipped. If it comes
up heads, you roll a 6-sided die; if it comes up tails, you roll a 4-sided die (with faces
1, 2, 3, 4). Let E be the event that the die shows a 3. Define Ω for this experiment and use
the partition of the outcome space into the “heads” and “tails” events to compute Pr [E] as
Pr [E | heads] · Pr [heads] + Pr [E | tails] · Pr [tails]. This is exactly the case-splitting move you
will need in the one-time pad proof below.

H = heads

T = tails

$\Omega$ = H $\cap$ T

E = dice roll 3

Using the law of total probability

Pr[E] = Pr[E | H] * Pr[H] + Pr[E | T] * Pr[T]

Pr[E] = (1/6 * 0.5) + (1/4 * 0.5) = 0.2083333

intuition for this - there's a 50% chance for each heads/tails ofc
then if heads there's a 1/6 probability that the roll is a 3 (because is 6-sided dice)
and there's a 1/4 probability (because it's 4-sided dice). Add those together and we get 0.2083333

### exercise 5

Using the definitions above, show that two events, E1 and E2, are independent if
and only if P(E1 ∩ E2) = P(E1) · P(E2). This is the usual textbook definition of independent
events, but it sometimes hides the intuition behind what this has to do with independence.

Definition of independent events: We say that E1 and E2 are independent events if Pr [E1 | E2] = Pr [E1].
Intuitively, this says that learning E2 happened tells you nothing about whether E1 happened.

Conditional probability:

Pr[E1 | E2] = Pr[E1 ∩ E2] / Pr[E2]

Substitute:

Pr[E1] = Pr[E1 ∩ E2] / Pr[E2]

Pr[E1] * Pr[E2] = Pr[E1 ∩ E2] == Pr[E1 ∩ E2] = Pr[E1 | E2] * Pr[E2]


This only evaluates to true if P[E1 ∩ E2] == P[E1] · P[E2]


### exercise 6

We now have all of the tools we need to prove that the one-time pad is secure!

(a) Let us begin with the example of the single-bit encryption case (where λ = 1). Show
that for all adversary programs, A, it is always true that Advdistinguishcipher (λ) = 0.
(Hint: let Ω = K×{0, 1}×R be the set of triples with one element from K = {0, 1}, another
bit element from {0, 1}, and an element of some set, R, which represents the randomness
used by our adversary (these are sources of randomness). Show that if m0 and
m1 are any pair of messages, then Pr[A(c) = b] = 1/2 , where c = E(k, mb) = k ⊕ mb and
A is any program, by breaking into the four cases (k, b) = (0, 0), (0, 1), (1, 0), (1, 1) and
then using the partition of Ω into sets where k and b are fixed to compute Pr[A(c) = b].
Note that you will also have to treat separately the case that m0 = m1 and the case that
m0 ̸= m1).
(Extra hint if you get stuck near the end: Remember Pr[A(c) = 0] + Pr[A(c) = 1] = 1
for any fixed value of c since the probability that A returns an output must be 1).

From the pairs (k, b) = (0, 0), (0, 1), (1, 0), (1, 1), that means the probability
of each pair is 1/4

We need to find the Pr[A(c) = b] where c = E(k, mb) = k ⊕ mb
then Pr[A(k ⊕ mb) = b]

With all possible pairs:

Pr[A(k ⊕ mb) = b] = 
1/4 * (Pr[A(k ⊕ m0) = b | k = 0, b = 0]) + 
1/4 * (Pr[A(k ⊕ m0) = b | k = 0, b = 1]) + 
1/4 * (Pr[A(k ⊕ m1) = b | k = 1, b = 0]) + 
1/4 * (Pr[A(k ⊕ m1) = b | k = 1, b = 1])

Pr[A(k ⊕ mb) = b] = 1/4 * 1 + 1/4 * 0 + 1/4 * 1 + 1/4 * 0

Pr[A(k ⊕ mb) = b] = 1/2




(b) Using the same general argument as in the previous part, show that Advdistinguishcipher (λ) = 0
for all λ and for all adversaries, A.


### exercise 7

Using the security definitions we have seen so far as a model, define an attack
game that calls on an adversary to distinguish between an output of G and a truly random
element. Define the advantage an adversary has in this game, and define what it means for
a PRG, G, to be secure in terms of advantages.

--------------------------------------------------------------------------------------------
DistinguishPRG^A(l, C)

--------------------------------------------------------------------------------------------
z <-- ${0,1}^(l+C)

s <-- ${0, 1}^l

x <-- G(s)

y <-- A(x, z)

return y = x

$ means chosen uniformly at random

The advantage A has in the game:

$Adv_{A}^{DistinguishPRG}$(l, C) := Pr[DistinguishPRG^A(l, C) = true] - 1/2

For G to be secure it means that Pr[DistinguishPRG^A(l, C) = true] == 1/2




### exercise 8

Using the definition of a secure cipher given in Figure 1, and your definition of
a secure PRG from the previous exercise, use proof by reduction (the technique from Week
2a) to show that if G is a secure PRG, then EG is a secure encryption function. You may
use the fact, that we will prove next week, that if F(λ) is not negligible, then F(λ)/2 is also not
negligible.

Hint: Given a program A that has a non-negligible advantage against the game in Figure
1 for EG , construct an adversary program A′(L, r) that plays your game from the previous
exercise by using r as a one-time pad key to encrypt a random message given by A(L). To
argue that A′ has a non-negligible advantage, use the law of total probability to split on the
challenger’s hidden bit:
Pr[A′ wins] = 1/2 · Pr[A′ wins | r is random] + 1/2 · Pr[A′ wins | r = G(s)]
Notice that this is the same case-splitting move you practiced in Exercise 4, now doing real
work: one term is governed by the perfect secrecy of the one-time pad, and the other by A’s
advantage against EG .


To prove
if PRG is hard, then EG is hard.

Proof by reduction:
if EG is easy then PRG is easy


Write program A' that plays DistinguishPRG^A' game above (from previous exercise) and has non-negligible advantage

--------------------------------------------------------------------------------------------

A'(L, r)

--------------------------------------------------------------------------------------------
Z <-- G(s)

(m0, m1) <- A(L)

b <- ${0,1}

c <- Z ⊕ mb

b^ <- A(L, c)

y <- b^

return y


Pr[y = r] = Pr[y = r | b^ = 0] * Pr[b^ = 0] + Pr[y = r | b^ = 1] * Pr[b^ = 1]

Pr[y = r] = 1/2 * 1/2 + (AdvA + 1/2) * 1/2

Pr[y = r] = 1/4 + 1/2 AdvA + 1/4

Pr[y = r] = 1/2 + 1/2 AdvA




AdvA' = 1/2 + 1/2 AdvA - 1/2

AdvA' = 1/2 AdvA

