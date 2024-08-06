use std::io::{Read, stdin};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use Parse_new::Wart_key;
mod lib;
use Parse_new::Wallet;
use std::thread;
use num_cpus;

fn main() {
    let mut choice = String::new();
    println!("Choose one of the options:\n1-Create a random address\n2-Create your desired address.");
    stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();
    match choice {
        "1"=>{
            let address = Wart_key::new();
            println!("Wart address - {}\nWart public key - {}\nWart private key - {}",address.get_address(),address.get_public_key(),address.get_priv_key())
        }
        "2"=>{
            let mut target = String::new();
            println!("Enter address generation criteria (end of address)");
            stdin().read_line(&mut target).unwrap();
            let target = Arc::new(target.trim().to_string());
            let  found =  Arc::new(AtomicBool::new(false));
            let mut vec_threads:Vec<JoinHandle<()>> = Vec::new();
            let address_count = Arc::new(Mutex::new(0));
            for i in 0..num_cpus::get(){
                let count_adr = Arc::clone(&address_count);
                let arc_count = Arc::clone(&target);
                let found = Arc::clone(&found);
                let thread = thread::spawn( move ||
                loop  {
                    if found.load(Ordering::Relaxed){
                        break
                    }
                    let temporary_address = Wart_key::new();
                    match &temporary_address.get_address()[temporary_address.get_address().len() - arc_count.len()..] == *arc_count {
                        false => {
                            let mut lock_count = count_adr.lock().unwrap();
                            *lock_count+=1;
                            if *lock_count%100000 == 0{
                                println!("Generated addresses:{}",*lock_count);

                            }
                        }
                        true => {
                            println!("Wart address - {}\nWart public key - {}\nWart private key - {}", temporary_address.get_address(), temporary_address.get_public_key(), temporary_address.get_priv_key());
                            found.store(true,Ordering::Relaxed);
                            break
                        }

                    }


                });
                vec_threads.push(thread);
            }
            for i in vec_threads.into_iter(){
                i.join().unwrap()
            }

        }
        _=>println!("Wrong choice")
    }

}
