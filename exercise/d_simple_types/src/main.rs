// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

fn main() {
    let coords: (f64, f64) = (6.3, 15.0);
    print_difference(coords.0, coords.1);   // mengakses tupple

    let coords_arr: [f64;2] = [coords.0, coords.1]; // mendefinisikan array
    print_array(coords_arr);
    
    let series: [i32;7] = [1, 1, 2, 3, 5, 8, 13];
    ding(series[6]); // akses array

    // sebisa mungkin anotation nya tidak harus dibuat
    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
  
    on_off(mess.2[1].0); // akses kombinasi array dan tupple

    print_distance(coords);
}


// destructuring parameter
fn print_distance((x,y): (f64, f64)) {
    println!(
        "Distance to the origin is {}",
        (x.powf(2.0) + y.powf(2.0)).sqrt()
    );
}

// Challenge:
//
// Although types can often be inferred by the compiler, sometimes we write them out for clarity.
// Like we did with the `let coords: (f64, f64) = ...` declaration at the top of the `main`
// function.
//
// - Add the type annotation for the `series` variable in `main`.
// - Add the type annotation for the `mess` variable in `main`. (This may be a good example of why
//   it is nice to *not* have to add the type annotation! 😆)

fn print_difference(x: f64, y: f64) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

fn print_array(a: [f64; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}
