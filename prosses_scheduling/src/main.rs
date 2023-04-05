use std::sync::mpsc;
use std::thread;
use std::collections::{VecDeque, HashMap};
use std::io::{stdin};
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, SystemTime};
use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};


#[derive(Clone)]
enum ProcessState {
    Done,
    Working,
    Waiting,
}

#[derive(Clone)]
struct Process {
    name: String,
    total_time: usize,
    remain_time: usize,
    state: ProcessState,
    entry_time: usize,
}

impl Process {
    fn new(name: &str, total_time: usize, entry_time: usize) -> Process {
        return Process {
            name: name.to_string(),
            total_time,
            remain_time: total_time,
            state: ProcessState::Waiting,
            entry_time,
        };
    }


    fn done(&self) -> bool {
        self.remain_time == 0
    }
}


fn get_process_from_user(line: String) -> Process {
    let name: String;
    let total_time: usize;
    let entry_time: usize;
    let v: Vec<String>;
    v = line.trim().split(' ').map(|x| x.to_string()).collect();
    name = v[0].clone();
    total_time = v[1].parse().unwrap();
    entry_time = v[2].parse().unwrap();

    Process::new(&name, total_time, entry_time)
}

fn diff_time(x: &SystemTime, y: &SystemTime) -> u128 {
    y.duration_since(*x).unwrap().as_millis()
}


fn time_need_from(rr_or_fcfs: u8, remain_time: usize, btime: usize) -> usize {
    match rr_or_fcfs {
        // rr
        0 => {
            match remain_time.cmp(&btime) {
                Ordering::Less => { remain_time }
                Ordering::Equal => { remain_time }
                Ordering::Greater => { btime }
            }
        }
        // fcfs
        1 => {
            remain_time
        }
        _ => { 0usize }
    }
}

fn process_handler(rr_or_fcfs: u8, btime_mil: usize, rx: mpsc::Receiver<Process>, r_req_for_average: Receiver<()>, t_ans: Sender<(f64, f64)>) {
    let start_time = SystemTime::now();
    let mut que: VecDeque<Process> = VecDeque::new();
    let mut proc_infos = HashMap::new();
    let btime_mil= (btime_mil as f64/ QTIME as f64) as usize;
    loop {
        let proc = rx.try_recv();
        let finish = r_req_for_average.try_recv();
        if let Result::Ok(_) = finish {
            if que.is_empty() {
                break;
            }
        }
        if let Result::Ok(proc) = proc {
            // real time of (entry, total, done, lunched)
            proc_infos.insert(
                proc.name.clone(),
                (
                    (proc.entry_time as u64 * QTIME) as u128, // entry
                    (proc.total_time as u64 * QTIME) as u128,         // total
                    0u128,                                             // done
                    diff_time(&start_time, &SystemTime::now())
                ),
            );
            que.push_back(proc);
        }

        // Do process work
        let mut proc = match que.pop_front() {
            None => {
                // there is no process.
                continue;
            }
            Some(proc) => { proc }
        };

        proc.state = ProcessState::Working;
        println!("Process {} launched", proc.name);
        let time_need = time_need_from(rr_or_fcfs, proc.remain_time, btime_mil);

        thread::sleep(Duration::from_millis(QTIME * (time_need as u64)));
        proc.remain_time -= time_need;
        if proc.done() {
            proc.state = ProcessState::Done;
            let mut a = proc_infos.get(&*proc.name).unwrap().clone();
            a.2 = diff_time(&start_time, &SystemTime::now());
            proc_infos.insert(proc.name.clone(), (a.0, a.1, a.2, a.3));
            // println!("Process with name {} done its work", proc.name);
            continue;
        }
        proc.state = ProcessState::Waiting;
        que.push_back(proc);
    }
    // calculate total time
    let mut waiting_time = 0u128;
    let mut turnaround = 0u128;
    for (k, proc) in proc_infos.iter() {
        // real time of (entry, total, done, lunched)
        waiting_time += proc.3 - proc.0;
        turnaround += proc.2 - proc.0;
        println!("{} wait {} milli sec and turnaround {}", k,proc.3 - proc.0,proc.2 - proc.0);
    }
    let tur = turnaround as f64 / proc_infos.len() as f64;
    let wait = waiting_time as f64 / proc_infos.len() as f64;
    t_ans.send((wait, tur)).unwrap();
}


