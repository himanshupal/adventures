const { Node } = require("./node");

const list1 = new Node(1, new Node(1, new Node(3, new Node(6, new Node(8)))));
const list2 = new Node(2, new Node(4, new Node(5, new Node(7, new Node(8)))));

function mergeSortedLists1(list1, list2) {
  let newList = new Node();
  let cursor = newList;

  if (!list1 && !list2) return newList;

  while (list1 && list2) {
    if (list1.value < list2.value) {
      cursor.next = list1;
      list1 = list1.next;
    } else {
      cursor.next = list2;
      list2 = list2.next;
    }
    cursor = cursor.next;
  }

  cursor.next = list1 || list2;

  return newList.next;
}

// Drawback of not being able to match if the same node has repeated values
function mergeSortedLists2(l1, l2) {
  if (!l1) return l2;
  if (!l2) return l1;
  if (l1.val <= l2.val) {
    l1.next = mergeSortedLists2(l1.next, l2);
    return l1;
  }
  l2.next = mergeSortedLists2(l2.next, l1);
  return l2;
}

console.log(JSON.stringify(mergeSortedLists2(list1, list2), null, 2));
