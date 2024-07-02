include!("./solution.rs");

fn main() {
    let test1_1 = vec![1,2,2,1];
    let test1_2 = vec![2, 2];
    let result = Solution::intersect(test1_1, test1_2);
    println!("Result test 1: {:?}", result);

    let test2_1 = vec![4,9,5];
    let test2_2 = vec![9,4,9,8,4];
    let result = Solution::intersect(test2_1, test2_2);
    println!("Result test 2: {:?}", result);
}