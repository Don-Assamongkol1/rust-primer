fn task1() {
    /*
    TASK 1
    *******
    Clippy is a tool that allows you to catch mistakes and improve your code. For this
    exercise, run ``` cargo clippy ``` to see the suggestions and correct the errors.
    */
    let x = 1.2331f64;
    let y = 1.2332f64;
    if y != x {
        // clippy will suggest adding in a margin of error
        println!("Success!");
    }

    let mut res = 42;
    let option = Some(12);
    for x in option {
        // clippy will suggest changing this to 'if let Some(x) = option'
        res += x;
    }
    println!("{}", res);
}
/*
    TASK 2
    *******
    Go back to the exercises in Module 1 and Module 2 and run cargo fmt
    Note if this changes your formatting.


    TASK 3
    *******
    Generics Task:
    Create an animal trait, with characteristics such as type, and weight. Then initialize two different animal structs using the animal trait.

*/
