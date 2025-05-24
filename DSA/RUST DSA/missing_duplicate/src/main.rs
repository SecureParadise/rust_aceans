// https://leetcode.com/problems/find-missing-and-repeated-values/description/
fn brute_force_duplicate(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut duplicate = vec![];
    let flat: Vec<i32> = matrix.into_iter().flatten().collect();
    for i in 0..flat.len() {
        for j in (i + 1)..flat.len() {
            if flat[i] == flat[j] && !duplicate.contains(&flat[i]) {
                duplicate.push(flat[i]);
            }
        }
    }
    return duplicate;
}

// hashMap optimized one


fn main() {
    let mat = vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]];
    let clone = brute_force_duplicate(mat);
    println!("{:?}", clone);
}
