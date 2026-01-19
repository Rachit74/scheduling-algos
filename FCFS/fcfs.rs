
#[derive(Debug)]
struct Process {
    pid: u32,
    arrival_time: u32,
    burst_time: u32,
    remaining_time: u32,
}

/*
Arrival Time: The time at which process arrives in queue
Completion Time: The time at which process has completed its execution
Brust Time: The required by the process for cpu execution
 */

fn main() {

    let mut p_list: Vec<Process> = vec![];

    let p1: Process = Process {
        pid: 1,
        arrival_time: 4,
        burst_time: 6,
        remaining_time: 6,
    };

    let p2: Process = Process {
        pid: 2,
        arrival_time: 0,
        burst_time: 3,
        remaining_time: 3,
    };

    let p3: Process = Process {
        pid: 3,
        arrival_time: 9,
        burst_time: 2,
        remaining_time: 2,
    };

    let p4: Process = Process {
        pid: 4,
        arrival_time: 5,
        burst_time: 8,
        remaining_time: 8,
    };

    let p5: Process = Process {
        pid: 5,
        arrival_time: 2,
        burst_time: 4,
        remaining_time: 4,
    };

    // Random order insertion
    p_list.push(p3);
    p_list.push(p1);
    p_list.push(p5);
    p_list.push(p2);
    p_list.push(p4);


    // println!("{:?}", p_list);

    fcfs(&mut p_list);


}

fn fcfs(processes: &mut Vec<Process>) {
    processes.sort_by_key(|p| p.arrival_time);

    let mut current_time: u32 = 0;

    for p in processes.iter_mut() {
        if current_time < p.arrival_time {
            current_time = p.arrival_time;
        }
        let start_time = current_time;
        let completion_time = current_time + p.burst_time;
        // 
        current_time = completion_time;
        p.remaining_time = 0;

        println!(
            "PID {} | Start {} | Completion {}",
            p.pid, start_time, completion_time
        );
    }

}
