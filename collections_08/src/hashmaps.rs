use std::collections::HashMap;

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

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point,
    // because ownership is moved to HashMap
}