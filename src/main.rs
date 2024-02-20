#[macro_use] extern crate prettytable;

use core::fmt;
use std::{collections::HashMap, io::{self, Write}};
use rand::Rng;
use prettytable::Table;

struct Product {
    product_id: String,
    product_name: String,
    product_description: String,
    product_price: String,
    product_quantity: String
}

impl Product {
    fn new(
        product_id: String,
        product_name: String,
        product_description: String,
        product_price: String,
        product_quantity: String
    ) -> Self {
        Product {
            product_id,
            product_name,
            product_description,
            product_price,
            product_quantity
        }
    }

    fn update(
        &mut self,
        product_name: String,
        product_description: String,
        product_price: String,
        product_quantity: String
    ) {
        self.product_name = product_name;
        self.product_description = product_description;
        self.product_price = product_price;
        self.product_quantity = product_quantity;
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ProductID: {}, Product Name: {}, Product Description: {}, Product Price: {}, Product Quantity: {}",
            self.product_id,
            self.product_name,
            self.product_description,
            self.product_price,
            self.product_quantity
        )
    }
}

fn simulate_auth() -> bool {
    let mut user_name = String::new();
    let mut user_pass = String::new();

    print!("Enter your username: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut user_name).expect("Invalid username provided");

    print!("Enter your password: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut user_pass).expect("Please provide a password");

    let parsed_user_name: String = user_name.trim().parse().expect("Invalid username");
    let parsed_user_pass: String = user_pass.trim().parse().expect("Invalid password");

    if parsed_user_name == "inventoryuser" && parsed_user_pass == "mypassword" {
        return true;
    }

    println!("username or password is incorrect");
    io::stdout().flush().unwrap();

    false
}

fn list_product(products_store: &mut HashMap<String, Product>) {
    let mut table = Table::new();

    table.add_row(row!("Product ID", "Product Name", "Product Description", "Product Price", "Product Quantity"));

    for (key, value) in products_store.iter() {
        table.add_row(row![key, value.product_name, value.product_description, value.product_price, value.product_quantity]);
    }

    table.printstd();

    display_options_menu(products_store);
}

fn add_product(products_store: &mut HashMap<String, Product>) {
    if let Some(product) = get_product_info() {
        products_store.insert(product.product_id.clone(), product);

        println!("Product has been saved in the inventory!");
        io::stdout().flush().unwrap();
    }
    else {
        display_options_menu(products_store);
    }
}

fn get_product_id_from_input() -> String {
    let mut product_id = String::new();

    print!("Enter product Id: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut product_id).expect("Please enter product id");

    product_id.trim().to_string()
}

fn edit_product(
    products_store: &mut HashMap<String, Product>
) {
    let product_id = get_product_id_from_input();

    if let Some(product) = products_store.get_mut(&product_id) {
        
        if let Some(updated_product) = get_product_info() {
            product.update(
                updated_product.product_name,
                updated_product.product_description,
                updated_product.product_price,
                updated_product.product_quantity
            );
        }
        else {
            println!("Error occured while updating the product, please try again");
            io::stdout().flush().unwrap();

            display_options_menu(products_store);
        }

    }
    else {
        println!("Product with the given id does not exist");
        io::stdout().flush().unwrap();
        
        display_options_menu(products_store);
    }
}

fn delete_product(
    products_store: &mut HashMap<String, Product>,
) {
    let product_id = get_product_id_from_input();

    if let Some(_) = products_store.get(&product_id) {
        products_store.remove(&product_id);
    }
    else {
        println!("Product with the given id does not exist");
        io::stdout().flush().unwrap();
        
        display_options_menu(products_store);
    }
}

fn search_product(
    products_store: &mut HashMap<String, Product>
) {
    let product_id = get_product_id_from_input();

    if let Some(product) = products_store.get_mut(&product_id) {
        let mut table = Table::new();

        table.add_row(row!("Product ID", "Product Name", "Product Description", "Product Price", "Product Quantity"));
        
        table.add_row(
            row![product_id,
            product.product_name,
            product.product_description,
            product.product_price,
            product.product_quantity
        ]);

        table.printstd();

        display_options_menu(products_store);
    }
    else {
        println!("Product with the given id does not exist");
        io::stdout().flush().unwrap();
        
        display_options_menu(products_store);
    }
}

fn display_options_menu(products_store: &mut HashMap<String, Product>) {
    println!("");
    print!("What would you like to do today:\n\
    1. View all products\n\
    2. Add new product\n\
    3. Edit existing product\n\
    4. Delete product\n\
    5. Search product\n\
    6. Exit program\n\
    >> ");
    io::stdout().flush().unwrap();
    

    let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Please provide a valid choice");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice, please enter from the given numbers");
                return;
            }
        };

        match choice {
            1 => list_product(products_store),
            2 => add_product(products_store),
            3 => edit_product(products_store),
            4 => delete_product(products_store),
            5 => search_product(products_store),
            6 => {
                print!("Exiting the program....");
                std::process::exit(0);
            },
            _ => println!("Invalid choice")
        }
}

fn generate_random_id(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let id: String = (0..length).map(|_| {
        let digit: u8 = rng.gen_range(0..9);
        digit.to_string()
    })
    .collect();

    id
}

fn get_product_info() -> Option<Product> {
    let mut product_name = String::new();
    let mut product_description = String::new();
    let mut product_price = String::new();
    let mut product_quantity = String::new();

    print!("Enter product's name: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut product_name).expect("Please enter product name");
    let parsed_name = product_name.trim().parse().expect("Invalid product name");

    print!("Enter product's description: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut product_description).expect("Please enter product description");
    let parsed_description = product_description.trim().parse().expect("Invalid product description");

    print!("Enter product's price: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut product_price).expect("Please enter product price");
    let parsed_price: f32 = product_price.trim().parse().expect("Invalid product price");

    print!("Enter product's quantity: ");
    io::stdout().flush().unwrap();
    
    io::stdin().read_line(&mut product_quantity).expect("Please enter product quantity");
    let parsed_quantity: i32 = product_quantity.trim().parse().expect("Invalid quantity provided");

    if parsed_quantity < 1 {
        println!("Product quantity must be greater than or equal to 1");
        return None;
    }

    if parsed_price < 1.0 || parsed_price == 0.0 {
        println!("Product price cannot be negative or zero");
        return None;
    }

    let product_id = generate_random_id(10);

    Some(
        Product::new(
            product_id,
            parsed_name,
            parsed_description,
            parsed_price.to_string(),
            parsed_quantity.to_string()
        )
    )
}

fn main() {
    let is_authenticated = simulate_auth();

    if is_authenticated {
        // Simulating a store using HashMap for storing products
        let mut products_store: HashMap<String, Product> = HashMap::new();

        println!("Welcome to the rusty inventory management system");

        loop {
            display_options_menu(&mut products_store);
        }
    }
}