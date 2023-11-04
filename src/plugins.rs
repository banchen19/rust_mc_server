pub struct Plugin {
   pub name: String,
   pub version: (u8, u8, u8),
   pub authors: String,
   pub repository: String,
   pub keywords: Vec<String>,
}

impl Plugin {
    pub fn new(
        name: String,
        version: (u8, u8, u8),
        authors: String,
        repository: String,
        keywords: Vec<String>,
    ) -> Plugin {
        Plugin {
            name,
            version,
            authors,
            repository,
            keywords,
        }
    }    

    pub fn display_info(&self)->String {
        format!("Plugin Name: {}", self.name)+
        &format!("Version: {}.{}.{}",self.version.0, self.version.1, self.version.2)+
        &format!("Authors: {}", self.authors)+
        &format!("Repository: {}", self.repository)+
        &format!("Keywords: {:?}", self.keywords)
    }
    
}