fn getting_proc_thread(sender_to_proc_handler: Sender<Process>, receiver_proc_from_main: Receiver<Process>) {
    for res in receiver_proc_from_main {
        sender_to_proc_handler.send(res).unwrap();
    }
}


const QTIME: u64 = (4e2) as u64;
const BTIME: usize = (QTIME) as usize;

fn main() {
    let (tx_to_proc_thread, rx_from_main) = mpsc::channel();
    let (tx_to_proc_handler, rx_from_proc_thread) = mpsc::channel();
    let (t_rec_for_average, r_req_for_average) = mpsc::channel();
    let (t_ans, r_ans) = mpsc::channel();
    const P1:&str = "This is a simulation of two process management algorithms:\n\
                    1) Round Robin\n\
                    2) First-Come First-Serve\n\
                    This program try to simulate real scheduling so, theres some script to do this\n\
                    for example when you all process and enter exit program take all them and try to\n\
                    give them to scheduler at their (Entry time * Quantum time) milli second after\n\
                    scheduler starts its work.\n\
                    So its not exact as we calculate on paper mathematically :))\n";
    println!("{}\n", P1);
    println!("An other things is you can change Quantum time and Btime by chang code\nnow the default is {} and {} respectively.\n", QTIME,BTIME);
    println!("Any way you can chose what algorithm prefer to use\n\
              1 for RR and 2 is for FCFS\n\
              Witch one do you want to use? Enter number: ");


    let mut rr_or_fcfs = String::new();
    std::io::stdin().read_line(&mut rr_or_fcfs).expect("invalid enter number");
    let rr_or_fcfs: u8 = match rr_or_fcfs.trim().parse::<u8>(){
        Ok(number) => {
            if number>2 || number<1 {
                println!("invalid number I'll chose RR algorithm!");
                0
            } else {
                number-1
            }
        },
        Err(_) => {
            println!("invalid number I'll chose RR algorithm!");
            0
        }
    };

    println!("Now enter all processes as template: name total_time entry_time then exit at end");
    let mut procs: Vec<Process> = Vec::new();
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(_) => {}
            Err(_) => {
                println!("Did not enter correct string! Try again.");
                continue;
            }
        };
        if "exit".to_string().eq(&(line.trim())) {
            break;
        }
        procs.push(get_process_from_user(line));
    }
    if procs.is_empty() {
        println!("No process to do");
        return;
    }
    procs.sort_by(|x: &Process, y: &Process| {
        match x.entry_time.cmp(&y.entry_time) {
            Ordering::Less => { Less }
            Ordering::Equal => { Equal }
            Ordering::Greater => { Greater }
        }
    });

    if procs.is_empty() {
        println!("No Processes to work with.");
        return;
    }
    let mut this_time = 0usize;
    let mut total_time = procs[0].total_time + procs[0].entry_time;
    for i in 1..procs.len() {
        if total_time > procs[i].entry_time {
            total_time += procs[i].total_time;
        } else {
            total_time = procs[i].entry_time + procs[i].total_time;
        }
    }

    total_time += 1;


    println!("Processes sorted by entry time:");
    for p in procs.iter() {
        println!("{} {} {}", p.name, p.entry_time, p.total_time);
    }


    thread::spawn(
        move || getting_proc_thread(tx_to_proc_handler, rx_from_main)
    );
    thread::spawn(
        move || process_handler(rr_or_fcfs, BTIME, rx_from_proc_thread, r_req_for_average, t_ans)
    );


    for proc in procs.iter() {
        while proc.entry_time > this_time {
            println!("Time is {}", this_time);
            thread::sleep(Duration::from_millis(QTIME - 5));
            this_time += 1;
        }
        tx_to_proc_thread.send(proc.clone()).unwrap();
    }

    loop {
        if total_time < this_time {
            break;
        }

        println!("Time is {}", this_time);
        thread::sleep(Duration::from_millis(QTIME - 5));
        this_time += 1;
    }
    t_rec_for_average.send(()).unwrap();
    let ans = r_ans.recv().unwrap();

    println!("Base on Quantum_time, Average waiting time: {}, Average turnaround time: {}", ans.0 / QTIME as f64, ans.1 / QTIME as f64);
    println!("All process hase been ended.");
}