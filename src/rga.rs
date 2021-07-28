use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::Rng;

//use std::collections::HashMap;

pub fn rga(popsize: usize, searchdimension: usize, kmax : usize, min : f64, max : f64, mutation_probability : usize)-> (Vec<f64>, Vec<f64>){
    
    let mut n: usize = 2;
    let mut d: usize = 1;

    //check the population size, add 1 if odd to perform crossover   
    if popsize > 2 {
        if popsize % 2 == 0 {
            n=popsize;
         } 
         else { n=popsize+1; }                
    } 
    
    //check the search space dimension  
    if searchdimension > 1 {
        d=searchdimension;
    }

    let mut parents = vec![vec![0.0f64; d]; n];
    let mut offspring = vec![vec![0.0f64; d]; 2*n];
    let mut tmp_fitness = vec![0.0f64; 2*n];
    let mut sorted_fitness : Vec<(usize, f64)> = Vec::new();
    let mut best_chart : Vec<f64> = Vec::new();
    let mut best_solution = vec![0.0f64; d]; 
    let mut current_bestfitness : f64 = 0.0;
        
    for i in 0..2*n {
       sorted_fitness.push((i, 0.0f64));
    } 
            
    //Step 1 : Initialize search population
     initilize(&mut parents, min, max);

     //write(&parents, &String::from("Parents initial generation"));
     
     //write(&offspring, &String::from("initial offspring"));
    
    //Do evolution using : crossover, mutation and selection
     let mut k: usize =0;
     
    while k < kmax
    {
        
        println!("generation -> {} : best fitness = {} ",k, current_bestfitness);

        //do crossover 

        do_crossover(&parents, &mut offspring);
        
        //write(&offspring, &String::from("parents + offspring after crossover:"));

        //do mutation
        do_mutation(&mut offspring, min, max, mutation_probability);

        //println!("__ parents after mutation:"); 
        //write(&x);

        //println!("__ offspring after mutation:");
        //write(&offspring); 
        
        //Compute fitness value for each genome
        compute_fitness(&offspring, &mut tmp_fitness);
                     
        //write_f64(&tmp_fitness);

        //sort genomes by fitness 

        sort_genomes(&mut tmp_fitness, &mut sorted_fitness);
       
        //println!("----- sorted genomes");    
        //write_usizef64(&sorted_fitness); 
        
        if k == 0 {
            current_bestfitness = sorted_fitness[0].1;
            //save the best solution 
            for j in 0..d {
            best_solution[j]= offspring[sorted_fitness[0].0][j];
            }
        }
        else {

            if current_bestfitness > sorted_fitness[0].1 {

                //update the current best fitness 
                 current_bestfitness = sorted_fitness[0].1;

                  //save the best solution 
                 for j in 0..d {
                    best_solution[j]= offspring[sorted_fitness[0].0][j];
                    } 
            }
        }

        //save the best chart 
        best_chart.push(current_bestfitness);

        //println!("----- tmp fitness of genomes"); 
        //write_f64(&tmp_fitness);
        
        select_newgeneration(&mut parents, &sorted_fitness, &offspring);
                
        //write(&x, &String::from("THE NEW GENARATION __ parents :"));
                
        k +=1;

    }
    
    //println!("the best solution is ** -- ");
    //write_f64(&best_solution);

    //println!("the best chart is ");
    //write_f64(&best_chart); 
     
    //compute fitness value using the objectif 
    //creating a hashmap (key, alue)
  
    //println!("__ parents :"); 
    //write(&x);

    //println!("__ offspring :");
    //write(&offspring);   

     (best_solution, best_chart)
}

fn initilize(x: &mut Vec<Vec<f64>>, min : f64, max : f64) {

    let intervall_g = Uniform::from(0.0f64..1.0f64);
    let mut rng = rand::thread_rng();
    
    let dist = max-min;

    let mut randgen :f64;  // = rand::thread_rng();
    for i in x.iter_mut() {
        for  j in i.iter_mut() {   
            randgen = intervall_g.sample(&mut rng);
             
            *j = (randgen*dist) + min;  
       }
    }    
}

fn do_crossover(parents : &Vec<Vec<f64>>, offspring : &mut Vec<Vec<f64>>){
    
    let _n = parents.len();
    let _d = parents[0].len();
   
    let interval_d = Uniform::from(1.._d);
    //let interval_crossover = Uniform::from(0..100);

    let mut rng = rand::thread_rng();

    let mut indexes = vec![0usize; _n];
    
    for i in 0.._n {
        indexes[i]=i;
    }
    
    let mut p1 : usize ;
    let mut p2 : usize ;  
    let mut indx1 : usize;
    let mut indx2 : usize; 
    let mut crosspoint : usize;
    //let mut randcorossproba : usize;
    let mut writingindex: usize = _n;

     //copy parents in offspring to keep the best of parents and offspring later
     for i in 0.._n {
         for j in 0.._d {
             offspring[i][j] = parents[i][j]; 
         }
     }

     while indexes.len()>0 {
        
        //randcorossproba = interval_crossover.sample(&mut rng); //rand::thread_rng().gen_range(0..100);
        
        //println!("random crossover probability : {}", randcorossproba);

        //do crossover if generated random value "randcorossproba" is less then "crossoverprobability".  
        //if randcorossproba <= crossoverprobability {

            indx1 = rand::thread_rng().gen_range(0..indexes.len());
            p1=indexes[indx1];
            indexes.remove(indx1);

            indx2 = rand::thread_rng().gen_range(0..indexes.len());
            p2 = indexes[indx2];
            indexes.remove(indx2);

            crosspoint = interval_d.sample(&mut rng);
 
            writingindex = onepoint_crossover(&parents[p1], &parents[p2], crosspoint, offspring, writingindex); 
            
            //writingindex = twopoint_crossover(&parents[p1], &parents[p2], crosspoint, offspring, writingindex); 
            
            //println!("p1 = {}, p2 = {}, crosspt = {}", p1, p2, crosspoint);
        //}
    }
    //write2(&indexes);    
}

