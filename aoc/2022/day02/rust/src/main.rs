use std::{collections::HashMap, fs};

fn main() {
    let index = vec!["X", "Y", "Z"];
    let key_map = HashMap::from([("A", index[0]), ("B", index[1]), ("C", index[2])]);

    fn get_index(v: Vec<&str>, s: &str) -> Option<usize> {
        return v.iter().position(|&ch| &ch == &s);
    }

    if let Ok(data) = fs::read_to_string("../input.txt") {
        let sum: usize = data
            .lines()
            .map(|chars| {
                let letters = chars.split(' ').collect::<Vec<&str>>();
                let (o, m) = (letters[0], letters[1]);
                let o_idx = get_index(index.clone(), key_map.get(o).unwrap().clone()).unwrap();
                let m_idx = get_index(index.clone(), m).unwrap();

                let res = if m_idx == 0 && o_idx == 2 {
                    /* 'w' */
                    1 + 6
                } else if m_idx == 2 && o_idx == 0 {
                    /* 'l' */
                    3 + 0
                } else if m_idx > o_idx {
                    /* 'w' */
                    (m_idx + 1) + 6
                } else if m_idx < o_idx {
                    /* 'l' */
                    (m_idx + 1) + 0
                } else {
                    /* 'd' */
                    (m_idx + 1) + 3
                };

                return res;
            })
            .sum();

        println!("Final score: {sum}")
    }
}
