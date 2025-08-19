use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList},
    hash::Hash,
};

fn main() {
    // fn a() -> bool {
    //     println!("call a");
    //     true
    // }

    // fn b() -> bool {
    //     println!("call b");
    //     true
    //   }

    // println!("短絡評価");
    // println!("{}", a() || b());

    // println!("非短絡評価");
    // println!("{}", a() & b());

    //////////////////////////////

    // 参照と破壊的代入

    // let mut n: u64 = 100;

    // let a: &u64 = &n;
    // println!("*a = {}, addr = {:p}", a, a);

    // let b: &mut u64 = &mut n;
    // *b = 200;
    // println!("n = {n}");

    /////////////////////////////

    // 配列型とスライス
    // let arr: [u32; 4] = [1, 2, 3, 4];

    // // 0以上2未満
    // let s: &[u32] = &arr[..2];
    // // debug用の出力 {:?}
    // println!("{:?}", s);

    // let s1: &[u32] = &arr[..=2];
    // println!("{:?}", s1);

    /////////////////////////////

    // 文字列と文字列スライス
    // &String => &str 自動的にされるが、その逆 (&str => &String)はできない
    // let a: &str = "  Hello";

    // let mut b: String = a.to_string();
    // b += ", World";
    // println!("{b}");

    // let c: &str = b.trim();
    // println!("{c}");

    //////////////////////////////

    // ユニット型 () <= 0次元のtupple
    // fn unit_type() {}
    // println!("{:?}", unit_type());

    //////////////////////////////

    // 関数ポインタ
    // fn calc_result(f: fn(u32, u32) -> u32, a: u32, b: u32) {
    //     println!("{}", f(a, b));
    // }

    // fn add(a: u32, b: u32) -> u32 {
    //     a + b
    // }
    // fn mul(a: u32, b: u32) -> u32 {
    //     a * b
    // }

    // calc_result(add, 10, 10);
    // calc_result(mul, 10, 10);

    ////////////////////////////////

    // 列挙子
    // enum Storage {
    //     HDD { size: u32, rpm: u32 },
    //     SSD(u32),
    // }

    // struct PCSpec {
    //     cpus: u16,
    //     memory: u32,
    //     storage: Storage,
    // }

    // let spec = PCSpec {
    //     cpus: 8,
    //     memory: 16,
    //     storage: Storage::SSD(1024),
    // };

    // println!("{}", spec.cpus);

    // 自動参照外し
    // let borrowed_cpu: &PCSpec = &spec;
    // println!("{}", borrowed_cpu.memory);

    // struct Square(u32, u32);

    // let sq1 = Square(50, 50);
    // println!("{} * {} = {}", sq1.0, sq1.1, sq1.0 * sq1.1);

    /////////////////////////////////

    // ジェネリクス

    // Box型 => ヒープメモ上にある値への参照,
    // Boxを使うとヒープメモリ上に値を保存できる

    // enum List<T> {
    //     Node { data: T, next: Box<List<T>> },
    //     Nil,
    // }

    // let n1 = List::<u32>::Nil;

    // let n2 = List::<u32>::Node {
    //     data: 10,
    //     next: Box::<List<u32>>::new(n1),
    // };

    // let n3 = List::Node {
    //     data: 40,
    //     next: Box::new(n2),
    // };

    // 連結リスト n3 -> n2 -> n1
    // fn make_pair<T1, T2>(a: T1, b: T2) -> (T1, T2) {
    //     (a, b)
    // }

    // let p1 = make_pair::<u32, bool>(1, true);
    // println!("{:?}", p1);

    // // guess
    // let p2 = make_pair(10, false);
    // println!("{:?}", p2)

    // 定数を受け取るgenerics
    // struct Buffer<const S: usize> {
    //     buf: [u8; S],
    // }

    // let buf = Buffer::<128> { buf: [0; 128] };
    // println!("{:?}", buf.buf);

    /////////////////////////////////////////

    // Option / Result 型

    // struct Status {
    //     code: u32,
    //     message: String,
    // }

    // fn maybe(a: u32) -> Option<Status> {
    //     if a > 50 {
    //         Some(Status {
    //             code: 1,
    //             message: String::from("your request suceeded!!"),
    //         })
    //     } else {
    //         None
    //     }
    // }

    // let res = maybe(100);

    // match res {
    //     Some(r) => println!("code: {}, message: {}", r.code, r.message),
    //     None => println!("failed"),
    // }

    //////////////////////////////////////////

    // if 式

    // let a = 32;
    // ifは式だから値を返せる、評価できる
    // let b = if a > 30 { 100000 } else { 11 };
    // println!("{b}");

    //////////////////////////////////////////

    // ループ系

    // fn sumup_loop(mut n: u64) -> u64 {
    //     let mut total = 0;
    //     loop {
    //         if n == 0 {
    //             break;
    //         }
    //         total += n;
    //         n -= 1;
    //     }
    //     total
    // }

    // println!("loop: total for 1..5 is {}", sumup_loop(5));

    // fn sumup_while(mut n: u64) -> u64 {
    //     let mut total = 0;
    //     while n > 0 {
    //         total += n;
    //         n -= 1;
    //     }
    //     total
    // }

    // println!("while: total for 1..5 is {}", sumup_while(5));

    // fn sumup_for(n: u64) -> u64 {
    //     let mut total = 0;
    //     for i in 0..=n {
    //         total += i;
    //     }
    //     total
    // }

    // println!("for: total for 1..5 is {}", sumup_for(5));

    // continue 命令
    // let v = [3, 8, 5, 7, 12, 15, 22];
    // let mut result = 0;
    // for i in v.iter() {
    //     if *i % 2 == 0 {
    //         continue;
    //     } else {
    //         result += *i;
    //     }
    // }

    // println!("result: {result}")

    ////////////////////////////////////////////////////

    // パターンマッチ
    // fn maybe_fail() -> Option<u32> {
    //     Some(10)
    // }

    // match maybe_fail() {
    //     Some(n) => println!("{n}"),
    //     None => (),
    // }

    // enum Storage {
    //     HDD { size: u32, rpm: u32 },
    //     SSD(u32),
    // }

    // struct PCSpec {
    //     cpus: u16,
    //     memory: u32,
    //     storage: Storage,
    // }

    // let spec = PCSpec {
    //     cpus: 8,
    //     memory: 16,
    //     storage: Storage::SSD(1024),
    // };

    // match &spec {
    //     PCSpec {
    //         storage: Storage::SSD(512),
    //         ..
    //     } => {
    //         println!("512GiB SSD");
    //     }
    //     PCSpec {
    //         cpus: 4 | 8,
    //         memory: m,
    //         storage: _,
    //     } => {
    //         println!("4 or 8 CPUs");
    //         println!("{}GiB memory", *m);
    //     }
    //     PCSpec { memory: m, .. } if *m < 4 => {
    //         println!("the memory is less than 4GiB");
    //     }
    //     _ => (),
    // }

    // if/while let
    // fn maybe_fail() -> Option<u32> {
    //     Some(10)
    // }

    // if let Some(n) = maybe_fail() {
    //     println!("{n}")
    // }

    // while let Some(n) = maybe_fail() {
    //     println!("{n}");
    // }

    // fn average(v: &[f32]) -> Option<f32> {
    //     if v.is_empty() {
    //         return None;
    //     }

    //     let mut total = 0.0;
    //     for n in v.iter() {
    //         total += n;
    //     }

    //     Some(total / v.len() as f32)
    // }

    // let arr: [f32; 5] = [0.0, 4.0, 2.3, 1.5, 3.4];
    // match average(&arr) {
    //     Some(n) => println!("{n}"),
    //     None => (),
    // }

    ///////////////////////////////////////////////

    // マクロ

    // assert!系
    // let n = 3.0; // <= 0.0以上でないとパニックを起こす
    // assert!(n >= 0.0);

    // assert_eq!(3, 3);
    // assert_ne!(3, 2);

    // let a = Some(10);
    // assert_matches!マクロは使えず、assert!とmatches!を併用するみたい
    // assert!(matches!(a, Some(_)));
    // 同じ意味
    // assert_eq!(a.is_some(), true);

    // print系
    // let n = 85;
    // println!("{}", n);
    // // > 右寄せ 左を0埋め
    // println!("{:>04}", n);
    // // n進数表記 #でprefixが付く
    // println!("{:#b}", n);
    // println!("{:b}", n);

    // println!("{:#x}", n);
    // println!("{:x}", n);
    // // 桁数指定16進数 #なら0xも含めて16桁
    // println!("{:016x}", n);
    // println!("{:#016x}", n);

    // ユーザーを義型をデバッグする
    // Storage型は、Debugからの派生(derive)した型であることを意味する
    // #[derive(Debug)]
    // enum Storage {
    //     HDD { size: u32, rpm: u32 },
    //     SSD(u32),
    // }

    // let s = Storage::HDD {
    //     size: 2048,
    //     rpm: 7200,
    // };

    // println!("{:?}", s);
    // println!("{:#?}", s);

    // その他のマクロ
    // let v = vec![true, false, false];

    //　プログラムをパニックさせて停止させるマクロたち
    // panic!("panic! panic! MISAE!!");

    // 後で実装する
    // todo!();

    // 実際するとは限らない
    // unimplemented!();

    // このコードへは到達しない
    // unreachable!();

    ///////////////////////////////////////////

    // クロージャ
    // 特徴: クロージャの外で定義された、要するにスコープの違う変数をキャプチャ、捉えることが出来るところ

    // let f = |a, b| a + b;
    // let n = f(10, 20);
    // println!("{n}");

    // #[derive(Debug)]
    // enum Storage {
    //     HDD { size: u32, rpm: u32 },
    //     SSD(u32),
    // }

    // let mut s = Storage::SSD(512);

    // println!("{:?}", s);

    // println!("--------------------- before/after ----------------------------");
    // let mut f = || match &mut s {
    //     Storage::HDD { size: s, .. } => *s += 64,
    //     _ => (),
    // };

    // f();

    // println!("{:?}", s);

    // move、要するに所有権がクロージャgの環境に移動し、スコープを抜けると削除される。sは使えなくなる
    // let mut g = move || match &mut s {
    //     Storage::HDD { size: s, .. } => *s += 64,
    //     _ => (),
    // };
    // g();

    // sをprintしようと試みると、エラーが出る！！

    //////////////////////////////////////////////

    // メソッドと関連関数

    // #[derive(Debug)]
    // enum Storage {
    //     HDD { size: u32, rpm: u32 },
    //     SSD(u32),
    // }

    // impl Storage {
    //     fn get_size(&self) -> u32 {
    //         match self {
    //             Storage::HDD { size: s, .. } => *s,
    //             Storage::SSD(s) => *s,
    //         }
    //     }

    //     fn set_size(&mut self, size: u32) {
    //         match self {
    //             Storage::HDD { size: s, .. } => *s = size,
    //             Storage::SSD(s) => *s = size,
    //         }
    //     }
    // }

    // let mut ssd1 = Storage::SSD(512);
    // ssd1.set_size(1024);
    // println!("{}GiB's SSD", ssd1.get_size());

    // let mut hdd1 = Storage::HDD {
    //     size: 2048,
    //     rpm: 8400,
    // };
    // hdd1.set_size(4096);
    // println!("{}GiB's HDD", hdd1.get_size());

    // 型関連関数
    // #[derive(Debug)]
    // struct PCSpec {
    //     cpus: u16,
    //     memory: u32,
    //     storage: Storage,
    // }

    // impl PCSpec {
    // 型関連関数, selfが引数にない
    //     fn new(cpus: u16, memory: u32, storage: Storage) -> PCSpec {
    //         PCSpec {
    //             cpus,
    //             memory,
    //             storage,
    //         }
    //     }
    // }

    // let pc1 = PCSpec::new(16, 64, Storage::SSD(1048));
    // println!("{:?}", pc1);

    // 型関連定数
    // struct List<T> {
    //     data: T,
    //     next: Box<List<T>>,
    // }

    // impl<T> List<T> {
    // 型関連定数
    // const MAX: usize = 1024;
    // ...省略
    // }

    // Rustでいう、ある型の実装とは？ => implを用いた、メソッド・型関連関数・型関連定数の定義のこと

    //////////////////////////////////////////////

    // コンパイル時定数 <= コンパイル時に値が確定
    // f64を末尾に付けるとそういう扱いになる Rustでは_アンダーバーを数字につけるのは自由 0_000.001
    // const PI: f64 = 3.14159265358979323846264338327950288_f64;
    // わけわからない区切り方
    // const a: f64 = 0.0_0__0000f64;

    // 静的変数 <= 最初か最後まで生きている
    // static A: u32 = 100;
    // static mut B: u32 = 200;

    ////////////////////////////////////////////////

    // 命名規則
    // - snake case ... 関数名、変数名、フィールド名、モジュール名
    // - CamelCase ... ユーザー定義型の名前、トレイと名、関数外で定義される静的変数(グローバル変数)、関数外で定義されるコンパイル時定数

    ////////////////////////////////////////////////

    // コレクション

    ////////////////////////////////////////////////

    // シーケンス

    // 双方向リスト
    // let mut list1 = LinkedList::new();
    // list1.push_back(0);
    // list1.push_back(10);
    // list1.push_back(20);
    // list1.push_back(20);
    // list1.push_back(20);
    // list1.push_back(20);

    // println!("{:?}", list1);

    // let mut list2 = LinkedList::new();
    // list2.push_back(0);
    // list2.push_back(10);
    // list2.push_back(20);
    // list2.push_back(10);
    // list2.push_back(50);
    // list2.push_back(0);

    // list1.append(&mut list2);

    // list1.push_front(-10);

    // println!("{:?}", list1);

    // bを空にしている
    // println!("{:?}", list2);

    /////////////////////////////////////////

    // マップ系

    // let mut m = BTreeMap::new();
    // m.insert(1, "Angel");
    // m.insert(2, "Demon");
    // m.insert(3, "Elf");
    // m.insert(4, "Dwarf");
    // m.insert(5, "Dragon");
    // m.insert(6, "Orc");

    // println!("{:#?}", m);

    // if let Some(old) = m.remove(&3) {
    //     println!("Removed: {old} in the world.");
    // }
    // println!("");
    // println!("--------------------------");
    // println!("");
    // println!("New World:");
    // println!("{:#?}", m);

    // if let Some(value) = m.get(&2) {
    //     println!("log: {value} appeared !!")
    // }

    // println!("");
    // println!("--------------------------");

    ///////////////////////////////////////////////

    // セット <= 集合を取り扱う

    ///////////////////////////////////////////////

    // イテレータ <= コレクションを順番に処理するための抽象的な型

    // let mut s = BTreeSet::new();
    // s.insert(100);
    // s.insert(400);
    // s.insert(6);
    // s.insert(1);
    // s.insert(250);

    // nにsの各要素へのイテレータが代入される。各要素への参照とほぼ同等と考えていい。
    // for n in s.iter() {
    //     println!("{n}");
    // }

    // _ アンダースコアで推論してくれる
    // let mut a: LinkedList<_> = s.iter().clone().collect();
    // a.push_back(&2);

    // println!("{:?}", a);

    //////////////////////////////////////////////

    // マルチスレッド

    // fn worker() -> u32 {
    //     println!("worker");
    //     100
    // }

    // let handler = std::thread::spawn(worker);

    // match handler.join() {
    //     Ok(n) => println!("{n}"),
    //     Err(e) => println!("{:?}", e),
    // }

    // println!("main thread !!");

    /////////////////////////////////////////////

    // チャネル <= スレッド間で通信するための回線のようなもの
    // let (tx, rx) = std::sync::mpsc::sync_channel(64);

    // let handler = std::thread::spawn(move || match rx.recv() {
    //     Ok((x, y)) => println!("({}, {})", x, y),
    //     Err(e) => eprintln!("{e}"),
    // });

    // if let Err(e) = tx.send((10, 20)) {
    //     eprintln!("{e}");
    // }

    // if let Err(e) = handler.join() {
    //     eprintln!("{:?}", e);
    // }

    //////////////////////////////////////////////

    // 並列にソートする
}
