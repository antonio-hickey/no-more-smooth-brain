# Set Theory Terminology

* __set__: basically a collection of things, kind of like a hashset, but there's no order.

* __$∈$__: "element of", like `A.includes(x) == true` where `x` is a element in hashset `A`.

* __$∉$__: "not element of", like `A: Hashset.includes(x) == false` where `x` is NOT a element in hashset `A`.

* __$∪$__: "union" (think merge of sets), like `C = A.concat(B)` where `C` is a hashset containing all the elements in both hashsets `A` and `B`.

* __$∩$__: "Intersection" (elements in both sets), like `A.includes(x) && B.includes(x) == true` where x is a element in BOTH hashsets `A` and `B`.

* __$⊆$__: "subset" (set fully contained in set), like `A = [1, 2, 3] && B = [1, 2]` where `B` is a subset of `A`.
