const { Node } = require("./node");

const list = new Node(1, new Node(2, new Node(3, new Node(4, new Node(5)))));

// Not working
function swapNodes1(head, k) {
  const list = [];
  let temp = list;

  while (temp) {
    list.push(temp);
    temp = temp.next;
  }

  const tempValue = list[k];
  list[k] = list[-k];
  list[-k] = tempValue;

  return list;
}

function swapNodes2(head, k) {
  let left = head;
  while (--k) left = left.next;

  let temp = left;
  let right = head;
  while (temp.next) {
    right = right.next;
    temp = temp.next;
  }

  [left.value, right.value] = [right.value, left.value];

  return head;
}

console.log(JSON.stringify(swapNodes2(list, 1), null, 2));
