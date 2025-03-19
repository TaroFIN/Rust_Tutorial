// Structs

// Structs in Rust are similar to classes in other languages. They are used to create custom data types.
// Structs can have fields and methods.

// use struct_example::Color;
// use struct_example::Color2;

struct Education {
    school: String,
    degree: String,
    major: String,
    year: u16,
}

impl Education {
    fn construct_education(school: String, degree: String, major: String, year: u16) -> Education {
        Education {
            school: school.to_string(),
            degree: degree.to_string(),
            major: major.to_string(),
            year: year,
        }
    }

    fn full_education(&self) -> String {
        format!("{} {} {} {}", self.school, self.degree, self.major, self.year)
    }

    fn change_major(&mut self, major: &str){
        self.major = major.to_string();
    }
}

fn main() {
    // // after defining the struct, we can create an instance of it
    // let mut bg = Color{red: 255, green: 70, blue: 0};
    // println!("Background color: {} {} {}", bg.red, bg.green, bg.blue);

    // // we can also change the values of the fields
    // bg.blue = 255;
    // println!("Background color: {} {} {}", bg.red, bg.green, bg.blue);

    // // we can also create a struct with tuple-like syntax
    // let mut bg2 = Color2(255, 70, 0);
    // println!("Background color2: {} {} {}", bg2.0, bg2.1, bg2.2);

    let mut education1 = Education::construct_education("University of Toronto".to_string(), "BSc".to_string(), "Computer Science".to_string(), 2023);
    println!("Education: {}", education1.full_education());

    //Change major
    education1.change_major("Software Engineering");
    println!("Education: {}", education1.full_education());
}
