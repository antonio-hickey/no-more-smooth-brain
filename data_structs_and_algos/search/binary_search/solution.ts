/// Solution: Compare the midpoint of the search space to the target dividing the haystack in half
/// every iteration until the needle is found or it's not found to be in the haystack. The haystack
/// is ordered, so if the midpoint is less than the needle we can remove every succeeding element or 
/// if the midpoint is greator than the needle we can remove every preceding element.
///
/// Time Complexity: O(log(n)) because in the worst case we go through every element in search space.
function binarySearch(haystack: number[], needle: number): number | null {
  let searchSpaceStart = 0;
  let searchSpaceEnd = haystack.length - 1;

  while (searchSpaceStart <= searchSpaceEnd) {
    let midpoint = Math.floor((searchSpaceStart + searchSpaceEnd) / 2);

    if (haystack[midpoint] < needle) {
      searchSpaceStart = midpoint + 1;
    } else if (haystack[midpoint] > needle) {
      searchSpaceEnd = midpoint - 1;
    } else {
      return midpoint;
    }
  }
  
  return null;
}

((/* Test: Can Find Needle ? */) => {
  let haystack = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let needle = 7;

  // The expected output is 6, bc the needle (7) is the 6th element.
  let expected = 6;
  let actual = binarySearch(haystack, needle);

  if (actual == expected) {
    console.log("Can Find Needle ? ✅"); 
  } else {
    console.log("Can Find Needle ? ❌"); 
  }
})();

((/* Test: Can Find Needle In Odd Haystack ? */) => {
  let haystack = [1, 2, 3, 4, 5]; 
  let needle = 4;

  // The expected output is 3, bc the needle (4) is the 3rd element.
  let expected = 3;
  let actual = binarySearch(haystack, needle);

  if (actual == expected) {
    console.log("Can Find Needle In Odd Haystack ? ✅");
  } else {
    console.log("Can Find Needle In Odd Haystack ? ❌");
  }
})();
 
((/* Test: Can Handle Empty Haystack ? */) => {
  let haystack: number[] = [];
  let needle = 3;

  // The expected output is null, bc the haystack does NOT contain the needle (3).
  let expected = null;
  let actual = binarySearch(haystack, needle);

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
  let actual = binarySearch(haystack, needle);

  if (actual == expected) {
    console.log("Can Handle Needle Not In Haystack ? ✅");
  } else {
    console.log("Can Handle Needle Not In Haystack ? ❌");
  }
})();
