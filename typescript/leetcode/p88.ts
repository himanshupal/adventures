import { assertEquals } from "https://deno.land/std/assert/assert_equals.ts";

/**
 Do not return anything, modify nums1 in-place instead.
 */
function merge(nums1: number[], m: number, nums2: number[], n: number): number[] {
  let found = 0;
  for (let i = 0; i < m; ++i) {
    const next = nums1.at(i + 1) || 0;
    if (n < nums2[found] && nums2[found] >= next) {
      console.log(n, nums2[found], next);
      found += 1;
    }
    nums1.pop();
  }
  return nums1;
}

Deno.test("Merge Test", function () {
  assertEquals(merge([1, 2, 3, 8, 9, 0, 0, 0, 0, 0], 5, [2, 5, 6, 8, 10], 5), [1, 2, 2, 3, 5, 6, 8, 8, 9, 10]);
});
