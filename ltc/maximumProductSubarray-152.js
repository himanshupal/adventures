// Case1 :- All the elements are positive : Then your answer will be product of all the elements in the array.
// Case2 :- Array have positive and negative elements both :
// If the number of negative elements is even then again your answer will be complete array because on multiplying all the negative numbers it will become positive.
// If the number of negative elements is odd then you have to remove just one negative element and for that u need to check your subarrays to get the max product.
// Case3 :- Array also contains 0 : Then there will be not much difference...its just that your array will be divided into subarray around that 0. What u have to so is just as soon as your product becomes 0 make it 1 for the next iteration, now u will be searching new subarray and previous max will already be updated.
// *(These cases are much clear in approach 3)

const input = [2, 3, -2, -4];
// const input = [-2, 0, -1];

function solution1() {
  let cur_max = input[0];
  let cur_min = input[0];
  let ans = input[0];
  for (let i = 1; i < input.length; i++) {
    const x = input[i];
    const tmp = cur_max;
    cur_max = Math.max(x, x * cur_max, x * cur_min);
    cur_min = Math.min(x, x * tmp, x * cur_min);
    ans = Math.max(ans, cur_max);
  }
  return ans;
}

function solution2() {
  let cur_max = input[0];
  let cur_min = input[0];
  let ans = input[0];
  for (let i = 1; i < input.length; i++) {
    const x = input[i];

    if (x < 0) {
      [cur_min, cur_max] = [cur_max, cur_min];
    }

    cur_max = Math.max(x, x * cur_max);
    cur_min = Math.min(x, x * cur_min);

    ans = Math.max(ans, cur_max);
  }
  return ans;
}

function solution3() {
  let l = 1;
  let r = 1;
  let ans = input[0];

  for (let i = 0; i < input.length; i++) {
    //if any of l or r become 0 then update it to 1
    l = l == 0 ? 1 : l;
    r = r == 0 ? 1 : r;

    l *= input[i]; //prefix product
    r *= input[input.length - 1 - i]; //suffix product

    ans = Math.max(ans, Math.max(l, r));
  }

  return ans;
}

console.log(solution3());
