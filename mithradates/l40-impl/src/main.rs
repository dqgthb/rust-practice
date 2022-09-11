#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl Animal {
    fn new_old_cat() -> Self {
        Self {
            age: 15,
            animal_type: AnimalType::Cat,
        }
    }
}

impl Animal {
    fn new_cat(age: u8) -> Self {
        Self {
            age: age,
            animal_type: AnimalType::Cat,
        }
    }

    fn new_dog(age: u8) -> Self {
        Self {
            age,
            animal_type: AnimalType::Dog,
        }
    }

    fn print(&self) {
        println!("I am a: {:?}", self);
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed to dog! Now I am: {:?}", self);
    }
}

impl AnimalType {
    fn check_type(&self) {
        use AnimalType::*;

        match self {
            Cat => println!("Animal type is cat"),
            Dog => println!("Animal type is dog"),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut my_animal = Animal::new_dog(10);
    my_animal.print();
    Animal::print(&my_animal);
    my_animal.change_to_dog();
    Animal::print(&my_animal);

    let mut my_old_animal = Animal::new_old_cat();

    use AnimalType::*;
    my_animal.animal_type.check_type();
}
