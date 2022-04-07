mod scalar;
mod compound;
mod functions;

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

    functions::normal();
    functions::with_parm(2);
    functions::with_multiple_params(2, 2, 'A');
    functions::expression();
    functions::function_with_return_value();
    let x = functions::also_function_with_return_value();
}