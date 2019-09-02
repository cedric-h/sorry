struct Item {
    name: String,
    count: u8,
}

fn main() {
    let item_compendium = vec![
        "Blood",
        "Water",
        "Circle_Sigil",
        //"Gym Shorts",
        "Salt",
        "Cloth",
        "Broom",
        "Handle",
        "Fibers",
        //"Assault Rifle",
        "Skeleton",
    ];
    let mut inventory: Vec<Item> = Vec::new();

    println!("Welcome to Crafting Sim CLI!");
    println!("Enter \"Help\" for help.");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("input error");

        let mut words = input.split_whitespace();

        match words.next() {
            Some("give") => {
                let mut item_name = String::new();
                let mut item_count = 1;

                let second_word = words.next();
                if let Some(second_word) = second_word {
                    if let Ok(number) = second_word.parse::<u8>() {
                        item_count = number;

                        if let Some(third_word) = words.next() {
                            item_name = third_word.to_string();
                        } else {
                            println!("I don't know what to give you.");
                        }
                    } else {
                        item_name = second_word.to_string();
                    }
                } else {
                    println!("You need to tell us what to give you");
                }

                let item_name = item_name.to_lowercase();

                let lowercase_compendium = item_compendium
                    .iter()
                    .map(|x| x.to_lowercase())
                    .collect::<Vec<_>>();

                // if the item the user gave us is in the game,
                if lowercase_compendium.contains(&item_name) {
                    println!("{} {}, coming right up!", item_count, item_name);

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

            Some("show") => {
                for item in inventory.iter() {
                    println!("You have {} {}", item.count, item.name);
                }
            }

            Some("q") | Some("quit") => break,

            _ => {
                println!("invalid command!");
            }
        }
    }

    println!("Thank you for using CraftSim CLI!");
}
