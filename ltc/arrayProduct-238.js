// @ts-check

// @ts-ignore
const input = [1, 2, 3, 4];

// Brute force
function solution1() {
  const final = [];

  for (let i = 0; i < input.length; i++) {
    let product = 1;
    for (let j = 0; j < input.length; j++) {
      if (i === j) continue;
      product *= input[j];
    }
    final[i] = product;
  }

  return final;
}

// Division approach; doesn't play well with zeroes
function solution2() {
  let product = 1;

  for (const value of input) {
    product *= value;
  }

  const final = [];
  for (let i = 0; i < input.length; i++) {
    final[i] = product / input[i];
  }
  return final;
}

// Prefix/Suffix
function solution3() {
  const prefix = new Array(input.length).fill(1);
  const suffix = new Array(input.length).fill(1);

  for (let i = 1; i < input.length; ++i) {
    prefix[i] = prefix[i - 1] * input[i - 1];
  }

  for (let i = input.length - 2; i >= 0; --i) {
    suffix[i] = suffix[i + 1] * input[i + 1];
  }

  const final = [];
  for (let i = 0; i < input.length; ++i) {
    final[i] = prefix[i] * suffix[i];
  }

  return final;
}

// Prefix/Suffix - Single pass
function solution4() {
  let pointer = 1;
  const count = input.length;
  const final = new Array(count).fill(1);

  for (let i = 0; i < count; i++) {
    final[i] *= pointer;
    pointer *= input[i];
  }

  pointer = 1;
  for (let i = count - 1; i >= 0; i--) {
    final[i] *= pointer;
    pointer *= input[i];
  }

  return final;
}

function solution5() {
  const final = new Array(input.length).fill(1);
  for (let i = 1; i < input.length; i++) {
    final[i] = final[i - 1] * input[i - 1];
  }
  for (let i = input.length - 1, suffixProd = 1; i >= 0; i--) {
    final[i] *= suffixProd; // multiply stored prefix product with suffix product
    suffixProd *= input[i]; // update suffix product
  }
  return final;
}

function solution6() {
  const final = new Array(input.length).fill(1);
  for (let i = 0, suf = 1, pre = 1, n = input.length; i < n; i++) {
    final[i] *= pre; // similar to prefix product being calculated in 1st loop of previous solution
    pre *= input[i];
    final[n - 1 - i] *= suf; // similar to suffix product being calculated in 2nd loop of previous solution
    suf *= input[n - 1 - i];
  }
  return final;
}

console.log(solution6());
