fn main() {
    enum Days {
        Mon,
        Tue,
        Wed,
        Thu,
        Fri,
        Sat,
        Sun,
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Sat | Days::Sun => true,
                _ => false,
            }
        }
    }

    let today: Days = Days::Mon;
    match today {
        Days::Mon => println!("Everybody hate Mon."),
        Days::Tue => println!("Donut day."),
        Days::Wed => println!("Hump day."),
        Days::Thu => println!("Pay day."),
        Days::Fri => println!("Almost weekend."),
        Days::Sat => println!("Weekend."),
        Days::Sun => println!("Weekend."),
    }

    println!("Is today is weekend {}", today.is_weekend());
}
