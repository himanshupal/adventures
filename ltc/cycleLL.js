const { Node } = require("./node");

var list = new Node(1, new Node(2, new Node(3, new Node(4, new Node(5)))));

function checkCycle(head) {
  if (!head) return false;
  let slow = head;
  let fast = head;

  while (fast && fast.next) {
    fast = fast.next.next;
    slow = slow.next;

    if (slow === fast) {
      return true;
    }
  }

  return false;
}

console.log(checkCycle(list)); // false
