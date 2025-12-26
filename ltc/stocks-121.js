const input = [7, 1, 5, 3, 6, 4];

function solution1() {
  let min = Infinity;
  let max = 0;

  for (const value of input) {
    if (value < min) {
      min = value;
      max = min;
    } else if (value > max) {
      max = value;
    }
  }

  return max - min;
}

function solution2() {
  let buy = input[0];
  let profit = 0;
  for (let i = 1; i < input.length; i++) {
    if (input[i] < buy) {
      buy = input[i];
    } else if (input[i] - buy > profit) {
      profit = input[i] - buy;
    }
  }
  return profit;
}

function solution3() {
  let buy = input[0];
  let profit = 0;
  for (let i = 1; i < input.length; i++) {
    buy = Math.min(input[i], buy);
    profit = Math.max(profit, input[i] - buy);
  }
  return profit;
}

function solution4() {
  let maxProfit = 0;
  let left = 0;
  let right = 1;

  while (right < input.length) {
    const sellPrice = input[right];
    const buyPrice = input[left];

    if (buyPrice < sellPrice) {
      maxProfit = Math.max(maxProfit, sellPrice - buyPrice);
    } else {
      left = right;
    }

    right++;
  }

  return maxProfit;
}

console.log(solution1());
