mod coffee_maker;mod money;mod menu;
use coffee_maker::*;use money::*;use menu::MENU;
use serde_json::Value;

fn main() {
    let mut is_on = true;
    while is_on {
        println!("Welcome! Your choices are:
            \nespresso \nlatte \ncappuccino
            \nreport \noff");
        let choice = get_input("What would you like?");
        let choice = choice.trim();
        match choice {
            "report" => { print_report(); profit(); },
            "off" => is_on = false,
            _ => {
                let menu: Value = serde_json::from_str(MENU).unwrap();if let Some(drink) = menu.get(choice) {
                let cost = drink["cost"].as_f64().unwrap();
                let ingredients = drink["ingredients"].as_object().unwrap();
                let order_ingredients: Vec<(&str, i64)> = ingredients.iter()
                    .filter(|(k, _)| RESOURCES_VALUE.lock().unwrap().get(k).is_some())
                    .map(|(k, v)| (k.as_str(), v.as_i64().unwrap())).collect();
                if check_resources(choice, &order_ingredients) {
                    println!("Please insert coins.");
                    let payment = pay();
                    if is_enough_money(payment, cost) {
                            make_coffee(choice, &order_ingredients);
                        }
                    } else {
                        println!("That drink is not available.");
                    }
                }
            }
        }
    }
}
