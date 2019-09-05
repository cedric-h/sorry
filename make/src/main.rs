type Entity = Vec<Component>;

enum Component {
    Item {
        name: String,
    },
    Decomposition {
        components: Vec<Entity>,
    },
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
            }

            Some("show") | Some("s") => {
            }

            Some("quit") | Some("q") => break,

            _ => {
                println!("Invalid command!");
            }
        }
    }

    println!("Thank you for using CraftSim CLI!");
}
