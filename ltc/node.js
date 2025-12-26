class Node {
  /**
   * @type {number|undefined}
   */
  value;
  /**
   * @type {Node|undefined}
   */
  next;

  /**
   * @param {number|undefined} value
   * @param {Node|undefined} next
   */
  constructor(value, next) {
    this.value = value;
    this.next = next;
  }
}

module.exports = {
  Node,
};
