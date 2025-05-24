use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answer_given_by_rabbits: Vec<i32>) -> i32 {
        // count frequency of each answer
        let mut count_map = HashMap::new();
        for &answer_given in &answer_given_by_rabbits {
            *count_map.entry(answer_given).or_insert(0) += 1;
        }
        // calculate total minimum rabbits
        let mut total_rabbits = 0;
        for (&answer, &count) in count_map.iter() {
            let group_size = answer + 1;
            let groups = (count + group_size - 1) / group_size;
            total_rabbits += groups * group_size;
        }
        total_rabbits
    }
}
























fn num_of_rabbits(ans:Vec<i32>) -> i32{
    let mut county = HashMap::new();
    for &jawaf in &ans{
        *county.entry(jawaf).or_insert(0) +1;
    }
    let mut total_rabbit = 0;
    for (&key,&value) in county.iter(){
        let huul_size = key + 1;
        let 
    }
}