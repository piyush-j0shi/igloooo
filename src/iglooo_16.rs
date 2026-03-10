use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Unit {
    Gram,
    Kilogram,
    Milimiter,
    Liter,
    Cup,
    Spoon,
}

#[derive(Debug, PartialEq)]
enum DietaryTag {
    Vegetarian,
    NonVegetarian,
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    quantity: f64,
    unit: Unit,
    dietary_tag: DietaryTag,
}

impl Recipe {
    fn scale(&mut self, target_serving: u32) {
        let scale_factor = target_serving as f64 / self.servings as f64;
        for ingredient in &mut self.ingredient {
            ingredient.quantity *= scale_factor;
        }
        self.servings = target_serving;
    }
}

#[derive(Debug)]
struct Recipe {
    name: String,
    ingredient: Vec<Ingredient>,
    instructions: Vec<String>,
    servings: u32,
    tags: Vec<DietaryTag>,
}

impl Ingredient {
    fn convert_unit(self, new_unit: Unit) -> Ingredient {
        match new_unit {
            Unit::Gram => {
                if self.unit == Unit::Kilogram {
                    Ingredient {
                        quantity: self.quantity * 1000.0,
                        unit: new_unit,
                        ..self
                    }
                } else {
                    self
                }
            }
            Unit::Kilogram => {
                if self.unit == Unit::Gram {
                    Ingredient {
                        quantity: self.quantity / 1000.0,
                        unit: new_unit,
                        ..self
                    }
                } else {
                    self
                }
            }
            Unit::Milimiter => {
                if self.unit == Unit::Liter {
                    Ingredient {
                        quantity: self.quantity * 1000.0,
                        unit: new_unit,
                        ..self
                    }
                } else {
                    self
                }
            }
            Unit::Liter => {
                if self.unit == Unit::Milimiter {
                    Ingredient {
                        quantity: self.quantity / 1000.0,
                        unit: new_unit,
                        ..self
                    }
                } else {
                    self
                }
            }
            _ => self,
        }
    }
}

#[derive(Debug)]
struct Substitution {
    original: String,
    alternatives: Vec<Ingredient>,
}

#[derive(Debug)]
struct RecipeBook {
    recipes: Vec<Recipe>,
    substitutions: Vec<Substitution>,
}

impl RecipeBook {
    fn new() -> Self {
        let recipes: Vec<Recipe> = Vec::new();
        let substitutions: Vec<Substitution> = Vec::new();

        Self {
            recipes: recipes,
            substitutions: substitutions,
        }
    }

    fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    fn remove_recipe(&mut self, name: &str) {
        if let Some(index) = self.recipes.iter().position(|x| x.name == name) {
            self.recipes.remove(index);
        } else {
            println!("recipe did not found for this name");
        }
    }

    fn findrecipe_byname(&self, name: &str) {
        if let Some(recipe) = self.recipes.iter().find(|x| x.name.contains(name)) {
            println!("recipe found : {:#?}", recipe);
        } else {
            println!("recipe did not found");
        }
    }

    fn findby_ingredient(&self, ingredientsearch: &str) {
        let mut ingredientsvec: Vec<&Recipe> = Vec::new();

        for ingredients in &self.recipes {
            for moreingredients in &ingredients.ingredient {
                if moreingredients.name.contains(ingredientsearch) {
                    ingredientsvec.push(ingredients);
                }
            }
        }

        if ingredientsvec.is_empty() {
            println!("no recipes found");
        } else {
            println!("all recipes are : {:#?}", ingredientsvec);
        }
    }

    fn findrecipe_bydietary(&self, tag: DietaryTag) {
        let mut ingredientsvec: Vec<&Recipe> = Vec::new();

        for ingredients in &self.recipes {
            if ingredients.tags.contains(&tag) {
                ingredientsvec.push(ingredients);
            }
        }

        if ingredientsvec.is_empty() {
            println!("nothing found");
        } else {
            println!("all recipes are : {:#?}", ingredientsvec);
        }
    }

