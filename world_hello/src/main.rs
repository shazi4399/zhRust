fn greet_world() {
    let southern_germany = "FFFF";
    let chinese = "世界，你好";
    let english = "World,hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn test_let() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}

fn main() {
    println!("Hello, world!");
    greet_world();
    test_let();
}
