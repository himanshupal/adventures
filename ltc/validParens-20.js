const input = "()[]{}[";

function solution1() {
  const stack = [];
  const mapping = { ")": "(", "}": "{", "]": "[" };

  for (const char of input.split("")) {
    if (char in mapping) {
      const top = stack.length ? stack.pop() : "#";
      if (mapping[char] !== top) {
        return false;
      }
    } else {
      stack.push(char);
    }
  }

  return !stack.length;

  // stack = []
  //       mapping = {')': '(', '}': '{', ']': '['}

  //       for char in s:
  //           if char in mapping:
  //               top_element = stack.pop() if stack else '#'
  //               if mapping[char] != top_element:
  //                   return False
  //           else:
  //               stack.append(char)

  //       return not stack
}

console.log(solution1());
