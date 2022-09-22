struct CCar{}

fn function_that_can_panic() -> (){}

fn should_continue() -> bool {return false;}

fn memory_example()
{
    let car = Box::new(CCar{});
    let car2 = car;
    //let car3 = car2.clone();
    let my_string = String::from("LGR");
    function_that_can_panic();
    if !should_continue(){return;}
}

fn file_example()
{
    let path = std::path::Path::new("example.txt");
    let file = std::fs::File::open(&path).unwrap();
    function_that_can_panic();
    if !should_continue() {return;}
}