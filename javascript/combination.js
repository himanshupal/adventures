function gen_combinations(str) {
  const combinations = [];

  for (let i = 0; i < str.length; ++i) {
    if (str[i] === " ") {
      const combination = str.slice(0, i) + str.slice(i + 1);
      combinations.push(combination);
    }
  }

  return combinations.length > 10
    ? combinations.flatMap((t) => gen_combinations(t))
    : combinations;
}

console.log(
  gen_combinations(
    "Lorem ipsum dolor sit amet consectetur adipisicing elit. Obcaecati eum soluta distinctio corporis ab cum dignissimos illum beatae. Debitis voluptatum dolor autem quod eos dolorum!"
  )
);
