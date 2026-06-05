Fermat's little theorem

## Statement

$a^p \equiv a \pmod{p}$

if $p \nmid a$ => $ a^{p-1} \equiv 1 \pmod{p}$



### Proof

Start with set {a, 2a, 3a ... (p-1)a}

Reduce the set modulo p and it results in {1, 2, 3 ... p-1} rearranged

true because:

since $p \nmid a$ then for any k where `1 <= k <= p-1` $p \nmid ka$ 

Then, the terms (a, 2a, 3a...) must be distinct because 
if $ia \equiv ja \pmod{p}$  where i,j < p-1. 
This then implies that $i \equiv j \pmod{p}$


Therefore:

$ a * 2a * 3a ... * (p-1)a \equiv 1 * 2 * 3 ... p-1 \pmod{p}$

$ a^{p-1}(p-1)! \equiv (p-1)! \pmod{p}$

cancel out and results in $ a^{p-1} \equiv 1 \pmod{p}$
