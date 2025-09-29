
pub fn enums_working (){
    enum Days {
        MONDAY,
        TUESDAY,
        WEDNESDAY,
        THURSDAY,
        FRIDAY,
        SATURDAY,
        SUNDAY
    }

    impl Days{
        fn is_weekend (&self) -> bool {
            match self{
                Days::SATURDAY | Days::SUNDAY => return true,
                _ => return false
            };

        }

    }

    let day_check:Days = Days::SATURDAY;

    println!("Today we are going to work on to check whether today is weekend => {}",day_check.is_weekend());

}