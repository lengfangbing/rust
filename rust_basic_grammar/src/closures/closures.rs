use rand;
use rand::Rng;

fn get_random_number() -> Result<f32, ()> {
    let mut rng = rand::thread_rng();
    let float = format!("{:.2}", rng.gen::<f32>()).parse::<f32>();
    if let Ok(val) = float {
        Ok(val)
    } else {
        Err(())
    }
}

fn map_number_for_suggestion(num: f32) -> String {
    if num < 0.5 {
        format!("Your num is {}, eat more!", num)
    } else {
        format!("Your num is {}, keep healthy!", num)
    }
}

pub fn test_closures () {
    let float = get_random_number();
    let suggestion_result = |num| {
        if num < 0.5 {
            format!("Your num is {}, eat more!", num)
        } else {
            format!("Your num is {}, keep healthy!", num)
        }
    };
    if let Ok(val) = float {
        println!("{}", suggestion_result(val))
    }
}