const NUM: usize = 200000000;

struct XOR64 {
    x: u64,
}

impl XOR64 {
    fn new(seed: u64) -> XOR64 {
        XOR64 {
            x: seed ^ 88172645463325252,
        }
    }

    fn next(&mut self) -> u64 {
        let x = self.x;
        let x = x ^ (x << 13);
        let x = x ^ (x << 7);
        let x = x ^ (x << 17);
        self.x = x;
        return x;
    }
}

fn main() {
    println!("start single_threaded():");
    println!("-----------------------------");
    single_threaded();
    println!("start multi_threaded():");
    println!("-----------------------------");
    multi_threaded();
}

fn randomized_vec() -> (Vec<u64>, Vec<u64>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    let mut generator = XOR64::new(1234);

    for _ in 0..NUM {
        let x1 = generator.next();
        let x2 = generator.next();
        v1.push(x1);
        v2.push(x2);
    }

    (v1, v2)
}

fn single_threaded() {
    let (mut v1, mut v2) = randomized_vec();

    let start = std::time::Instant::now();

    v1.sort();
    v2.sort();

    let end = start.elapsed();
    println!(
        "single_threaded: {}.{:03}秒",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

fn multi_threaded() {
    let (mut v1, mut v2) = randomized_vec();

    let start = std::time::Instant::now();

    // スレッドを生成
    let handler1 = std::thread::spawn(move || {
        println!("handler1: start");
        v1.sort();
        println!("handler1: ended");
        v1
    });

    let handler2 = std::thread::spawn(move || {
        println!("handler2: start");
        v2.sort();
        println!("handler2: ended");
        v2
    });

    // joinでスレッドの処理を受け取るまでブロックする。

    // 1. スレッド1と2はそれぞれに処理をしていて、値を返す直前まで進む。
    // 2.↓でv1の方をスレッド1から受け取る
    let _v1 = handler1.join().unwrap();
    // 3. ↓でv2の方をスレッド2から受け取る
    let _v2 = handler2.join().unwrap();
    let end = start.elapsed();

    println!(
        "multi_threaded: {}.{:03}秒",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
