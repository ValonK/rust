

pub fn first_word_without_slices(s: &String) -> usize { // end of word index
    let bytes = s.as_bytes();
    
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

pub fn first_word_with_slices(s: &str) -> &str {
    let string_bytes = s.as_bytes();
    for (i, &item) in string_bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

pub fn other_slices() {

    let x = [1, 2, 3, 4, 5];

    let slice = &x[..2];
    
    let y = [1,2,3];

    assert_eq!(slice, y);
}