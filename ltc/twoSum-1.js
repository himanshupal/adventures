const input = [2, 7, 11, 15];
const toFind = 9;

function solution1() {
  // No need to check last element as inner loop will go over it
  for (let i = 0; i < input.length - 1; ++i) {
    // Because we do not have to consider the same number twice
    for (let j = i + 1; j < input.length; ++j) {
      if (input[i] + input[j] === toFind) {
        return [i, j];
      }
    }
  }

  return [-1, -1];
}

function solution2() {
  const map = new Map();

  for (let i = 0; i < input.length; ++i) {
    map.set(input[i], i);
  }

  for (let i = 0; i < input.length; ++i) {
    const complement = toFind - input[i];
    if (map.get(complement) !== i) {
      return [i, map.get(complement)];
    }
  }

  return [-1, -1];
}

function solution3() {
  const map = new Map();

  for (let i = 0; i < input.length; ++i) {
    const complement = toFind - input[i];
    // Add current value to map as another value's complement will be this values itself
    if (!map.has(complement)) {
      map.set(input[i], i);
    } else {
      return [map.get(complement), i];
    }
  }

  return [-1, -1];
}

console.log(solution2());
