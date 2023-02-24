use std::io;
use std::thread;
use std::sync::{Arc, Mutex};


const base: u32 = 10;
const modulo:u32 = base.pow(9) + 7;

const maximum:usize = 1000000;

//assert_eq!(&modulo, &reference);


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
fn compute(phases_c: &Arc<Mutex<Vec<secret::Phase>>>){
    let first = secret::Phase::new(1,1);
    let second = secret::Phase::new(2,3);
    let third = secret::Phase::new(3,6);
    let fourth = secret::Phase::new(5,11);


    let mut phases = phases_c.lock().unwrap();
    phases.push(first);
    phases.push(second);
    phases.push(third);
    phases.push(fourth);
    let mut dummy_copies = phases[3].get_copies();
    let mut dummy_so_far = phases[3].get_so_far();
    drop(phases);
    
    //let mut phases: Vec<secret::Phase> = Vec::new();





    

    let mut dummy_counter = 0;
    while dummy_counter < maximum{
        let mut phases = phases_c.lock().unwrap();
        let vec_len = phases.len();
        let second_last = &phases[vec_len-2].get_copies();
        let fourth_last = &phases[vec_len-4].get_copies();

        dummy_copies =  (dummy_copies+second_last+fourth_last).rem_euclid(modulo); 
        dummy_so_far =  (dummy_so_far +
                        dummy_copies).rem_euclid(modulo);
        let phase_dummy = secret::Phase::new(dummy_copies,dummy_so_far);
        phases.push(phase_dummy);
        //println!("COMPUTED: {}, copies: {}",dummy_counter,dummy_copies);
        drop(phases); 
        dummy_counter+=1;
    } 
    // assert_eq!(phases[6].get_so_far(), 64);
    // assert_eq!(phases[4].get_so_far(), 20);
    // assert_eq!(phases[46].get_so_far(), 349633386);
    //helper_print(&phases);

}
fn read_input(phases_r: &Arc<Mutex<Vec<secret::Phase>>>){
    //let mut input_string: String = String::new();


    //io::stdin().read_line(&mut input_string)
    let mut dummy = 0;
    while dummy < maximum
    {
        let phases = phases_r.lock().unwrap();
        if phases.len() > dummy
        {
            //println!("READING: {}, copies: {}",dummy,phases[dummy].get_copies());
            dummy+=1;    
        }
        drop(phases);        
    }
    let phases = phases_r.lock().unwrap();
    assert_eq!(phases[6].get_so_far(), 64);
    assert_eq!(phases[4].get_so_far(), 20);
    assert_eq!(phases[46].get_so_far(), 349633386);
    println!("Asserts DONE");
    drop(phases);  
}


fn sequential()
{

}
fn parallel()
{
    let phases = Arc::new(Mutex::new(Vec::<secret::Phase>::new()));

    let phases_c = Arc::clone(&phases);
    let phases_r = Arc::clone(&phases);
    let computer = thread::spawn(move ||{
        compute(&phases_c)
    });
    let receiver = thread::spawn(move ||{
        read_input(&phases_r)
    }); 
    computer.join().expect("The computer thread has panicked");
    receiver.join().expect("The receiver thread has panicked");
}

fn main(){
    parallel();
}