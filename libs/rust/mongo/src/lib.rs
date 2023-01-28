pub fn mongo() -> String {
    "mongo".into()
}

// pub struct Greeter {
//     greeting: String,
// }

// impl Greeter {
//     pub fn new(greeting: &str) -&gt; Greeter {
//         Greeter { greeting: greeting.to_string(), }
//     }

//     pub fn greet(&self, thing: &str) {
//         println!("{} {}", &self.greeting, thing);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(mongo(), "mongo".to_string());
    }
}
