fn greet_world() {
    let southern_germany = "FFFF";
    let chinese = "世界，你好";
    let english = "World,hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn greet(name : String) {
    println!("Hello, {}", name);
}
fn test_let() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
}

fn test_array() {
    let months = [
        "January",
        "February",
        "March",
        "4",
        "5",
    ];
    let index = [3,12,13,14];
    let month = months[index[0]];

    println!("{}", month);
}

fn main() {
    println!("Hello, world!");
    //greet_world();
    //test_let();
    //greet(String::from("zhanghao"));
    test_array();
}
