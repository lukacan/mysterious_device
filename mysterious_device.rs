use std::io;
use std::thread;
use std::sync::{Arc, Mutex};


const BASE: u32 = 10;
const MODULO:u32 = BASE.pow(9) + 7;

const BASE_: usize = 10;
const MAXIMUM:usize = BASE_.pow(10);

mod secret{
    pub struct Phase{
        copies: u32,
        so_far: u32,
    }
    impl Phase{
        pub fn new(arg1:u32,arg2:u32)->Self
        {
            Self {copies:arg1,so_far:arg2}
        }
        pub fn get_copies(&self)->u32{
            self.copies
        }
        pub fn get_so_far(&self)->u32{
            self.so_far
        }
    }
}
#[allow(dead_code)] 
fn compute_seq(phases: &mut Vec<secret::Phase>, max_x: &usize){
    let first = secret::Phase::new(1,1);
    let second = secret::Phase::new(2,3);
    let third = secret::Phase::new(3,6);
    let fourth = secret::Phase::new(5,11);

    phases.push(first);
    phases.push(second);
    phases.push(third);
    phases.push(fourth);
    let mut dummy_copies = phases[phases.len()-1].get_copies();
    let mut dummy_so_far = phases[phases.len()-1].get_so_far();
    
    let mut dummy_counter = phases.len();
    while dummy_counter < *max_x{
        let vec_len = phases.len();
        let second_last = &phases[vec_len-2].get_copies();
        let fourth_last = &phases[vec_len-4].get_copies();

        dummy_copies =  (dummy_copies+second_last+fourth_last).rem_euclid(MODULO); 
        dummy_so_far =  (dummy_so_far +dummy_copies).rem_euclid(MODULO);
        let phase_dummy = secret::Phase::new(dummy_copies,dummy_so_far);
        phases.push(phase_dummy); 
        dummy_counter+=1;
    }
}
#[allow(dead_code)]
fn compute_par(phases: &mut Vec<secret::Phase>, max_x_c: &Arc<Mutex<usize>>){
    let first = secret::Phase::new(1,1);
    let second = secret::Phase::new(2,3);
    let third = secret::Phase::new(3,6);
    let fourth = secret::Phase::new(5,11);


    phases.push(first);
    phases.push(second);
    phases.push(third);
    phases.push(fourth);
    let mut dummy_copies = phases[phases.len()-1].get_copies();
    let mut dummy_so_far = phases[phases.len()-1].get_so_far();
    
    
    let mut dummy_counter:usize = phases.len();
    let mut over_all_max:usize = MAXIMUM;
    while dummy_counter < over_all_max{
        if dummy_counter % 1000 == 0{
            let max_x = max_x_c.lock().unwrap();
            over_all_max =*max_x as usize; 
            drop(max_x);
        }

        
        let second_last = &phases[dummy_counter-2].get_copies();
        let fourth_last = &phases[dummy_counter-4].get_copies();

        dummy_copies =  (dummy_copies+second_last+fourth_last).rem_euclid(MODULO); 
        dummy_so_far =  (dummy_so_far +dummy_copies).rem_euclid(MODULO);
        let phase_dummy = secret::Phase::new(dummy_copies,dummy_so_far);
        phases.push(phase_dummy);
        dummy_counter+=1;
    } 
}

fn read_num(read_into:&mut usize) ->Result<(),()> {
    let mut input_string: String = String::new();
    match io::stdin().read_line(&mut input_string){
        Ok(_n) =>{
            match input_string.trim().parse::<usize>(){
                Ok(f)=>{
                    *read_into = f;
                    return Ok(())
                }
                _=>{
                    return Err(())
                }
            }   
        }
        _=>{
            Err(())
        }   
    }
}

fn output_solution(x: &Vec<usize>, phases: &Vec::<secret::Phase>){
    for iter in x{
        println!("{}",phases[iter-1].get_so_far());
    }
    // assert_eq!(phases[6].get_so_far(), 64);
    // assert_eq!(phases[4].get_so_far(), 20);
    // assert_eq!(phases[46].get_so_far(), 349633386);
    // assert_eq!(phases[30].get_copies(), 20330163);
    // assert_eq!(phases[21].get_copies(), 128801);
}
fn read_input(x: &mut Vec<usize>, max_x: &mut usize){
    let mut questions: usize = 0;
    let mut dummy_x: usize = 0;
    let mut dummy = 0;

    match read_num(&mut questions) {
        Ok(_) => {
            if questions < 1 || questions > 1000{
                println!("Out of bounds");
                return
            }  
        },
        Err(_) => {
            println!("Wrong input");
            return
        },
    }

    while dummy < questions{
        match read_num(&mut dummy_x) {
            Ok(_) => {
                if dummy_x < 4 || dummy_x > MAXIMUM{
                    println!("Out of bounds");
                    x.clear();
                    *max_x = 0;
                    return
                }else{
                    x.push(dummy_x);
                    dummy+=1;
                    if dummy_x > *max_x{
                        *max_x = dummy_x;
                    }
                }
            },
            Err(_) => {
                println!("Wrong input");
                x.clear();
                *max_x = 0;
                return
            },
        }
    }
    return
}
#[allow(dead_code)]
fn parallel(){
    //! call operations in parallel, so first thread starts
    //! computing recurrence, second thread reads input values,
    //! after all values read, evaluate max , and set global
    //! max, compute fn can then check for max and if it`s
    //! current pos > max, end computing
    //! 
    //! Within testing, we can see that this approach is 
    //! as much efective as sequential approach due to the
    //! fact that read input does not take much time,
    //! so we do not save enough time to speed up 
    //! the process by parallelizing it.
    let mut dummy_max_x = 0;
    let max_x = Arc::new(Mutex::new(MAXIMUM));
    let mut phases = Vec::<secret::Phase>::new();
    let mut x: Vec<usize> = Vec::new();

    let max_x_c = Arc::clone(&max_x);
    let max_x_r = Arc::clone(&max_x);

    
    thread::scope(|scope|{
        let computer = scope.spawn(||{
            compute_par(&mut phases,&max_x_c)
        });

        let receiver = scope.spawn(||{
            read_input(&mut x,&mut dummy_max_x);
            
            let mut max_x = max_x_r.lock().unwrap();
            *max_x = dummy_max_x;
            drop(max_x);
        });

        receiver.join().expect("The receiver thread has panicked");
        computer.join().expect("The computer thread has panicked");
    });
 
    output_solution(&x, &phases);
    
}
#[allow(dead_code)]
fn sequential(){
    //! call operations sequentially, so firstly read input, then
    //! compute values recurrsively until max
    let mut x: Vec<usize> = Vec::new();
    let mut phases = Vec::<secret::Phase>::new();
    let mut max_x:usize = 0;

    read_input(&mut x, &mut max_x);
    compute_seq(&mut phases, &max_x);
    output_solution(&x, &phases);
}

fn main(){
    // call parallel part
    //parallel();
    // call sequential part
    sequential();
}