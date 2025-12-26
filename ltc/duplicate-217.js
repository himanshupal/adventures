const input = [1, 2, 3, 1];

function solution1() {
  const n = input.length;

  for (let i = 0; i < n - 1; i++) {
    for (let j = i + 1; j < n; j++) {
      if (input[i] === input[j]) return true;
    }
  }

  return false;
}

function solution2() {
  const sorted = input.sort();

  for (let i = 1; i < sorted.length; ++i) {
    if (sorted[i] === sorted[i - 1]) return true;
  }

  return false;
}

function solution3() {
  return new Set(input).size !== input.length;
}

function solution4() {
  const set = new Set();

  for (const value of input) {
    if (set.has(value)) return true;
    set.add(value);
  }

  return false;
}

console.log(solution2());
