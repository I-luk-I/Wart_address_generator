use std::io::{Read, stdin};
use std::ops::Deref;
use std::sync::{Arc};
use std::sync::atomic::{Ordering};
use std::thread::{JoinHandle};
use Parse_new::Wart_key;
mod lib;
use std::thread;
use num_cpus;
use std::sync::atomic::{AtomicU64};
use std::time::Duration;
use::Parse_new::Wallet;

fn main() {
    let mut choice = String::new();
    println!("Choose one of the options:\n1-Create a random address\n2-Create your desired address.");
    stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();
    let global_count = Arc::new(std::sync::atomic::AtomicU64::new(0));
    match choice {
        "1"=>{
            let address = Wart_key::new();
            println!("address - {}\npublic key - {}\nprivate key - {}",address.get_address(),address.get_public_key(),address.get_priv_key())
        }
        "2"=>{
            let mut target = String::new();
            println!("Enter address generation criteria ");
            stdin().read_line(&mut target).unwrap();
            let target = Arc::new(target.trim().to_string());
            let mut vec_threads:Vec<JoinHandle<()>> = Vec::new();
            for i in 0..num_cpus::get() + 2{
                let global_count_clone = Arc::clone(&global_count);
                let target_clone = Arc::clone(&target);
                let thread = thread::spawn( move ||{
                    loop  {
                        let temp_address = Wart_key::new();
                        match &temp_address.get_address()[temp_address.get_address().len() - target_clone.len()..] == *target_clone {
                            false => {
                                global_count_clone.fetch_add(1, Ordering::SeqCst);
                            }
                            true => {
                                println!("address - {}\npublic key - {}\n private key - {}", temp_address.get_address(), temp_address.get_public_key(), temp_address.get_priv_key());
                            }
                        }
                    }});

                vec_threads.push(thread);
            }
            let print_glob = Arc::clone(&global_count);
            let print_count = std::thread::spawn(move ||{
                loop {
                    println!("{}",global_count.load(Ordering::SeqCst));
                    thread::sleep(Duration::from_secs(15))
                }

            });
            vec_threads.push(print_count);
            vec_threads.into_iter().for_each(|i| i.join().unwrap() )
        }
        _=>println!("Wrong choice")
    }

}



