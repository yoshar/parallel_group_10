use std::thread;
use std::sync::{Arc,Mutex};

fn main() {
    let num_philosopher: usize = 5; //five philosophers
    let mut handles = vec![]; //mutable vector store all of our threads
    let fork_state = Arc::new(Mutex::new(vec![false; num_philosopher])); //makes a boolean list of size of num_philosopher and all values will be false

    for index in 0..5{
        let fork_state = Arc::clone(&fork_state); //reference to fork_state to use in threads
        let h = thread::spawn(move||{
            let mut fork_state = fork_state.lock().unwrap(); //locking variables so they are used in each thread respectively
            //println!("thread {}", index);
            if fork_state[index] == false{  //if fork is not being used pick it up
                fork_state[index] = true;
                println!("Philosopher {} picked up left fork and is contemplating", index); //the philosopher contemplates when he grabs a fork
            }

            if index != 4{  //this is to make sure we dont get the index out of bounds error
                if fork_state[index+1] == false && index != 4{   //if the fork to the right is available pick it up
                    fork_state[index+1] = true;
                    println!("Philosopher {} picked up right fork", index);
                    if fork_state[index] == true && fork_state[index+1]== true{ //if philosopher has both forks he can eat
                        println!("Philosopher {} is eating since he has acquired both forks", index);
                        println!("Philosopher {} drop forks and is done eating", index); //when he is done eating he doesnt need the forks so he drops them
                        fork_state[index] = false;
                        fork_state[index+1] = false;
                    }
                }
            }

            if fork_state[index-index] == false && index == 4{ //if the fork to the right is available pick it up and since our forks are 0-4 the one right to 4 should be 0
                fork_state[index-index] = true;
                println!("Philosopher {} picked up right fork", index);
                if fork_state[index] == true && fork_state[index-index] == true{ //if philosopher has both forks he can eat
                    println!("Philosopher {} is eating since he has acquired both forks", index);
                    println!("Philosopher {} drop forks and is done eating", index);
                    fork_state[index] = false;
                    fork_state[index-index] = false;
                }
            }
            
        });
        handles.push(h);
    }

    for h in handles.into_iter(){ //join all the threads before main thread to let them run before the main thread
        h.join().unwrap();
    }


    
}
