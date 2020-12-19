// ちょっと勉強
#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::{
        sync::{
            atomic::AtomicBool,
            atomic::{AtomicUsize, Ordering},
            Arc, Mutex,
        },
        thread::spawn,
    };

    #[test]
    fn test_ctrl_c() {
        let running = Arc::new(AtomicBool::new(true));
        // Arcオブジェクトをclone()する。
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
            println!("Ctrl-C..."); // 4
            println!("{:?}", r); // 5
        })
        .expect("Error setting Ctrl-C handler");

        println!("Waiting for Ctrl-C..."); // 1
        while running.load(Ordering::SeqCst) {
            println!("{:?}", running) // 2
        }

        println!("Got it! Exiting..."); // 3
    }

    // イミュータブルなグローバルデータは、迷わず
    lazy_static! {
        static ref COUNTER1: Mutex<usize> = Mutex::new(0);
    }

    fn count_up1() {
        for _ in 0..50000000 {
            let mut counter = COUNTER1.lock().unwrap();
            *counter += 1;
        }
    }
    // ミューテックス
    #[test]
    fn test_mutex() {
        let thread = spawn(count_up1);
        count_up1();
        thread.join().unwrap();
        eprintln!("counter = {}", *COUNTER1.lock().unwrap());
    }

    static COUNTER2: AtomicUsize = AtomicUsize::new(0);

    fn count_up2() {
        for _ in 0..50000000 {
            // 「1を足す」操作をアトミックに行う
            COUNTER2.fetch_add(1, Ordering::SeqCst);
        }
    }

    // アトミック（こっちの方が圧倒的に速い）
    #[test]
    fn test_atomic() {
        // 別スレッドで起動。
        let thread = spawn(count_up2);
        // ここのスレッドで起動。
        count_up2();
        // 終了まで待機
        thread.join().unwrap();
        eprintln!("counter = {}", COUNTER2.load(Ordering::SeqCst));
    }
}
