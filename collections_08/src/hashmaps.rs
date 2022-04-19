// By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables1.
// This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. 
// If you profile your code and find that the default hash function is too slow for your purposes,
// you can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait.


use std::{collections::{HashMap, btree_map::Values}, process, str::pattern::StrSearcher, primitive};

fn create_hashmap_using_new_example(){

    let mut score = HashMap::new();
    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Yellow"), 50);
}

// team names and initial scores in two separate vectors,
// we could use the zip method to create an iterator of tuples where “Blue” 
// is paired with 10, and so forth.
fn create_hashmap_using_iterators_example(){

    let teams = vec![
        String::from("Blue"),
        String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // For the parameters for the key and value types, however,
    // we use underscores, and Rust can infer the types that
    // the hash map contains based on the types of the data in the vectors
    let mut scores: HashMap<_,_> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}


fn hashmap_ownership_example(){
    let field_name = String::from("Color");
    let field_value = String::from("Blue");

    // If we insert references to values into the hash map, 
    // the values won’t be moved into the hash map.
    // The values that the references point to must be valid for at least as long as the hash map is valid. 
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point,
    // because ownership is moved to HashMap
}

fn accessing_values_example(){
    let mut map = HashMap::new();
    map.insert("Key1", "Value");
    map.insert("Key2", "Value");
    map.insert("Key3", "Value");

    let key = String::from("Key1");
    let value = map.get(&key);
    
    match value {
        Some(Value) => println!("{}", value),
        None => println!("no value present"),
    }
}

fn iterate_map_example(){
    let mut map = create_map();
    for (key, value) in map {
        println!("value {} key {}", key, value);
    }
}

fn check_if_key_has_value() {
    let mut map = create_map();

    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists,
    // and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value. 
    map.entry("Key1").or_insert("other value");

    // print map
    println!("{:?}", map);
}



fn create_map() -> &mut HashMap<String, String>{
    let mut map = HashMap::new();
    map.insert("Key1", "Value1");
    map.insert("Key2", "Value2");
    map.insert("Key3", "Value3");
    map
}