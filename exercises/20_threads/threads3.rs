use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // TODO: We want to send `tx` to both threads. But currently, it is moved
    // into the first thread. How could you solve this problem?
    let tx_arc = Arc::new(Mutex::new(tx));

    let tx_thread_1 = tx_arc.clone();
    thread::spawn(move || {
        for val in q.first_half {
            let tx = tx_thread_1.lock().unwrap();
            println!("Sending {val:?}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    let tx_thread_2 = tx_arc.clone();
    thread::spawn(move || {
        let tx = tx_thread_2.lock().unwrap();
        for val in q.second_half {
            println!("Sending {val:?}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
