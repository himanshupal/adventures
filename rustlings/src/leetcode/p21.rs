#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn append_to_list(list: &mut Option<Box<ListNode>>, val: i32) {
    if let Some(l) = list {
        if val > l.val {
            append_to_list(&mut l.next, val);
        } else {
            *list = Some(Box::new(ListNode {
                val,
                next: list.to_owned(),
            }))
        }
    } else {
        *list = Some(Box::new(ListNode { val, next: None }))
    }
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut merged_list: Option<Box<ListNode>> = None;

    loop {
        if let Some(v) = list1 {
            append_to_list(&mut merged_list, v.val);
            list1 = v.next;
        }

        if let Some(v) = list2 {
            append_to_list(&mut merged_list, v.val);
            list2 = v.next;
        }

        if list1.is_none() && list2.is_none() {
            break;
        }
    }

    merged_list
}

#[test]
fn merge_two_lists_test() {
    assert_eq!(
        merge_two_lists(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 5, next: None }))
                        }))
                    }))
                }))
            }))
        }))
    );
    assert_eq!(merge_two_lists(None, None), None);
    assert_eq!(
        merge_two_lists(None, Some(Box::new(ListNode { val: 0, next: None }))),
        Some(Box::new(ListNode { val: 0, next: None }))
    );
    assert_eq!(
        merge_two_lists(
            Some(Box::new(ListNode { val: 5, next: None })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None }))
                }))
            }))
        }))
    );
}
