mod scalar;
mod compound;

fn main() {
    scalar::shadowing_example();
    scalar::mutable_example();
    print!("CONST: {}", scalar::CONST_VALUE);
    scalar::integer_types();
    scalar::number_literal_examples();
    scalar::floatingtype_numbers();
    scalar::numeric_operations();

    compound::array_example();
    compound::tuple_example();
}