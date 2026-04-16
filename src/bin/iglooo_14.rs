use std::collections::HashMap;

#[derive(Debug)]
struct Category {
    id: String,
    name: String,
    product_ids: Vec<String>,
}

#[derive(Debug)]
struct Product {
    id: String,
    name: String,
    category_id: String,
    supplier_id: String,
    stock: u32,
    price_history: Vec<f64>,
}

#[derive(Debug)]
struct Supplier {
    id: String,
    name: String,
    contact_info: String,
}

#[derive(Debug)]
struct InventorySystem {
    categories: HashMap<String, Category>,
    suppliers: HashMap<String, Supplier>,
    products: HashMap<String, Product>,
}

impl InventorySystem {
    fn new() -> Self {
        let category_map: HashMap<String, Category> = HashMap::new();
        let products_map: HashMap<String, Product> = HashMap::new();
        let suppliers_map: HashMap<String, Supplier> = HashMap::new();

        Self {
            categories: category_map,
            products: products_map,
            suppliers: suppliers_map,
        }
    }

    fn add_category(&mut self, id: &str, name: &str) {
        let new_category = Category {
            id: id.to_string(),
            name: name.to_string(),
            product_ids: vec![],
        };
        self.categories.insert(id.to_string(), new_category);
    }

    fn add_supplier(&mut self, supplier_id: &str, supplier_name: &str, supplier_contact: &str) {
        let supplier_man = Supplier {
            id: supplier_id.to_string(),
            name: supplier_name.to_string(),
            contact_info: supplier_contact.to_string(),
        };
        self.suppliers.insert(supplier_id.to_string(), supplier_man);
    }

    fn add_product(
        &mut self,
        id: &str,
        name: &str,
        category_id: &str,
        supplier_id: &str,
        stock: u32,
        initial_price: f64,
    ) {
        if self.suppliers.contains_key(&supplier_id.to_string()) {
            let new_product = Product {
                id: id.to_string(),
                name: name.to_string(),
                category_id: category_id.to_string(),
                supplier_id: supplier_id.to_string(),
                stock: stock,
                price_history: vec![initial_price],
            };

            self.products.insert(id.to_string(), new_product);

            if let Some(newproduct) = self.categories.get_mut(&category_id.to_string()) {
                // println!("product found : {:?}", newproduct);
                newproduct.product_ids.push(id.to_string());
            } else {
                println!("noithing");
            }
        } else {
            println!("supplier id does not exists");
        }
    }

    fn update_stock(&mut self, product_id: &str, new_stock: u32) {
        if let Some(previous_product) = self.products.get_mut(&product_id.to_string()) {
            // println!("product found : {:#?}", previous_product);

            previous_product.stock = new_stock;

            println!("stock updates");
            println!("updated product : {:#?}", previous_product);
        } else {
            println!("no product found");
        }
    }

    fn update_price(&mut self, product_id: &str, new_price: f64) {
        if let Some(already_product) = self.products.get_mut(&product_id.to_string()) {
            already_product.price_history.push(new_price);

            println!("price updated");
            println!("updated product : {:#?}", already_product);

            println!(
                "latest price is : {:?}",
                already_product.price_history.last()
            );
        }
    }

    fn getlowstock_report(&self, threshold: u32) {
        // let all_products: &Vec<&Product> = &self.products.values().collect();
        // println!("products : {:#?}", all_products);

        for products in self.products.values() {
            if products.stock < threshold {
                println!("product name: {}", products.name);
                println!("product stock : {}", products.stock);

                println!("======================================");
            }
            println!("stock : {:?}", products.stock);
        }
    }

    fn getcategory_report(&self, category_id: &str) {
        if let Some(already_product) = self.categories.get(&category_id.to_string()) {
            // println!("product ids are : {:#?}", already_product.product_ids);

            for ids in &already_product.product_ids {
                // println!("ids are : {}", ids);
                if let Some(with_productid) = self.products.get(ids) {
                    println!("product name : {}", with_productid.name);
                    println!("product stock : {}", with_productid.stock);
                    println!("product price : {:?}", with_productid.price_history);

                    println!("======================================");
                } else {
                    println!("i don't know man");
                }
            }
        } else {
            println!("no product found with this category");
        }
    }

    fn getsupplier_product(&self, supplier_id: &str) {
        // let all_products: &Vec<&Product> = &self.products.values().collect();

        for products in self.products.values() {
            if products.supplier_id == supplier_id.trim().to_string() {
                println!("product name : {}", products.name);
            }
        }
    }
}

fn main() {
    println!(".rs");
    let mut inventorydb = InventorySystem::new();
    inventorydb.add_category("category1", "first item of category1");
    inventorydb.add_category("category2", "second item of category2");
    inventorydb.add_category("category3", "third item of category 3");

    println!("inevntory categories : {:#?}", inventorydb.categories);

    inventorydb.add_supplier("supplier1", "first_supplier", "supplier1@supplier.com");
    inventorydb.add_supplier("supplier2", "second_supplier", "supplier2@supplier.com");
    inventorydb.add_supplier("supplier3", "third_supplier", "supplier3@supplier.com");

    println!("inventory suppliers : {:#?}", inventorydb.suppliers);

    inventorydb.add_product(
        "product1",
        "first product",
        "category1",
        "supplier1",
        32,
        45.0,
    );

    inventorydb.add_product(
        "product2",
        "second product",
        "category1",
        "supplier1",
        56,
        42.0,
    );

    inventorydb.add_product(
        "product3",
        "third product",
        "category2",
        "supplier3",
        60,
        97.0,
    );

    println!("inventory products : {:#?}", inventorydb.products);
    println!("inventory categories : {:#?}", inventorydb.categories);

    inventorydb.update_stock("product1", 32);
    inventorydb.update_price("product1", 64.0);
    inventorydb.getlowstock_report(57);
    inventorydb.getcategory_report("category1");
    inventorydb.getsupplier_product("supplier1");
}
