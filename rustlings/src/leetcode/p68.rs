pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let mut final_vec = Vec::<String>::new();
    let mut words_read = 0;

    while words_read != words.len() {
        let mut joined_string = String::new();
        let mut space_indexes = Vec::<usize>::new();

        loop {
            match words.get(words_read) {
                Some(w) => {
                    if w.len() + joined_string.len() > max_width as usize {
                        joined_string.pop();
                        space_indexes.pop();
                        break;
                    }
                    space_indexes.push(w.len() + joined_string.len());
                    joined_string.push_str(&format!("{} ", w));
                    words_read += 1;
                }
                None => {
                    joined_string.pop();
                    space_indexes.pop();
                    break;
                }
            }
        }

        let space_count = space_indexes.len();

        println!(
            "temp: |{joined_string}| length: {} has {} words, indexes: {:?}",
            joined_string.len(),
            space_count + 1,
            space_indexes
        );

        let space_needed = max_width as usize - joined_string.len();

        println!("Space needed: {space_needed}");

        if (space_count + 1) > 1 && words_read != words.len() {
            let mut odd_one_out = false;
            let mut space_to_distribute = if space_needed > (space_count + 1) {
                space_needed / space_count
            } else {
                space_needed
            };

            println!("Space to distribute: {space_to_distribute}");

            for index in 0..(space_to_distribute).min(space_count) {
                if space_needed <= space_count {
                    println!("Space: {index} 1 {}", joined_string.len());
                    joined_string.insert(space_indexes[index] + index, ' ');
                } else if space_needed % space_count != 0 {
                    odd_one_out = true;
                    let extra_space = space_needed % space_count;
                    let remaining_space = space_needed - extra_space;
                    println!("Catch... {extra_space} {remaining_space} {space_to_distribute}");
                    space_to_distribute = remaining_space / space_count;
                    joined_string.insert_str(
                        space_indexes[index] + (space_to_distribute * index),
                        &" ".repeat(space_to_distribute),
                    );
                } else {
                    println!("Space: {index} {space_to_distribute}");

                    joined_string.insert_str(
                        space_indexes[index] + (space_to_distribute * index),
                        &" ".repeat(space_to_distribute),
                    );
                };
            }

            if odd_one_out {
                joined_string.insert(space_indexes[0], ' ');
            }
        } else {
            joined_string.push_str(&" ".repeat(space_needed));
        }

        final_vec.push(joined_string);
    }

    final_vec
}

#[test]
fn full_justify_test() {
    assert_eq!(
        full_justify(
            vec![
                "Don't", "go", "around", "saying", "the", "world", "owes", "you", "a", "living;",
                "the", "world", "owes", "you", "nothing;", "it", "was", "here", "first."
            ]
            .iter()
            .map(|v| v.to_string())
            .collect(),
            30
        ),
        vec![
            "Don't  go  around  saying  the",
            "world  owes  you a living; the",
            "world owes you nothing; it was",
            "here first.                   "
        ]
    );
    assert_eq!(
        full_justify(
            vec![
                "This",
                "is",
                "an",
                "example",
                "of",
                "text",
                "justification."
            ]
            .iter()
            .map(|v| v.to_string())
            .collect(),
            16
        ),
        vec!["This    is    an", "example  of text", "justification.  "]
    );
    assert_eq!(
        full_justify(
            vec!["What", "must", "be", "acknowledgment", "shall", "be"]
                .iter()
                .map(|v| v.to_string())
                .collect(),
            16
        ),
        vec!["What   must   be", "acknowledgment  ", "shall be        "]
    );
    assert_eq!(
        full_justify(
            vec![
                "Science",
                "is",
                "what",
                "we",
                "understand",
                "well",
                "enough",
                "to",
                "explain",
                "to",
                "a",
                "computer.",
                "Art",
                "is",
                "everything",
                "else",
                "we",
                "do"
            ]
            .iter()
            .map(|v| v.to_string())
            .collect(),
            20
        ),
        vec![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  "
        ]
    );
    assert_eq!(
        full_justify(
            vec![
                "The",
                "important",
                "thing",
                "is",
                "not",
                "to",
                "stop",
                "questioning.",
                "Curiosity",
                "has",
                "its",
                "own",
                "reason",
                "for",
                "existing."
            ]
            .iter()
            .map(|v| v.to_string())
            .collect(),
            17
        ),
        vec![
            "The     important",
            "thing  is  not to",
            "stop questioning.",
            "Curiosity has its",
            "own   reason  for",
            "existing.        "
        ]
    );
}
