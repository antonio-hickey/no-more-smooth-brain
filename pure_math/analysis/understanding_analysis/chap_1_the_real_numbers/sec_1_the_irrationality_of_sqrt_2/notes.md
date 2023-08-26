# The Irrationality of $\sqrt{2}$

TLDR; Good basic introduction to theorems, proofs, and beauty of pure math.

### Theorem 1.1.1: *There is no rational number whose square is 2.*

Proof: A rational number is any number that can be expressed in the form $p/q$, where $p$ and $q$ are integers. So, the theorem says that no matter how $p$ and $q$ are chosen, it's never the case that $(p/q)^2=2$. "The line of attack is indirect, using a type of argument referred to as a proof by contradiction." The idea is assuming that there IS a rational number whose square is 2 and then proceed along logical lines until you reach a conclusion that is unacceptable.

(1) $$(\frac{p}{q})^2 = 2$$
* We may also assume that $p$ and $q$ have no common factor, because if they did we could simply cancel it out and rewrite the fraction in lowest terms.

(2) $$p^2 = 2q^2$$
* Here, we can see the integer $p^2$ is an even number as it's divisible by 2, so $q$ must be even as well as the square of an odd number is odd. This allows us to say $p = 2r$, where r is also an integer. If we substitute $2r$ for $p$ in equation (2), then a little algebra gets us the relationship: 

$$2r^2 = q^2$$

* But now we reached logical impasse, as the last equation implies $q^2$ is even, so $q$ must also be even. Thus, we've shown that $p$ and $q$ are both divisible by 2 when they were previously assumed to have no common factor. We can only conclude that equation (1) cannot hold for any integers $p$ and $q$, and thus the theorem is proved.