fn onepoint_crossover(g1 : &Vec<f64>, g2 : &Vec<f64>, crosspoint : usize, offspring : &mut Vec<Vec<f64>>, pushindex : usize) -> usize {
    
     for i in 0..crosspoint {
         offspring[pushindex][i] = g1[i];
         offspring[pushindex+1][i]= g2[i];
     } 

     for j in crosspoint..g1.len() {
        offspring[pushindex][j] = g2[j];
        offspring[pushindex+1][j] = g1[j];
     } 

      let newpushindex  = pushindex + 2;

      newpushindex
}


//fn twopoint_crossover(g1 : &Vec<f64>, g2 : &Vec<f64>, crosspoint : usize, offspring : &mut Vec<Vec<f64>>, pushindex : usize) -> usize {
    
//    let _d = g1.len();
//    let interval_d = Uniform::from(crosspoint.._d);
//    let mut rng = rand::thread_rng();
//    let  crosspoint2 = interval_d.sample(&mut rng);
//     for i in 0..crosspoint {
//        offspring[pushindex][i] = g1[i];
//        offspring[pushindex+1][i]= g2[i];
//    } 
//    for j in crosspoint..crosspoint2 {
//       offspring[pushindex][j] = g2[j];
//       offspring[pushindex+1][j] = g1[j];
//    } 
//    for i in crosspoint2.._d {
//        offspring[pushindex][i] = g1[i];
//        offspring[pushindex+1][i]= g2[i];
//    } 
//     let newpushindex  = pushindex + 2;
//     newpushindex
//}

fn do_mutation(genomes : &mut Vec<Vec<f64>>, min :f64 , max : f64, mutationprobability : usize) {
    //one point mutation
    //let intervall_n = Uniform::from(0..genomes.len());
    let intervall_d = Uniform::from(0..genomes[0].len());

    let intervall_g = Uniform::from(0.0f64..1.0f64);

    let intervall_mutation = Uniform::from(0..100);

    let mut rng = rand::thread_rng();
    //let mut indx_n : usize;
    let mut indx_d : usize;
    let mut randvalue : f64;
    let mut randmutation : usize;

    let i0 = genomes.len()/2;

    for i in i0..genomes.len() {

        randmutation = intervall_mutation.sample(&mut rng);

        if randmutation <= mutationprobability {
        
        //indx_n = intervall_n.sample(&mut rng);
        
        indx_d = intervall_d.sample(&mut rng);

        randvalue = intervall_g.sample(&mut rng);

        genomes[i][indx_d]= randvalue*(max-min)+min;
        //println!("i= {}, j= {}; rand value = {}",indx_n,indx_d, randvalue);
        }    
    }
}

fn compute_fitness(offspring : &Vec<Vec<f64>>, fitness : &mut Vec<f64>){
   
    let _m = offspring.len();
      
    for j in 0.._m{
        fitness[j]= objective_function(&offspring[j]);
    }  
}

fn sort_genomes(fitness : &mut Vec<f64>, sorted : &mut Vec<(usize, f64)>) {

    let _m : usize =fitness.len();
    let mut max = fitness[0];
    let mut min : f64;
    let mut min_index : usize=0;

    //find the max value
    for i in 0.._m {
        if fitness[i] >= max {
            max = fitness[i];
        }        
    }
    
    for i in 0.._m {
        min = max;

        for j in 0.._m {
             if fitness[j] <= min {
                min = fitness[j];
                min_index= j;
            } 
        }
        sorted[i].0=min_index;
        sorted[i].1=min; 
        fitness[min_index]=max;      
    }
} 

fn select_newgeneration( parents : &mut Vec<Vec<f64>>, sorted_fitness : &Vec<(usize, f64)>, childs : &Vec<Vec<f64>>){
    
    let _n = parents.len();
    let _d = parents[0].len();
    
    for i in 0.._n {
        for j in 0.._d {
             parents[i][j]= childs[sorted_fitness[i].0][j];
        }
    }     
}

fn objective_function(genome : &Vec<f64>)->f64{

//    let mut fitness : f64 = 0.0;

//    for j in gen.iter(){
//        fitness += j.powi(2); //
//     } 

//    fitness
    //f1(genome)
    f0(genome)

}

fn f1(gen : &Vec<f64>)->f64{

    let mut fitness : f64 = 0.0;
 
      for j in gen.iter(){
         fitness += j.powi(2);
      } 
 
     fitness
 }

 fn f0(gen : &Vec<f64>)->f64{

    let mut fitness : f64 = 0.0;
 
      for j in gen.iter(){
         fitness += j.abs();
      } 
 
     fitness
 }


//fn write(x : &Vec<Vec<f64>>, msg : &String)
//{
//  println!("{}", msg);

//  for i in x.iter(){
//      for j in i.iter(){
//         print!("{} ", j);            
//      }
//  println!("_");

//   }
//}

//fn write_usize(y : &Vec<usize>) {
//    for elem in y.iter() {
//        println!("{:?}", elem);
//    }
//}

pub fn write_f64(y : &Vec<f64>) {
    for elem in y.iter() {
        println!("{:?}", elem);
    }
}

//fn write_usizef64(y : &Vec<(usize, f64)>) {
//    for elem in y.iter() {
//        println!("{:?}", elem);
//    }
//}
