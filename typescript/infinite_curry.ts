interface InfiniteCurry<T> {
  [Symbol.toPrimitive](): T;
  (next: T): InfiniteCurry<T>;
}

function add<T = string | number>(initial: T): InfiniteCurry<T> {
  // @ts-ignore
  const v: InfiniteCurry<T> = (next: T) => add(initial + next);
  v[Symbol.toPrimitive] = () => initial;
  return v;
}

console.log(Number(add(1)(2)(3)(4)(5)) === 15);
console.log(String(add("W")("h")("a")("t")("!")) === "What!");
