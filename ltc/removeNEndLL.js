const { Node } = require("./node");

const list = new Node(1, new Node(2, new Node(3, new Node(4, new Node(5)))));
const toRemove = 3;

function removeNFromEnd1(head, n) {
  if (!head) return null;
  let fast = head;
  let slow = head;

  while (n--) fast = fast.next;
  if (!fast) return head.next;

  while (fast.next) {
    fast = fast.next;
    slow = slow.next;
  }

  slow.next = slow.next.next;

  return head;
}

function removeNFromEnd2(head, n) {
  let copy = head;
  let length = 0;
  let i = 1;

  while (copy) {
    copy = copy.next;
    length++;
  }

  if (length === n) return haed.next;

  for (copy = head; i < length - n; i++) {
    copy = copy.next;
  }

  copy.next = copy.next.next;
  return head;
}

console.log(JSON.stringify(removeNFromEnd2(list, toRemove), null, 2));
