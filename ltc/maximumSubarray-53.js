// @ts-check
// https://leetcode.com/problems/maximum-subarray/solutions/1595195/c-python-7-simple-solutions-w-explanation-brute-force-dp-kadane-divide-conquer

// Prefer solution 4 or 7

// @ts-ignore
// const input = [1];
// const input = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
const input = [5, 4, -1, 7, 8];

/**
 * Time Complexity : O(N2), where N is the number of elements in nums. There are N*(N+1)/2 total sub-arrays and trying out each one takes time of O(N2)
 * Space Complexity : O(1)
 */
function solution1() {
  let ans = Number.NEGATIVE_INFINITY;

  for (let i = 0; i < input.length; i++) {
    let current_sum = 0;

    for (let j = i; j < input.length; j++) {
      current_sum += input[j];
      ans = Math.max(ans, current_sum);
    }
  }

  return ans;
}

// Recursive
/**
 * Time Complexity : O(N2), we are basically considering every subarray sum and choosing maximum of it.
 * Space Complexity : O(N), for recursive space
 */
function solution2() {
  function solve(i, pick) {
    if (i >= input.length) return pick ? 0 : Number.NEGATIVE_INFINITY;
    return Math.max(input[i] + solve(i + 1, true), pick ? 0 : solve(i + 1, false));
  }

  return solve(0, false);
}

// Tabulation
/**
 * Time Complexity : O(N), we are just iterating over the nums array once to compute the dp array and once more over the dp at the end to find maximum subarray sum. Thus overall time complexity is O(N) + O(N) = O(N)
 * Space Complexity : O(N), for maintaining dp.
 */
function solution3() {
  const lut = new Array(2).fill(new Array(input.length).fill(0));
  lut[1][0] = lut[0][0] = input[0];

  for (let i = 1; i < input.length; i++) {
    lut[1][i] = Math.max(input[i], input[i] + lut[1][i - 1]);
    lut[0][i] = Math.max(lut[0][i - 1], lut[1][i]);
  }

  return lut[0][-1];
}

/**
 * Kadane's algo
 * Time Complexity : O(N), for iterating over nums once
 * Space Complexity : O(1), only constant extra space is being used.
 */
function solution4() {
  let max = 0;
  let current_max = Number.NEGATIVE_INFINITY;

  for (const value of input) {
    max = Math.max(value, max + value);
    current_max = Math.max(current_max, max);
  }

  return current_max;
}

/**
 * Time Complexity : O(NlogN), the recurrence relation can be written as T(N) = 2T(N/2) + O(N) since we are recurring for left and right half (2T(N/2)) and also calculating maximal subarray including mid element which takes O(N) to calculate. Solving this recurrence using master theorem, we can get the time complexity as O(NlogN)
 * Space Complexity : O(logN), required by recursive stack
 */
function solution5() {
  /**
   * @param {typeof input} list
   * @param {number} left
   * @param {number} right
   */
  function solve(list, left, right) {
    if (left > right) return Number.NEGATIVE_INFINITY;
    let [mid, leftSum, rightSum, currentSum] = [Math.floor((left + right) / 2), 0, 0, 0];
    for (let i = mid - 1; i > left - 1; --i) {
      leftSum = Math.max(leftSum, (currentSum += list[i]));
    }
    currentSum = 0;
    for (let i = mid + 1; i < right + 1; ++i) {
      rightSum = Math.max(rightSum, (currentSum += list[i]));
    }
    return Math.max(solve(list, left, mid - 1), solve(list, mid + 1, right), leftSum + list[mid] + rightSum);
  }

  return solve(input, 0, input.length - 1);
}

/**
 * Time Complexity : O(N), the recurrence relation can be written as T(N) = 2T(N/2) + O(1) since we are recurring for left and right half (2T(N/2)) and calculating maximal subarray including mid element in O(1). Solving this recurrence using master theorem, we can get the time complexity as O(N)
 * Space Complexity : O(N), required by suf and pre.
 */
function solution6() {
  const [prefix, suffix] = [[...input], [...input]];

  for (let i = 1; i < input.length; ++i) prefix[i] += Math.max(0, prefix[i - 1]);
  for (let i = input.length - 2; i >= 0; --i) suffix[i] += Math.max(0, suffix[i + 1]);

  /**
   * @param {typeof input} list
   * @param {number} left
   * @param {number} right
   */
  function solve(list, left, right) {
    if (left === right) return list[left];
    const mid = Math.floor((left + right) / 2);
    return Math.max(solve(list, left, mid), solve(list, mid + 1, right), prefix[mid] + suffix[mid + 1]);
  }

  return solve(input, 0, input.length - 1);
}

function solution7() {
  let maxi = input[0];

  let start = 0;
  let end = 0;
  let sum = 0;
  let s = 0; // the temporary start position

  for (let i = 0; i < input.length; i++) {
    sum += input[i];

    if (sum > maxi) {
      maxi = sum;
      start = s;
      end = i;
    }

    if (sum < 0) {
      sum = 0;
      s = i + 1;
    }
  }

  console.log(start, end);

  return maxi;
}

console.log(solution7());