    fn add_substitution(&mut self, original: String, alternative: Ingredient) {
        if let Some(sub) = self
            .substitutions
            .iter_mut()
            .find(|s| s.original == original)
        {
            sub.alternatives.push(alternative);
        } else {
            self.substitutions.push(Substitution {
                original,
                alternatives: vec![alternative],
            });
        }
    }

    fn find_substitution(&self, original: &str) -> Option<&Substitution> {
        self.substitutions.iter().find(|s| s.original == original)
    }
}

#[derive(Debug)]
struct MealPlan {
    days: HashMap<String, Vec<Recipe>>,
}

impl MealPlan {
    fn add_recipe_to_day(&mut self, day: String, recipe: Recipe) {
        self.days.entry(day).or_default().push(recipe);
    }

    fn remove_recipe_from_day(&mut self, day: &str, recipe_name: &str) {
        if let Some(recipes) = self.days.get_mut(day) {
            recipes.retain(|r| r.name != recipe_name);
        }
    }

    fn view_plan(&self) {
        for (day, recipes) in &self.days {
            println!("{}:", day);
            for recipe in recipes {
                println!("  - {}", recipe.name);
            }
        }
    }
}

fn main() {
    let mut spaghetti_recipe = Recipe {
        name: String::from("Spaghetti with Tomato Sauce"),
        ingredient: vec![
            Ingredient {
                name: String::from("Spaghetti"),
                quantity: 400.0,
                unit: Unit::Gram,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Olive oil"),
                quantity: 2.0,
                unit: Unit::Spoon,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Garlic (minced)"),
                quantity: 3.0,
                unit: Unit::Spoon,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Canned crushed tomatoes"),
                quantity: 800.0,
                unit: Unit::Gram,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Salt"),
                quantity: 1.0,
                unit: Unit::Spoon,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Sugar"),
                quantity: 1.0,
                unit: Unit::Spoon,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Fresh basil (chopped)"),
                quantity: 10.0,
                unit: Unit::Spoon,
                dietary_tag: DietaryTag::Vegetarian,
            },
            Ingredient {
                name: String::from("Parmesan cheese"),
                quantity: 50.0,
                unit: Unit::Gram,
                dietary_tag: DietaryTag::Vegetarian,
            },
        ],
        instructions: vec![
            String::from("Boil spaghetti in salted water until cooked."),
            String::from("Heat olive oil, sauté garlic until fragrant."),
            String::from("Add crushed tomatoes, salt, and sugar; simmer."),
            String::from("Stir in fresh basil."),
            String::from("Mix spaghetti with sauce."),
            String::from("Top with Parmesan cheese before serving."),
        ],
        servings: 4,
        tags: vec![DietaryTag::Vegetarian],
    };

    let ingredient = Ingredient {
        name: String::from("Spaghetti"),
        quantity: 400.0,
        unit: Unit::Gram,
        dietary_tag: DietaryTag::Vegetarian,
    };

    println!("ingredient before : {:#?}", ingredient);
    let new_ingredient = ingredient.convert_unit(Unit::Kilogram);
    println!("ingredient after : {:#?}", new_ingredient);
    println!("=========================================================================");

    spaghetti_recipe.scale(8);
    println!("update recipe is : {:#?}", spaghetti_recipe);
    println!("==========================================================================");

    let mut recipe_book = RecipeBook::new();

    recipe_book.add_recipe(spaghetti_recipe);
    // recipe_book.add_recipe(spaghetti_recipe1);

    // println!("new updates recipe_book : {:#?}", recipe_book);

    recipe_book.findrecipe_byname("Spaghetti with Tomato Sauce");
    println!("==========================================================================");

    recipe_book.findby_ingredient("Sugar");
    println!("==========================================================================");

    recipe_book.findrecipe_bydietary(DietaryTag::Vegetarian);
    println!("==========================================================================");

    recipe_book.remove_recipe("Spaghetti with Tomato Sauce");
    // println!("==========================================================================");

    // println!("new updated recipe_book : {:#?}", recipe_book);
}
