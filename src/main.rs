fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for window in vec.windows(4) {
        println!("{:?}", window);
    }

    let slice = &vec[2..8];
    println!("{:?}", slice);

    let filtered: Vec<_> = vec.iter().filter(|&&x| x % 2 == 0).collect();
    println!("{:?}", filtered);

    if let Some(third) = vec.get(2) {
        println!("{}", third);
    }

    let reversed: Vec<_> = vec.iter().rev().collect();
    println!("{:?}", reversed);

    let sum: i32 = vec.iter().sum();
    println!("{}", sum);

    let mut slices: Vec<&[i32]> = vec.chunks(3).collect();
    println!("{:?}", slices);

    let first_two = &vec[..2];
    println!("{:?}", first_two);

    let last_three = &vec[7..];
    println!("{:?}", last_three);

    let mut max_number = *vec.iter().max().unwrap();
    println!("{}", max_number);
    let index = vec.iter().position(|&x| x == max_number).unwrap();
    println!("{}", index);
}