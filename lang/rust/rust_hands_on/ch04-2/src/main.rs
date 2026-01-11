use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::sync::mpsc;

fn _main1() {
    println!("===== スレッドの実行と処理の順序 =====");

    thread::spawn(|| {
        println!("Thread:Start!");
        thread::sleep(Duration::from_millis(10));
        println!("Thread:End.");
    });

    println!("Main:Start!");
        thread::sleep(Duration::from_millis(100));
    println!("Main:End.");
}

fn _main2() {
    println!("===== 両スレッドの実行準を確認する =====");

    thread::spawn(|| {
        for n in 1..10 {
            println!("Thread:No, {}.", n);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for n in 1..10 {
        println!("Main:No, {}.", n);
        thread::sleep(Duration::from_millis(100));
    }
}

fn _main3() {
    println!("===== sleepの持つ役割 =====");

    thread::spawn(|| {
        for n in 1..100 {
            println!("Thread:No, {}.", n);
        }
    });

    thread::sleep(Duration::from_millis(1));

    for n in 1..100 {
        println!("Main:No, {}.", n);
    }
}

fn _main4() {
    println!("===== JoinHandleとjoinメソッド =====");

    println!("Main:start!");

    let h = thread::spawn(|| {
        let h1 = thread::spawn(|| {
            for n in 1..10 {
                println!("H1:No, {}.", n);
                thread::sleep(Duration::from_millis(2));
            }
        });

        let h2 = thread::spawn(|| {
            for n in 1..10 {
                println!("H2:No, {}.", n);
                thread::sleep(Duration::from_millis(2));
            }
        });

        for n in 1..10 {
            println!("Thread:No, {}.", n);
            thread::sleep(Duration::from_millis(1));
        }

        let _res1 = h1.join();
        let _res2 = h2.join();
    });

    let _res = h.join();
    println!("Mail:End.");
}

fn _main5() {
    println!("===== スレッドによる値の共有 =====");

    let mut num = 1;
    println!("Main:start!");

    let h1 = thread::spawn(move || {
        println!("H1: start!");
        for n in 1..5 {
            num = 10 * n;
            println!("H1: num_h={}.", num);
            thread::sleep(Duration::from_millis(10));
        }
        println!("H1: End.");
    });

    let h2 = thread::spawn(move || {
        println!("H2: start!");
        for n in 1..5 {
            num += n;
            println!("H2: num_h={}.", num);
            thread::sleep(Duration::from_millis(10));
        }
        println!("H2: End.");
    });

    let _res = h1.join();
    let _res = h2.join();

    println!("Main: End.");
}

fn _main6() {
    println!("===== Arc/Mutexで値を共有する =====");

    let num = Arc::new(Mutex::new(1));

    println!("Main:start!");

    let num_1 = Arc::clone(&num);

    let h1 = thread::spawn(move || {
        let mut num_h1 = num_1.lock().unwrap();
        println!("H1: start!");
        for n in 1..5 {
            *num_h1 += n;
            println!("H1: num_h={}.", *num_h1);
            thread::sleep(Duration::from_millis(10));
        }
        println!("H1: End.");
    });

    let num_2 = Arc::clone(&num);

    let h2 = thread::spawn(move || {
        let mut num_h2 = num_2.lock().unwrap();
        println!("H2: start!");
        for n in 1..5 {
            *num_h2 *= n;
            println!("H2: num_h={}.", *num_h2);
            thread::sleep(Duration::from_millis(10));
        }
        println!("H2: End.");
    });

    let _res = h1.join();
    let _res = h2.join();

    println!("Main: End.");
}


fn _main7() {
    println!("===== 交互にArcを利用する =====");

    let num = Arc::new(Mutex::new(1));

    println!("Main:start!");

    let num_1 = Arc::clone(&num);

    let h1 = thread::spawn(move || {
        println!("H1: start!");
        for n in 1..5 {
            {
                let mut num_h1 = num_1.lock().unwrap();
                *num_h1 += n;
                println!("H1: num_h={}.", *num_h1);
            }
            thread::sleep(Duration::from_millis(10));
        }
        println!("H1: End.");
    });

    let num_2 = Arc::clone(&num);

    let h2 = thread::spawn(move || {
        println!("H2: start!");
        for n in 1..5 {
            {
                let mut num_h2 = num_2.lock().unwrap();
                *num_h2 *= n;
                println!("H2: num_h={}.", *num_h2);
            }
            thread::sleep(Duration::from_millis(10));
        }
        println!("H2: End.");
    });

    let _res = h1.join();
    let _res = h2.join();

    println!("Main: End.");
}

fn _main8() {
    println!("===== スレッドのデッドロック =====");

    let num1 = Arc::new(Mutex::new(0));
    let num2 = Arc::new(Mutex::new(0));

    let value1a = Arc::clone(&num1);
    let value2a = Arc::clone(&num2);

    let value1b = Arc::clone(&num1);
    let value2b = Arc::clone(&num2);

    let h1 = thread::spawn(move || {
        let mut num1 = value1a.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let mut num2 = value2a.lock().unwrap();
        *num1 += 10;
        *num2 += 10;
    });

    let h2 = thread::spawn(move || {
        let mut num2 = value2b.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let mut num1 = value1b.lock().unwrap();
        *num1 += 100;
        *num2 += 100;
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("end");
}

fn _main9() {
    println!("===== チャンネルを利用する =====");

    let (tx, rx) = mpsc::channel();

    println!("Main: start!");

    let h1 = thread::spawn(move || {
        let mut num = 1;
        println!("H1: Start!");
        for n in 1..5 {
            num += n;
            tx.send(num).unwrap();
            println!("H1: num={}.", num);
            thread::sleep(Duration::from_millis(10));
        }
        println!("H1: End.");
    });

    let h2 = thread::spawn(move || {
        let mut num = 1;
        println!("H2: Start!");
        for n in 1..5 {
            let num_recv = rx.recv().unwrap();
            println!("H2: num_recv={}.", num_recv);
            thread::sleep(Duration::from_millis(20));
        }
        println!("H2: End.");
    });

    let _res = h1.join();
    let _res = h2.join();

    println!("Main: End.");
}

fn _main10() {
    println!("===== 相互に送受するには =====");

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    tx2.send(0).unwrap();
    println!("Main: start!");

    let h1 = thread::spawn(move || {
        println!("H1: Start!");
        for _ in 1..5 {
            let val = rx2.recv().unwrap();
            let num = val + 1;
            println!("H1: num={}.", num);
            tx1.send(num).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        println!("H1: End.");
    });

    thread::sleep(Duration::from_millis(5));

    let h2 = thread::spawn(move || {
        println!("H2: Start!");
        for _ in 1..5 {
            let val = rx1.recv().unwrap();
            let num = val * 2;
            println!("H2: num={}.", num);
            tx2.send(num).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        println!("H2: End.");
    });

    let _res = h1.join();
    let _res = h2.join();

    println!("Main: End.");
}

fn main() {
    println!("===== 非同期チャンネルと同期チャンネル =====");

    let (tx1, rx1) = mpsc::sync_channel(1);
    let (tx2, rx2) = mpsc::sync_channel(1);
    tx2.send(0).unwrap();
    println!("Main: start!");

    let h1 = thread::spawn(move || {
        println!("H1: Start!");
        for _ in 1..5 {
            let val = rx2.recv().unwrap();
            let num = val + 1;
            println!("H1: num={}.", num);
            tx1.send(num).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        println!("H1: End.");
    });

    thread::sleep(Duration::from_millis(5));

    let h2 = thread::spawn(move || {
        println!("H2: Start!");
        for _ in 1..5 {
            let val = rx1.recv().unwrap();
            let num = val * 2;
            println!("H2: num={}.", num);
            tx2.send(num).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        println!("H2: End.");
    });

    let _res = h1.join();
    let _res = h2.join();

    println!("Main: End.");
}


