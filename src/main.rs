include!("./solution.rs");

fn main() {
    let test1 = vec![5,3,2,4];
    let result = Solution::min_difference(test1);
    println!("Result test 1: {:?}", result);

    let test2 = vec![1,5,0,10,14];
    let result = Solution::min_difference(test2);
    println!("Result test 2: {:?}", result);
}