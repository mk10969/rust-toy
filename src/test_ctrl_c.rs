// ちょっと勉強
#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::{
        cell::UnsafeCell,
        hint::spin_loop,
        ops::{Deref, DerefMut},
        sync::{
            atomic::AtomicBool,
            atomic::{spin_loop_hint, AtomicUsize, Ordering},
            Arc, Mutex, MutexGuard,
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

    // スピンロック（ロックが獲得できない間、無限ループによってロック怪j法を待つ）
    pub struct Mutex<T> {
        // 「ロックされている」OR「ロックされていない」の２状態ある。
        locked: AtomicBool,
        // ミューテックスが保護するデータ。
        inner: UnsafeCell<T>,
    }
    // ロックを保持するRAIIガード
    pub struct MutexGuard<'a, T> {
        locked: &'a AtomicBool,
        inner: &'a mut T,
    }

    unsafe impl<T: Send> Send for Mutex<T> {}
    unsafe impl<T: Sync> Sync for Mutex<T> {}

    impl<T> Mutex<T> {
        fn new(value: T) -> Self {
            Self {
                locked: AtomicBool::new(false),
                inner: UnsafeCell::new(value),
            }
        }

        fn lock(&self) -> MutexGuard<'_, T> {
            loop {
                // 「ロックされているか調べ、されていなかったらロックを取得する」
                // という処理をする。
                // ただし、一気にやらないと競合状態が発生してしまう。
                // Compare-and-swapによって上記を一度に行う。
                // 「false だったら true に変更する、そうでなかったら何もしない」
                let old_locked = self.locked.compare_and_swap(false, true, Ordering::SeqCst);
                // 戻り値は元の値。
                if old_locked != false {
                    // 元の値がtrue → 他の人がロックしていた。
                    // 無限ループによって次の機会を待つ。
                    spin_loop_hint();
                    continue;
                }
                // 元の値がfalse → ロック成功
                break;
            }

            MutexGuard {
                locked: &self.locked,
                inner: unsafe { &mut *self.inner.get() },
            }
        }
    }

    impl<T> Drop for MutexGuard<'_, T> {
        fn drop(&mut self) {
            // アンロック時は競合状態は気にしなくてよいので、単にfalseを保存する。
            self.locked.store(false, Ordering::SeqCst);
        }
    }

    // https://doc.rust-jp.rs/book-ja/ch15-02-deref.html
    // Dereference
    impl<T> Deref for MutexGuard<'_, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            self.inner
        }
    }

    impl<T> DerefMut for MutexGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.inner
        }
    }
}
