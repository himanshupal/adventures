const { Node } = require("./node");

let list = new Node(1, new Node(2, new Node(3, new Node(4, new Node(5)))));

function reverseList1(head) {
  let reversed;

  while (head) {
    const current = head.next;
    head.next = reversed;
    reversed = head;
    head = current;
  }

  return reversed;
}

function reverseList2(head) {
  let reversed;

  while (head) {
    reversed = new Node(head.value, reversed);
    head = head.next;
  }

  return reversed;
}

function reverseList3(head) {
  if (!head?.next) return head;

  const newHead = reverseList3(head.next);
  head.next.next = head;
  head.next = undefined;
  return newHead;
}

function reverseList4(head, prev) {
  if (!head) return prev;
  const next = head.next;
  head.next = prev;
  return reverseList4(next, head);
}

console.log(JSON.stringify(reverseList4(list), null, 2));
