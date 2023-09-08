// @ts-check

/**
 * @description Simple but terrible sort method
 * @param {number[]} array
 */
function bubbleSort(array) {
  const length = array.length;
  for (let i = 0; i < length; ++i) {
    for (let j = 0; j < length - i - 1; ++j) {
      if (array[j] > array[j + 1]) {
        [array[j], array[j + 1]] = [array[j + 1], array[j]];
      }
    }
  }
  return array;
}

console.log(bubbleSort([1, 3, 2, 5, 2, 4]));
