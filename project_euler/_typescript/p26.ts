const regex = /\d\.(\d+)?(\d+)\2+(\d+)?/;

let max = 0;
let value = 0;

for (let i = 2; ++i < 100; ) {
  const decimal = String(1 / i);
  // console.log({ i, decimal });

  const match = regex.exec(decimal);
  if (!match) continue;

  const [_, m, n] = match;
  console.log(i, m, n);

  // if (n.length > max) {
  //   max = n.length;
  //   value = i;
  // }
}

console.log({ max, value });
