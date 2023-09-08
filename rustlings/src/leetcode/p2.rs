use super::{p21::ListNode, p83::append_to_list};

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut answer: Option<Box<ListNode>> = None;
    let mut carry = 0;

    loop {
        match (l1.clone(), l2.clone()) {
            (Some(v1), Some(v2)) => {
                let mut val = v1.val + v2.val + carry;
                if val > 9 {
                    carry = val / 10;
                    val %= 10;
                } else {
                    carry = 0;
                }
                append_to_list(&mut answer, val);
                l1 = v1.next;
                l2 = v2.next;
            }
            (Some(v1), None) => {
                let mut val = v1.val + carry;
                if val > 9 {
                    carry = val / 10;
                    val %= 10;
                } else {
                    carry = 0;
                }
                append_to_list(&mut answer, val);
                l1 = v1.next;
            }
            (None, Some(v2)) => {
                let mut val = v2.val + carry;
                if val > 9 {
                    carry = val / 10;
                    val %= 10;
                } else {
                    carry = 0;
                }
                append_to_list(&mut answer, val);
                l2 = v2.next;
            }
            _ => {
                if carry != 0 {
                    append_to_list(&mut answer, carry);
                }
                break;
            }
        }
    }

    answer
}

#[test]
fn add_two_numbers_test() {
    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None }))
            }))
        }))
    );

    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode { val: 0, next: None })),
            Some(Box::new(ListNode { val: 0, next: None }))
        ),
        Some(Box::new(ListNode { val: 0, next: None }))
    );

    assert_eq!(
        add_two_numbers(
            Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode {
                                    val: 9,
                                    next: Some(Box::new(ListNode { val: 9, next: None }))
                                }))
                            }))
                        }))
                    }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode { val: 9, next: None }))
                    }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode { val: 1, next: None }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }))
    );
}
