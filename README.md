# Number Complex

## Overview
This crate is really quite simple. The basic principle is to allow for numbers, $z \in \mathbb{C}$.

Anyone who has worked with complex numbers in the past should have no problem working with this crate, 
and for anyone who hasn't, I would recomend [this youtube series](https://www.youtube.com/playlist?list=PLiaHhY2iBX9g6KIvZ_703G3KJXapKkNaF)
by Weich Labs that explains how complex numbers work.


## Example

```rust
let numb1 = Rectangular::new(1., 2.);
let numb2 = Rectangular::new(3., 4.);

// numbers in either polar or rectangular form can be added, subtracted, multiplied
// and divided, just like you would any other number
let res = (numb1 + numb2).get_polar();

// numbers can also converted between rectangular and polar forms using the get_polar() 
// and get_rectangular() methods

println!("{res}");
```


Notice that in the example above, arguments are of type f64.
Also note that the polar form implements Display in such a way that we would in this instance print the number in Euler's forms
```
7.211102550927978*e^0.5880026035475675i
```

For Rectangular:  
$Re(z)$ can be accessed with the real() method  
$Im(z)$ can be accessed with the imag() method

Likewise for Polar:  
$arg(z)$ can be accessed with the arg() method  
$|z|$ can be accessed with the modulus() method
