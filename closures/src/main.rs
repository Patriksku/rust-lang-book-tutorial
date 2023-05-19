// Closures are like functions, except that they are anonymous (they do not have names).
// They can be stored as variables, and passed around.
// They can even be passed in as input parameters to a function.
// They capture the variables inside the scope in which they are defined. 

// Input parameters to closuers are inside of pipes ||, instead of () like in functions.
// For closures that are one-liners, we do not even need the exit {}.

// For closures, we do not need to specify the type of the input parameters, or the return value.
//      The compiler can infer these for us.
//      Also, once the compiler infers the types, the closures then have to be used with those types.
//      So if you make use of a closure that deems both input params and return type to be String,
//      then that is how it is going to be treated forward on.

// Closures are short and only relevant within a narrow context.

use std::{thread, time::Duration, collections::HashMap};

mod capturing_env;

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);

    capturing_env::main();
}

// For closures we need to implement one out of the 3 Fn traits: Fn, FnMut, FnOnce.
//      Note that regular functions also implement one of the 3 Fn traits.
//      So we can store regular functions in 'calculation' as well.
// Here we are defining a trait bound for our generic 'Fn', short for function.
// We add types to the Fn trait to represent the input parameters and the output parameters.
// 
// 'calculation' can be any closure that meets the 'where' trait bound.
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, value: HashMap::new() }
    }

    // New implementation with HashMap for storing many u32's.
    // Next task... Store any type, not just u32???
    // https://stackoverflow.com/questions/58660854/storing-a-generic-closure-in-a-struct
    // https://stackoverflow.com/questions/68168541/how-do-i-create-a-closure-that-takes-either-u32-or-str-and-returns-u32-or-usize
    fn value(&mut self, arg: &u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(*arg);
                self.value.insert(*arg, *arg);
                v
            }

        }
    // Let's get rusty basic tutorial implementation.
        // match self.value {
        //     if self.value.get(arg)
        //     Some(v) => v,
        //     None => {
        //         // Here, we are calling the calculation closure with the parameter arg.
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         v
        //     }
        // }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    // Instead of defining a var like above, we can use a closure like so:
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    // The closure is not storing the return value of our closure... Instead,
    // it is storing the closure itself.
    // We are wrapping the closure inside of our Cacher struct!

    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(&intensity));
        println!("Next, do {} situps!", cached_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(&intensity));

        }
    }

}
