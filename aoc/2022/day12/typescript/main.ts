enum Move {
  TOP,
  RIGHT,
  BOTTOM,
  LEFT,
  _,
}

type Position = {
  x: number;
  y: number;
  v: number;
};

const data = Deno.readTextFileSync("../_input.txt");
const lines = data.split("\n").filter(Boolean);

const moves = new Map<Position, Array<Move>[]>();
let start: Position, end: Position;

for (let y = 0; y < lines.length; ++y) {
  const chars = lines[y].split("");
  for (let x = 0; x < chars.length; ++x) {
    if (chars[x] === "S") {
      start = { x, y, v: -1 };
      moves.set(start, []);
    } else if (chars[x] === "E") {
      end = { x, y, v: 26 };
      moves.set(end, []);
    } else {
      const pos = {
        v: chars[x].charCodeAt(0),
        x,
        y,
      };
      moves.set(pos, []);
    }
  }
}

// @ts-ignore: Start or End will be found till this point
if (!start || !end) {
  throw new Error("Start or end not found");
}

const findPath = (moves: Map<Position, Array<Move>[]>, from: Position, fromDir: Move, visited: Set<Position>): Array<Move> => {
  for (const key of moves.keys()) {
    if (JSON.stringify(key) === JSON.stringify(from)) continue;
    if (visited.has(from)) continue;
    visited.add(key);

    console.log({ visited, key });

    if (key.v === 26) {
      // const returnValue: Array<Position> = [];
      // for (const v of visited.values()) returnValue.push(v);
      // console.log("Found end:", returnValue);
      return [fromDir];
    }

    // Since we have scanned linearly, the items at bottom of matrix will be at higer inde, so for checking some index from above, we need to subtract 1 from y
    if (from.y > 0) {
      if (moves.has({ ...from, y: from.y - 1 })) {
        const above = { ...from, y: from.y - 1 };
        console.log!(`key found (above): ${above}`);
        const found = findPath(moves, above, Move.LEFT, visited);
        moves.get(key)?.push(found);
        console.log!("Moves found:", found);
        console.log!("Updated at location: %s, Moves: %s", from, moves.get(key));
      } else if (
        moves.has({
          ...from,
          y: from.y - 1,
          v: from.v + 1,
        })
      ) {
        const above = {
          ...from,
          y: from.y - 1,
          v: from.v + 1,
        };
        console.log!("key found (above + 1):", above);
        const found = findPath(moves, above, Move.LEFT, visited);
        moves.get(key)?.push(found);
        console.log!("Updated at location: %s, Moves: %s", from, moves.get(from));
      }
    }

    {
      if (moves.has({ ...from, x: from.x + 1 })) {
        const right = { ...from, x: from.x + 1 };
        console.log!("key found (right): %s", right);
        const found = findPath(moves, right, Move.LEFT, visited);
        moves.get(key)?.push(found);
        console.log!("Moves found: %s", found);
        console.log!("Updated at location: %s, Moves: %s", from, moves.get(from));
      } else if (
        moves.has({
          ...from,
          v: from.v + 1,
          x: from.x + 1,
        })
      ) {
        const right = {
          ...from,
          v: from.v + 1,
          x: from.x + 1,
        };
        console.log!("key found (right + 1): %s", right);
        const found = findPath(moves, right, Move.LEFT, visited);
        moves.get(key)?.push(found);
        console.log!("Moves found: %s", found);
        console.log!("Updated at location: %s, Moves: %s", from, moves.get(from));
      }
    }

    {
      if (moves.has({ ...from, y: from.y + 1 })) {
        const below = { ...from, y: from.y + 1 };
        console.log!("key found (below): %s", below);
        const found = findPath(moves, below, Move.BOTTOM, visited);
        moves.get(key)?.push(found);
        console.log!("Moves found: %s", found);
        console.log!("Updated at location: %s, Moves: %s", from, moves.get(from));
      } else if (
        moves.has({
          ...from,
          v: from.v + 1,
          y: from.y + 1,
        })
      ) {
        const below = {
          ...from,
          v: from.v + 1,
          y: from.y + 1,
        };
        console.log!("key found (below + 1): %s", below);
        const found = findPath(moves, below, Move.BOTTOM, visited);
        moves.get(key)?.push(found);
        console.log!("Moves found: %s", found);
        console.log!("Updated at location: %s, Moves: %s", from, moves.get(from));
      }
    }

    if (from.x > 0) {
      if (moves.has({ ...from, x: from.x - 1 })) {
        const left = { ...from, x: from.x - 1 };
        console.log!("key found (left): %s", left);
        const found = findPath(moves, left, Move.RIGHT, visited);
        moves.get(key)?.push(found);
        console.log!("Moves found: %s", found);
        console.log!("Updated at location: %s, Moves: %s", from, moves.get(from));
      } else if (
        moves.has({
          ...from,
          v: from.v + 1,
          x: from.x - 1,
        })
      ) {
        const left = {
          ...from,
          v: from.v + 1,
          x: from.x - 1,
        };
        console.log!("key found (left + 1): %s", left);
        const found = findPath(moves, left, Move.RIGHT, visited);
        moves.get(key)?.push(found);
        console.log!("Moves found: %s", found);
        console.log!("Updated at location: %s, Moves: %s", from, moves.get(from));
      }
    }
  }

  return [];
};

console.log(findPath(moves, start, Move._, new Set<Position>()));
