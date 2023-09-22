Exercises for section 2 "Some Preliminaries" (basic set theory & function theory)
===

Exercise 1.2.3
---
Decide which of the following represent true statements about the nature of sets. For any that are false, provide a specific example where the statement in question does not hold.

---

* (a) If $A_{1} \nsubseteq A_{2} \nsubseteq A_{3} \nsubseteq A_{4} ...$ are all sets containing an infinite number of elements, then the intersection $\cap_{n=1}^{\infty}A_{n}$ is infinite as well.
    - FALSE, say we have some natural number $m$ that we think will satisfy $m \in \cap_{n=1}^{\infty}A_{n}.$ This would mean that $m \in A_{n}$ (for *every*) $A_{n}$ in our collection of sets, but because $m$ is not a element of $A_{m+1}$ this results with no such $m$ existing and the intersection is empty. $$\bigcap_{n=1}^{\infty}A_{n} = \emptyset$$

---

* (b) If $A_{1} \nsubseteq A_{2} \nsubseteq A_{3} \nsubseteq A_{4} ...$ are all finite, nonempty sets of real numbers, then the intersection $\cap_{n=1}^{\infty}$ is finite and nonempty. 

---

* (c) $A \cap (B \cup C) = (A \cap B) \cup C$

---

* (d) $A \cap (B \cap C) = (A \cap B) \cap C$

---

* (e) $A \cap (B \cup C) = (A \cap B) \cup (A \cap C)$

