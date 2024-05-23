use std::{
    collections::HashMap,
    thread,
    time::{Duration, SystemTime},
};
fn main() {
    let duration: Duration = Duration::from_millis(100);
    let exp_time = SystemTime::now() + duration;
    let mut set: HashMap<String, (String, SystemTime)> = HashMap::new();
    set.insert("name".to_string(), ("John".to_string(), exp_time));
    let key = "name";
    thread::sleep(Duration::from_secs(2));

    match set.get(key) {
        Some(value) => {
            if value.1 < SystemTime::now() {
                println!("Expired");
            } else {
                println!("Working");
            }
        }
        None => println!("None"),
    }
    println!("{:?}", set);
}
