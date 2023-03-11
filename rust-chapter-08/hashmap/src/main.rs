fn main() {
    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
    
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let score = scores.get("Blue");
        let score = scores.get("Blue").copied().unwrap_or(0);

    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
    
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    
        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }

    {
        use std::collections::HashMap;

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
    
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        
        // println!("field_name is {field_name} and field_value is {field_value}") // Invalid
    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
    
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
    
        println!("{:#?}", scores);
    }

    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
    
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
    
        println!("{:#?}", scores);
    }

    {
        use std::collections::HashMap;

        let text = "hello world wonderful world";
    
        let mut map = HashMap::new();
    
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
    
        println!("{:#?}", map);
    }
}
