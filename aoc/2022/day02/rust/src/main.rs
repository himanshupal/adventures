use std::fs;

fn main() {
    let m_index = vec!["X", "Y", "Z"];
    let o_index = vec!["A", "B", "C"];

    if let Ok(data) = fs::read_to_string("../input.txt") {
        let sums = data
            .lines()
            .map(|chars| {
                let letters = chars.split(' ').collect::<Vec<&str>>();
                let (o, m) = (letters[0], letters[1]);
                let o_idx = get_index(o_index.clone(), o).unwrap();
                let m_idx = get_index(m_index.clone(), m).unwrap();

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

                let part2 = if m == "X" {
                    /* 'l' */
                    if o_idx == 0 {
                        (2 + 1) + 0
                    } else {
                        (o_idx - 1 + 1) + 0
                    }
                } else if m == "Y" {
                    /* 'd' */
                    (o_idx + 1) + 3
                } else if m == "Z" {
                    /* 'w' */
                    if o_idx == 2 {
                        (0 + 1) + 6
                    } else {
                        (o_idx + 1 + 1) + 6
                    }
                } else {
                    0
                };

                return (res, part2);
            })
            .collect::<Vec<(usize, usize)>>();

        let (sum1, sum2) = sums
            // .iter()
            // .copied()
            .into_iter()
            .reduce(|(p_sum1, p_sum2), (sum1, sum2)| (p_sum1 + sum1, p_sum2 + sum2))
            .unwrap();

        println!("Final score: {sum1}, Part two: {sum2}")
    }
}

fn get_index(v: Vec<&str>, s: &str) -> Option<usize> {
    return v.iter().position(|&ch| &ch == &s);
}
