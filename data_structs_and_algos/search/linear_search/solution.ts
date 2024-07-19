/// Solution: Iterate over each element in search space until the needle is found, or if not found return null. 
/// Time Complexity: O(n) because in the worst case we go through every element in search space.
function linearSearch(haystack: number[], needle: number): number | null {
  for (let i = 0; i < haystack.length; i++) {
    if (haystack[i] == needle) {
      return i;
    }
  }

  return null;
}

((/* Test: Can Find Needle ? */) => {
  let haystack = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let needle = 7;

  // The expected output is 6, bc the needle (7) is the 6th element.
  let expected = 6;
  let actual = linearSearch(haystack, needle);

  if (actual == expected) {
    console.log("Can Find Needle ? ✅"); 
  } else {
    console.log("Can Find Needle ? ❌"); 
  }
})();
 
((/* Test: Can Handle Empty Haystack ? */) => {
  let haystack: number[] = [];
  let needle = 3;

  // The expected output is null, bc the haystack does NOT contain the needle (3).
  let expected = null;
  let actual = linearSearch(haystack, needle);

  if (actual == expected) {
    console.log("Can Handle Empty Haystack ? ✅");
  } else {
    console.log("Can Handle Empty Haystack ? ❌");
  }
})();

((/* Test: Can Handle Needle Not In Haystack ? */) => {
  let haystack = [2, 4, 6, 8, 10];
  let needle = 5;

  // The expected output is null, bc the haystack does NOT contain the needle (5).
  let expected = null;
  let actual = linearSearch(haystack, needle);

  if (actual == expected) {
    console.log("Can Handle Needle Not In Haystack ? ✅");
  } else {
    console.log("Can Handle Needle Not In Haystack ? ❌");
  }
})();
