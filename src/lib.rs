struct Student {
    name: String,
    major: String,
    age: u8,
    grade: f32,
}

impl Student {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_major(&self) -> String {
        self.major.clone()
    }

    pub fn get_age(&self) -> u8 {
        self.age.clone()
    }

    pub fn get_grade(&self) -> f32 {
        self.grade.clone()
    }

    pub fn set_name(&mut self, _name: String){
        println!("This Students's Name changes {} to {}", self.name.clone(), _name.clone());
        self.name = _name;
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut mary = Student{
            name: String::from("Mary".to_string()),
            major: String::from("Business".to_string()),
            age: 27,
            grade: 4.5,
        };

        println!("{}", mary.get_name());
        assert!(mary.get_name() == "Mary", "Name is not correct");

        mary.set_name(String::from("Cherry".to_string()));

        assert!(mary.get_name() == "Cherry", "Name does not change to correct");
    }
}