use std::fmt::Display;

pub fn pretty_print_matrix<T: Display>(v: &Vec<T>) -> String {
    let n = v.len();
    let root_n = (n as f64).sqrt() as usize;

    let mut temp_s: String;
    let mut s = String::new();

    for i in 0..root_n {
        for j in 0..(root_n - 1) {
            temp_s = format!("{}, ", v[i * root_n + j]);
            s.push_str(&temp_s);
        }

        temp_s = format!("{}\n", v[i * root_n + (root_n - 1)]);
        s.push_str(&temp_s);
    }

    s
}
