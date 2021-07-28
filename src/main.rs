include!("rga.rs");

fn main() {
    println!("Real-coded Genetic Algorithm (RGA) in Rust.");

    let n : usize =20;
    let d : usize =7;
    let kmax : usize =500;

    let min : f64 = -100.0; 
    let max : f64 = 100.0;

     //let crossproba : usize = 70;
    let mutationproba : usize = 50;

    let(bestsolution, bestchart) = rga(n, d, kmax, min,max, mutationproba);
   
     println!("The best chart (last):");

     println!("{:?}", bestchart.last());

     //write_f64(&bestchart);

     println!("The best solution :");

     write_f64(&bestsolution);

}
