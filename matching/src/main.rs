#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(t) => Some(t+1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(&coin); // Здесь избегается перемещение путем использования ссылки, вместо передачи объекта
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // _ => (), где () - единичное значение, пустой тип кортежа, который означает, что мы ничего не делаем
    }

    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max), // В тако реализации много лишнего, используем другую конструкцию
    //     _ => (),
    // }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin { // Больше мы не можем использовать coin, на этом этапе произошла передача владения
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
