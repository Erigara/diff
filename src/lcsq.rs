use std::cmp::max;

#[derive(Clone, Debug)]
pub enum Diff {
    Same,
    Added,
    Substracted,
}

fn lcsq_lengths(s: &[u64], t: &[u64]) -> Vec<usize> {
    let s_len = s.len();
    let t_len = t.len();
    let mut lengths_matrix = vec![0; (s_len + 1) * (t_len + 1)];
    for i in 1..s_len + 1 {
        for j in 1..t_len + 1 {
            if s[i - 1] == t[j - 1] {
                lengths_matrix[(t_len + 1) * i + j] =
                    1 + lengths_matrix[(t_len + 1) * (i - 1) + (j - 1)];
            } else {
                lengths_matrix[(t_len + 1) * i + j] = max(
                    lengths_matrix[(t_len + 1) * (i - 1) + j],
                    lengths_matrix[(t_len + 1) * i + (j - 1)],
                );
            }
        }
    }
    lengths_matrix
}

pub fn get_lcsq(s: &[u64], t: &[u64]) -> Vec<u64> {
    let lengths_matrix = lcsq_lengths(s, t);
    let s_len = s.len();
    let t_len = t.len();
    let lcsq_len = lengths_matrix[lengths_matrix.len() - 1];
    let mut lcsq = vec![0; lcsq_len];

    let mut i = s_len;
    let mut j = t_len;
    let mut z = lcsq_len;

    while i > 0 && j > 0 && z > 0 {
        if s[i - 1] == t[j - 1] {
            //lcsq.push_front(s[i - 1]);
            lcsq[z - 1] = s[i - 1];
            z -= 1;
            i -= 1;
            j -= 1;
        } else if lengths_matrix[(t_len + 1) * i + (j - 1)]
            > lengths_matrix[(t_len + 1) * (i - 1) + j]
        {
            j -= 1;
        } else {
            i -= 1;
        }
    }
    lcsq
}

pub fn get_rev_diff(s: &[u64], t: &[u64]) -> Vec<Diff> {
    let lengths_matrix = lcsq_lengths(s, t);
    let s_len = s.len();
    let t_len = t.len();
    let mut i = s_len;
    let mut j = t_len;
    let mut rev_diff: Vec<Diff> = Vec::new();
    while i > 0 || j > 0 {
        if i == 0 {
            j -= 1;
            rev_diff.push(Diff::Added);
        } else if j == 0 {
            i -= 1;
            rev_diff.push(Diff::Substracted);
        } else if s[i - 1] == t[j - 1] {
            rev_diff.push(Diff::Same);
            i -= 1;
            j -= 1;
        } else if lengths_matrix[(t_len + 1) * i + (j - 1)]
            < lengths_matrix[(t_len + 1) * (i - 1) + j]
        {
            rev_diff.push(Diff::Substracted);
            i -= 1;
        } else {
            rev_diff.push(Diff::Added);
            j -= 1;
        }
    }
    rev_diff
}

pub fn print_diff_iter<I, S>(rev_diff: &[Diff], s: &mut I, t: &mut I)
where
    S: std::fmt::Display,
    I: Iterator<Item = S>,
{
    for elem in rev_diff.iter().rev() {
        match elem {
            Diff::Same => {
                let elem_s = s.next().unwrap();
                t.next().unwrap();
                println!("    {}", elem_s);
            }
            Diff::Added => {
                let elem_t = t.next().unwrap();
                println!("+++ {}", elem_t);
            }
            Diff::Substracted => {
                let elem_s = s.next().unwrap();
                println!("--- {}", elem_s);
            }
        }
    }
}
