
 use std::mem;

fn main() {
    println!("Hello, world!");



    // `s` is ASCII which represents each `char` as one byte
    let s = "hello";
    assert_eq!(s.len(), 5);

    // A `char` array with the same contents would be longer because
    // every `char` is four bytes
    let s = ['h', 'e', 'l', 'l', 'o'];
    let size: usize = s.into_iter().map(|c| mem::size_of_val(&c)).sum();
    assert_eq!(size, 20);

    // However, for non-ASCII strings, the difference will be smaller
    // and sometimes they are the same
    let s = "💖💖💖💖💖";

    let mountain: &str = "\u{1F3D4}";
    let car : & str = "🚗";
    assert_eq!(s.len(), 20);

    println!("{:?}",s);
    println!("{:?}",s);
    println!("{:?}",car);
    println!("{:?}",mountain);
    const THREE_HOURS_IN_SECONDS : u32 = 60*60*3;
    println!("{}",THREE_HOURS_IN_SECONDS);
    let mut counter : u32 = 0;
    let x = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}",x);
}
