const { Node } = require("./node");

const list = new Node(1, new Node(2, new Node(3, new Node(4, new Node(5)))));

function reorderList(head) {}

console.log(JSON.stringify(reorderList(list), null, 2));
