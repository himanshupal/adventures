const getLines = async (): Promise<Array<string>> => {
  let lines = "";
  const decoder = new TextDecoder();
  const file = await Deno.open("../input.txt");
  for await (const chunk of file.readable) lines += decoder.decode(chunk);
  return lines.split("\n").filter(Boolean);
};

const lines = await getLines();
const lut = [...Array.from({ length: 9 }, (_, i) => `${i + 1}`)];

const solve = (lut: string[]) => {
  return lines.reduce((p, line) => {
    const found: string[] = [];

    for (const match of line.matchAll(new RegExp(`(?=(${lut.join("|")}))`, "g"))) {
      const matchFound = match.pop();
      if (!matchFound) continue;
      found.push(matchFound);
    }

    const numericValues = found.map((value) => (Number.isNaN(+value) ? lut.indexOf(value) + 1 : +value));

    const firstValue = numericValues.shift();
    const lastValue = numericValues.pop() || firstValue;

    return p + Number(`${firstValue!}${lastValue!}`);
  }, 0);
};

const part1Solution = solve(lut);
const part2Solution = solve(["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", ...lut]);

console.log("Part 1:", part1Solution);
console.log("Part 2:", part2Solution);
