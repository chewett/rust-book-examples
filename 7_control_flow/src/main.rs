fn main() {

    let number = 8;
    if number % 5  == 0{
        println!("Number divisible by 5");
    }else if number % 3 == 0 {
        println!("Number divisible by 3");
    }else if number % 2 == 0 {
        println!("Number divisible by 2");
    }else{
        println!("Number not divisible by 5, 3, or 2");
    }

    let new_number = if number % 2 == 0 { number / 2 } else { number + 1};

    println!("The number was {number}");
    println!("The new number is {new_number}");


    loop {
        println!("Without a break this would run forever");
        break;
    }

    let mut counter = 0;
    let value_from_loop = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value returned from the loop is {value_from_loop}");

    let mut times_ran = 0;
    //Label the loops so we can  break from specific ones
    'counting_up: loop {
        let mut inner_loop_ran = 0;
        loop {
            times_ran += 1;
            inner_loop_ran += 1;

            if times_ran > 10 {
                break 'counting_up;
            }

            if inner_loop_ran > 3 {
                break;
            }
        }
    }

    println!("The full loop ran {times_ran} times");

    let mut while_number = 4;
    while while_number > 0 {
        //Do something very important here
        while_number -= 1;
    }

    println!("After finishing the loop we are now on {while_number}");

    let collection_of_ints = [12,15,17,19,25,99];
    for int_from_collection in collection_of_ints {
        println!("One value in the collection is {int_from_collection}");
    }



}
