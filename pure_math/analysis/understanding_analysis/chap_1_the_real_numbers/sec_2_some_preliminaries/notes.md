# Some Preliminaries

Reviewing some set theory & theory of f(x)'s
---
* Sets: basically a collection of things, kind of like a hashset, but there's no order.

    * $∈$: "element of", like `A.includes(x) == true` where `x` is a element in hashset `A`.
    
    * $∉$: "not element of", like `A: Hashset.includes(x) == false` where `x` is NOT a element in hashset `A`.
    
    * $∪$: "union" (think merge of sets), like `C = A.concat(B)` where `C` is a hashset containing all the elements in both hashsets `A` and `B`.
    
    * $∩$: "Intersection" (elements in both sets), like `A.includes(x) && B.includes(x) == true` where x is a element in BOTH hashsets `A` and `B`.
    
    * $⊆$: "subset" (set contained in set), like `A = [0, 1, x:[1, 0, 1], 0, 0, 1]` where `x:[1, 0, 1]` is a subset of `A`.
    
    * There are many acceptable ways to assert the contents of a set. In the previous section, the set of natural numbers was defined by listing the elements: $N = {1,2,3...}$

    * Sets can also be described in words, for example we can define the set $E$ to be a collection of even natural numbers.

    * Sometimes it is more effient to provide a kind of ruke oir alforithnm for determining the elements of a set. For example, let $$S = {r ∈ Q : r^2 < 2}$$ Read aloud, the definition of $S$ says, "Let $S$ be the set of all rational numbers whose squares are less than 2." It follows that $1 ∈ S$, $\frac{4}{3} ∈ S$, but $\frac{3}{2} ∉ S$ because $\frac{9}{4} \geq 2$.

    * Example 1.2.2: Let $$A_{1} = \mathbf{N} = {1, 2, 3,\hspace{0.1cm}...},$$ $$A_{2} = {2, 3, 4,\hspace{0.1cm}...},$$ $$A_{3} = {3, 4, 5,\hspace{0.1cm}...},$$ and in general for each $n ∈ \mathbf{N}$, define the set $$A_{n} = {n, n + 1, n + 2,\hspace{0.1cm}...}.$$ The result is a nested chain of sets $$A_{1} ⊇ A_{2} ⊇ A_{3} ⊇ A_{4} ⊇ \hspace{0.1cm}...,$$ where each successive set is a subset of all previous iterations. Notionaly, $$\bigcup\limits_{n=1}^{\infty}A_{n}, \bigcup\limits_{n∈\mathbf{N}}A_{n}, \hspace{0.1cm}or\hspace{0.1cm} A_{1} ∪ A_{2} ∪ A_{3} ∪ \hspace{0.1cm}...$$ are all equivalent ways to indicate the set whose elements consist of any element that appears in at least one particular $A_{n}$. Because of the nested property of this particular collection of sets, "it is not too hard to see that" (:joy:): $$\bigcup\limits_{n=1}^{\infty}A_{n} = A_1.$$ The notion of intersection has the same kind of natural extension to infinite collection of sets. For this example we have $$\bigcap\limits_{n=1}^{\infty}A_{n} = \emptyset.$$ 
