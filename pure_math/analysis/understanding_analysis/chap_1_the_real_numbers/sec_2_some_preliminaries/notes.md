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

* Functions: Given 2 sets $A$ and $B$, a _function_ from $A$ to $B$ is a rule or mapping that takes each element $x ∈ A$ and associates with it a single element of $B$. In this case, $f : A \rightarrow B.$ Given an element $x ∈ A$, the expression $f(x)$ is used to represent the element of $B$ associated with $x$ by $f$. The set $A$ is called the __domain__ of $f$. The __range__ of $f$ is not necessarily equal to $B$, but refers to the subset of $B$ given by ${y ∈ B : y = f(x) \text{for some} x ∈ A}$. (Definition 1.2.3)

    * This definition of function is basically the one proposed by Peter Lejeune Dirichlet (1805-1859) in the 1830's. He was a german mathmematician who was one of the leaders in the development of the rigorous approach to functions that we are about to udnertake. His main motivation was to unravel the issues surrounding the convergence of Fourier series. What's important is that we see how Dirichlet's definition of function liberates the term from its interpretation as a type of "formula."

    * In the yers leading up to Dirichlet's time, the term "function" was generally referred to algebraic entities such as $f(x) = x^2 + 1$ or $g(x) = \sqrt{x^4 + 4}$. However Definition 1.2.3 allows for a much broader range of possibilities.

    * Example 1.2.4 (Dirichlet's Function): In 1829, Dirichlet proposed the unruly function: $$g(x) = \begin{cases} 1 & \text{if } x \in \mathbb{Q} \\ 0 & \text{if } x \notin \mathbb{Q} \end{cases}$$

    * The domain of $g$ is all of $\mathbb{R}$, and the range is the set {0, 1}. There is no single formula for $g$ in usual sense, and it is hard to graph this function (see section 4.1 for a rough attempt), but it certainly qualifies as a function according to the Definition 1.2.3.

     * Example 1.2.5 (Traingle Inequality): The __absolute value function__ is so important that it merits the special notation |x| in place of the usual $f(x)$ or $g(x)$. It's defined for every real number via piecewise definition $$|x| = | x | = \begin{cases} x & \text{if } x \geq 0 \\ -x & \text{if } x < 0 \end{cases}$$

     * The function in Example above (1.2.5) satisfies the following in respect to multiplication & division:
        - 1: $|ab| = |a||b|$ and
        - 2: $|a + b| \leq |a| + |b|$
    for all choices of a and b. Verifyin these properties (Excersise 1.2.6) is justy a matter of examining the different cases that arise when $a$, $b$, and $a+b$ are positive and negative. Property 2 is called __triangle inequality__. This innocuous looking inequality turns out to be fantastically important and will be frequently employed in the following way. Given three real numbers $a$, $b$, and $c$ we certainly have $$|a - b| = |(a - c) + (c - b)|.$$
    By the triangle inequality, $$|(a - c) + (c - b)| \leq |a - c| + |c - b|,$$
    so we get $$|a - b| \leq |a - c| + |c - b|.$$ 
    Now, the expression $|a - b| = |b - a|$ is true, and is best understood as the __distance__ between points $a$ and $b$ on the number line. With this interpretation the equation we get makes the plausible statement that the distance from $a$ to $b$ is less than or equal to the distance from $a$ to $c$ plus the distance $c$ to $b$.


