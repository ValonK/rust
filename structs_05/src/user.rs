pub struct User {
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub name: String,
    pub last_name: String
}

impl User {

    fn get_full_name(&self) -> String {

        let name = String::new();
        name.push_str(&self.last_name.to_string());        
        name
    }
}