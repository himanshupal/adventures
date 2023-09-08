const data = [3, 153, 123, 367, 12356, 4641684];

function getSum(num: number): number {
  if (!num) return num;
  const lastDigit = num % 10;
  const rest = Math.floor(num / 10);
  const finalValue = lastDigit + getSum(rest);
  return finalValue >= 10 ? getSum(finalValue) : finalValue;
}

function getCube(num: number): number {
  return num * num * num;
}

function sumOfCubeOfDigits(num: number): number {
  if (!num) return num;
  const lastDigit = num % 10;
  const rest = Math.floor(num / 10);
  return getCube(lastDigit) + sumOfCubeOfDigits(rest);
}

function armstrongNumber(arr: number[]) {
  return arr.map((v) => (sumOfCubeOfDigits(v) === v ? "It is an ARMSTRONG number" : "It is NOT an ARMSTRONG number"));
}

console.log(armstrongNumber(data));
