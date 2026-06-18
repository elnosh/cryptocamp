Exercise 1. Warm-up. A single key k is drawn uniformly at random from {0, 1}^3 (so Ω
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


Exercise 2. Suppose there is a room with 30 people, and assume that the probabilities of all
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


\begin{exercise}
Explain why the following is true using the definition of probability: $\pr{\bigcup_{i=1}^n E_i}\leq \sum_{i=1}^n\pr{E_i}.$ When are these two values equal? (Hint: first try the case where $n=2$).

In case this notation is not familiar to you, $\bigcup_{i=1}^n E_i = E_1\cup E_2\cup\cdots\cup E_n$ refers to the union of all of the sets $E_1$ through $E_n$, and $\sum_{i=1}^n\pr{E_i} = \pr{E_1} + \pr{E_2} + \cdots + \pr{E_n}$ refers to the sum of all of the probabilities $\pr{E_1}$ through $\pr{E_n}$. You can think of both of these as essentially for loops.

(This inequality is called the \emph{union bound}, and it is one of the most-used tools in all of cryptography: it lets us upper-bound the probability that \emph{anything} goes wrong by the sum of the probabilities of each individual bad event.)
\end{exercise}

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


Exercise 4. Warm-up for the law of total probability. A fair coin is flipped. If it comes
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


Exercise 5. Using the definitions above, show that two events, E1 and E2, are independent if
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

