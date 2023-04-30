use number_complex::Rectangular;
fn main() {
let numb1 = Rectangular::new(1., 2.);
let numb2 = Rectangular::new(3., 4.);

// numbers in either polar or rectangular form can be added, subtracted, multiplied
// and divided, just like you would any other number
let res = (numb1 + numb2).get_polar();

// numbers can also converted between rectangular and polar forms using the get_polar() 
// and get_rectangular() methods

println!("{res}");   
}
