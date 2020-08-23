use std::cmp::max;

fn main() {
    let s = [1, 3, 2, 4];
    let t = [1, 2, 3, 4];
    let lm = lcsq_lengths(&s, &t);
    let lcsq = backtrack_lcsq(&lm, &s, &t); 
    println!("{:?}", lm);
    println!("LCSQ: {:?}", lcsq);

    let diffs =  diff(&lm, &s, &t);
    println!("Diff: {:?}", diffs);
    print_diff(&diffs, &s, &t);
}


fn lcsq_lengths(s: &[i64], t: &[i64]) -> Vec<usize> {
    let s_len = s.len();
    let t_len = t.len();
    let mut lengths_matrix = vec![0; (s_len + 1) * (t_len + 1)];
    for i in 1..s_len + 1 {
        for j in 1..t_len + 1 {
            if s[i - 1] == t[j - 1] {
                lengths_matrix[(t_len + 1) * i + j] = 1 + lengths_matrix[(t_len + 1) * (i - 1) + (j - 1)];
            } else {
                lengths_matrix[(t_len + 1) * i + j] = max(lengths_matrix[(t_len + 1) * (i - 1) + j],
                                                          lengths_matrix[(t_len + 1) * i + (j - 1)]);
            }
        }
    }
    lengths_matrix
}

fn backtrack_lcsq(lengths_matrix: &[usize], s: &[i64], t: &[i64]) -> Vec<i64> {
    //let mut lcsq: VecDeque<i64> = VecDeque::new();
    let s_len = s.len();
    let t_len = t.len();
    let lcsq_len = lengths_matrix[lengths_matrix.len() - 1];
    let mut lcsq = vec![0; lcsq_len];

    let mut i = s_len;
    let mut j = t_len;
    let mut z = lcsq_len;

    while i > 0 && j > 0  && z > 0 {
        if s[i - 1] == t[j - 1] {
            //lcsq.push_front(s[i - 1]);
            lcsq[z - 1] = s[i - 1];
            z -= 1;
            i -= 1;
            j -= 1;
        } else if lengths_matrix[(t_len + 1) * i  + (j - 1)] > lengths_matrix[(t_len + 1) * (i - 1) + j] {
            j -= 1;
        } else {
            i -= 1;
        }
    }
    lcsq
}

#[derive(Clone, Debug)]
enum Diff {
    Same,
    Added,
    Substracted,
}

fn diff(lengths_matrix: &[usize], s: &[i64], t: &[i64]) -> Vec<Diff>{
    let s_len = s.len();
    let t_len = t.len();
    let mut i = s_len;
    let mut j = t_len;
    let mut diffs: Vec<Diff> = Vec::new();
    while i > 0 || j > 0  {
        if i == 0 {
            j -= 1;
            diffs.push(Diff::Added);
        } else if j == 0 {
            i -= 1;
            diffs.push(Diff::Substracted);
        } else if s[i - 1] == t[j - 1] {
            diffs.push(Diff::Same);
            i -= 1;
            j -= 1;
        } else if lengths_matrix[(t_len + 1) * i  + (j - 1)] < lengths_matrix[(t_len + 1) * (i - 1) + j] {
            diffs.push(Diff::Substracted);
            i -= 1;
        } else {
            diffs.push(Diff::Added);
            j -= 1;
        }
    }
    diffs
}

fn print_diff(diffs: &[Diff], s: &[i64], t: &[i64]) {
    let mut i = 0;
    let mut j = 0;
    for elem in diffs.iter().rev(){
        match elem {
            Diff::Same => {
                println!("    {}", s[i]);
                i += 1;
                j += 1;
            },
            Diff::Added => {
                println!("+++ {}", t[j]);
                j += 1;
            },
            Diff::Substracted => {
                println!("--- {}", s[i]);
                i += 1;
            },
        }
    }
}
