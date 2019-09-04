struct Item {
    name: String,
    count: u8,
}

fn main() {
    let item_compendium = vec![
        "Blood",
        "Water",
        "Salt",
        "Cloth",
        "Broom",
        "Handle",
        "Fibers",
        "Skeleton",
        "Circle Sigil",
    ];
    let mut inventory: Vec<Item> = Vec::new();

    println!("Welcome to Crafting Sim CLI!");
    println!("Enter \"help\" for help");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("input error");

        let mut words = input.split_whitespace();

        match words.next() {
            Some("help") | Some("h") => {
                println!(
                    "Command list:
                    \n\"give\" will give you a certain amount of items
                    \n\"show\" lets you see your inventory
                    \n\"quit\" terminates the program"
                );
            }
            Some("give") | Some("g") => {
                let mut item_name = String::new();
                let mut item_count = 1;

                let second_word = words.next();
                if let Some(second_word) = second_word {
                    if let Ok(number) = second_word.parse::<u8>() {
                        item_count = number;

                        if let Some(third_word) = words.next() {
                            if let Some(fourth_word) = words.next() {
                                item_name = third_word.to_string() + " " + &fourth_word.to_string();
                            } else {
                                item_name = third_word.to_string();
                            }
                        } else {
                            println!("I don't know what to give you!");
                        }
                    } else {
                        if let Some(third_word) = words.next() {
                            item_name = second_word.to_string() + " " + &third_word.to_string();
                        } else {
                            item_name = second_word.to_string();
                        }
                    }
                } else {
                    println!("I don't know what to give you!");
                }

                let item_name = item_name.to_lowercase();

                let lowercase_compendium = item_compendium
                    .iter()
                    .map(|x| x.to_lowercase())
                    .collect::<Vec<_>>();

                // if the item the user gave us is in the game,
                if lowercase_compendium.contains(&item_name) {
                    println!("It's dangerous to go alone! Take this!");
                    println!("You recieved {} unit(s) of {}", item_count, item_name);

                    let mut does_item_already_exist = false;

                    for item in inventory.iter_mut() {
                        if item.name == item_name {
                            does_item_already_exist = true;
                            item.count += item_count;
                        }
                    }

                    if !does_item_already_exist {
                        inventory.push(Item {
                            name: item_name,
                            count: item_count,
                        });
                    }
                } else {
                    println!("There's no such item!");
                }
            }

            Some("show") | Some("s") => {
                for item in inventory.iter() {
                    println!("Here is a list of items you have:");
                    println!("You have {} unit(s) of {}", item.count, item.name);
                }
            }

            Some("quit") | Some("q") => break,

            _ => {
                println!("Invalid command!");
            }
        }
    }

    println!("Thank you for using CraftSim CLI!");
}
