use super::p21::ListNode;

pub fn append_to_list(list: &mut Option<Box<ListNode>>, val: i32) {
    if let Some(l) = list {
        append_to_list(&mut l.next, val);
    } else {
        *list = Some(Box::new(ListNode { val, next: None }))
    }
}

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result: Option<Box<ListNode>> = None;
    let mut val = i32::MIN;

    loop {
        if let Some(v) = head {
            if let Some(r) = &mut result {
                if v.val != val {
                    val = v.val;
                    append_to_list(&mut result, v.val);
                }
            } else {
                val = v.val;
                append_to_list(&mut result, v.val);
            }
            head = v.next;
        } else {
            break;
        }

        // This is much slower due to usage of clone imo
        /* match (&mut head, &mut result) {
            (Some(v), None) => {
                val = v.val;
                result = Some(Box::new(ListNode { val, next: None }))
            }
            (Some(v), Some(r)) => {
                if v.val != val {
                    val = v.val;
                    append_to_list(&mut result, val);
                }
                head = v.next.clone();
            }
            _ => break,
        } */
    }

    result
}

#[test]
fn delete_duplicates_test() {
    assert_eq!(delete_duplicates(None), None);
    assert_eq!(
        delete_duplicates(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None }))
        }))
    );
    assert_eq!(
        delete_duplicates(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None }))
            }))
        }))
    );
}
