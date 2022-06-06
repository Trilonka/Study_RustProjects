pub trait Summary {
    fn summarize(&self) -> String;
    // fn summarize(&self) -> String {
    //     String::from("(Read more...)") // Реализация поведения по умолчанию
    // }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// impl Summary for NewsArticle {} // NewsArticle использует реализацию по умолчанию

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// fn notify(item: &impl Summary) { // Это "синтаксический сахар" для формы ниже
//     println!("Breaking new! {}", item.summarize());
// }

fn notify<T: Summary>(item: &T) { 
    println!("Breaking new! {}", item.summarize());
}

// fn notify(item: &(impl Summary + Display)) {} // item должен реализовывать и Summary, и Display

// fn some_function<T, U>(t: &T, u: &U) -> i32 // Если много ограничений, можно записать их через where
//     where T: Display + Clone,
//           U: Clone + Debug
// { }

fn returns_summarizable() -> impl Summary { // возвращает значение, реализующее Summary
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// fn returns_summarizable(switch: bool) -> impl Summary { // Не будет компилироваться, тк нужно вернуть
//     if switch {                                         // либо Tweet, либо NewsArticle
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

fn main() {
    let tweet = Tweet {
        username: String::from("hourse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
}
