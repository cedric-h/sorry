type Entity = Vec<Component>;

#[derive(Debug)]
enum Component {
    Name {
        name: String,
    },
    Decomposable {
        components: Vec<Entity>,
    },
    Glowing {
        lumens: u8,
    }
}

fn main() {
    /*let item_compendium = vec![
        "Blood",
        "Water",
        "Salt",
        "Cloth",
        "Broom",
        "Handle",
        "Fibers",
        "Skeleton",
        "Circle Sigil",
    ];*/
    let mut inventory: Vec<Entity> = Vec::new();

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
                let item = vec![
                    Component::Name {
                        name: "Sheep Loin".to_string(),
                    },
                    Component::Decomposable {
                        components: Vec::new(),
                    },
                    Component::Glowing {
                        lumens: 5,
                    }
                ];

                inventory.push(item);

                println!("Congratulations, you get a sheep loin");
            }

            Some("show") | Some("s") => {
                for item in inventory.iter() {
                    let mut full_name = String::new();
                    
                    // set name to their name
                    for comp in item.iter() {
                        if let Component::Name { name } = comp {
                            full_name = name.to_string();
                        }
                    }

                    // add modifiers to name
                    for comp in item.iter() {
                        match comp {
                            Component::Name { .. } => {},
                            _ => {
                                let comp_string = format!("{:?}", comp);
                                let comp_name = comp_string.split_whitespace().next().unwrap();
                                full_name = format!("{} {}", comp_name, full_name).to_string();
                            },
                        }
                    }

                    println!("[0] {}", full_name);
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
