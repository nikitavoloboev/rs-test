use log_macro::log;

// - enums
pub enum CarType {
    Hatch,
    Sedan,
    SUV,
}
pub fn print_size(car: CarType) {
    match car {
        CarType::Hatch => {
            log!("Small sized car");
        }
        CarType::Sedan => {
            log!("Medium sized car");
        }
        CarType::SUV => {
            log!("Large sized Sports Utility car");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run() {
        fn plus_one(x: i32) -> i32 {
            x + 1
        }
        let x = plus_one(8);
        log!(x);
    }
    #[test]
    fn enums() {
        print_size(CarType::SUV);
        print_size(CarType::Hatch);
        print_size(CarType::Sedan);
    }
}
