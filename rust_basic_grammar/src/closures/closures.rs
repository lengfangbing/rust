use rand;
use rand::Rng;
use std::time::Duration;
use std::thread;

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
        println!("{}", suggestion_result(val));
        struct_closures(val);
    }
}

struct Cache<T>
    where T: Fn(f32) -> f32
{
    calculation: T,
    value: Option<f32>,
}

impl<T> Cache<T>
    where T: Fn(f32) -> f32
{
    fn new(calc: T) -> Cache<T> {
        Cache{
            calculation: calc,
            value: None,
        }
    }
    fn value(&mut self, arg: f32) -> f32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn struct_closures(val: f32) {
    let mut result = Cache::new(|num| {
        thread::sleep(Duration::from_secs(2));
        num
    });
    if val < 0.5 {
        println!("calculation result is less than 0.5: {}", result.value(val));
    } else {
        println!("calculation result is more than 0.5: {}", result.value(val));
    }
}