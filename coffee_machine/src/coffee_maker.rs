use std::sync::Mutex;
use once_cell::sync::Lazy;
use serde_json::Value;
static RESOURCES: &'static str = r#"{
    "water": 300,
    "milk": 200,
    "coffee": 100
}"#;
pub static RESOURCES_VALUE: Lazy<Mutex<Value>> = Lazy::new(|| {
    let resources_str = RESOURCES.replace("i64:", "");  // Remove type annotations from the JSON string
    Mutex::new(serde_json::from_str(&resources_str).expect("Failed to parse resources"))
});
pub fn print_report() {
    let resources = RESOURCES_VALUE.lock().unwrap();
    println!("Water: {}ml", resources.get("water").unwrap().as_i64().unwrap());
    println!("Milk: {}ml", resources.get("milk").unwrap().as_i64().unwrap());
    println!("Coffee: {}g", resources.get("coffee").unwrap().as_i64().unwrap());
}
pub fn check_resources(_drink_name: &str, order_ingredients: &[(&str, i64)]) -> bool {
    let resources = RESOURCES_VALUE.lock().unwrap();
    for item in order_ingredients {
        if let Some(value) = resources.get(item.0) {
            if value.as_i64().unwrap() < item.1 {
                println!("Sorry, there is not enough {}.", item.0);
                return false;
            }
        } else {
            println!("Sorry, we don't have {}.", item.0);
            return false;
        }
    }
    true
}
pub fn make_coffee(drink_name: &str, order_ingredients: &[(&str, i64)]) {
    let mut resources = RESOURCES_VALUE.lock().unwrap();
    for item in order_ingredients {
        if let Some(value) = resources.get_mut(item.0) {
            *value = Value::from(value.as_i64().unwrap() - item.1);
        }
    }
    println!("Here's your {drink_name}. Enjoy! ☕️");
}
