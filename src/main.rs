use std::env::Args;
use std::io::{Read, stdin};
use std::ops::Deref;
use std::sync::{Arc};
use std::sync::atomic::{Ordering};
use std::thread::{JoinHandle};
use Parse_new::Wart_key;
mod lib;
use std::thread;
use num_cpus;
use std::sync::atomic::{AtomicU64,AtomicBool};
use std::time::{Duration, Instant};
use sha2::digest::consts::True;
use::Parse_new::Wallet;
use clap::Parser;
#[derive(Parser)]
struct Threads{
    #[arg(short,long)]
    num_threads:u8
}

fn main() {
    let threads = Threads::try_parse();
    let mut choice = String::new();
    println!("Choose one of the options:\n1-Create a random address\n2-Create your desired address.");
    stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();
    let global_count = Arc::new(std::sync::atomic::AtomicU64::new(0));
    let  found =  Arc::new(AtomicBool::new(false));
    let mut vec_threads:Vec<JoinHandle<()>> = Vec::new();
    let main_discriptor = std::thread::current();
    let time = std::time::Instant::now();
    match choice {
        "1"=>{
            let address = Wart_key::new();
            println!("Wart address - {}\nWart public key - {}\nWart private key - {}",address.get_address(),address.get_public_key(),address.get_priv_key());
            return
        }
        "2"=>{
            let mut target = String::new();
            println!("Enter address generation criteria (end of address)");
            stdin().read_line(&mut target).unwrap();
            let target = Arc::new(target.trim().to_string());
            let threads = if let Ok(i) = threads {i.num_threads} else { num_cpus::get() as u8 + 2 };
            for i in 0..threads{
                let found_clone = Arc::clone(&found);
                let global_count_clone = Arc::clone(&global_count);
                let target_clone = Arc::clone(&target);
                let main_discriptor = main_discriptor.clone();
                let thread = thread::spawn( move ||{
                    loop  {
                        if found_clone.load(Ordering::Relaxed){
                            break;
                        }
                        let temp_address = Wart_key::new();
                        match &temp_address.get_address()[temp_address.get_address().len() - target_clone.len()..] == *target_clone {
                            false => {
                                let count = global_count_clone.fetch_add(1, Ordering::SeqCst);
                                if count % 1000_000 == 0{
                                    main_discriptor.unpark();
                                }
                            }
                            true => {
                                println!("Wart address - {}\nWart public key - {}\nWart private key - {}", temp_address.get_address(), temp_address.get_public_key(), temp_address.get_priv_key());
                                found_clone.store(true,Ordering::Relaxed);
                                main_discriptor.unpark();
                            }
                        }
                    }});

                vec_threads.push(thread);
            }


        }
        _=>{println!("Wrong choice"); return}
    }
    while !found.load(Ordering::SeqCst) {
        let count = global_count.load(Ordering::Relaxed);
        println!("Addresses generated: {}  speed around {} in min",count, count/time.elapsed().as_secs() * 60);
        std::thread::park();
    }

    vec_threads.into_iter().for_each(|i| i.join().unwrap() )

}



