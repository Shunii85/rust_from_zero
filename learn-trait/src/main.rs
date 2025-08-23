use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::prelude::*,
    os::unix::thread,
    path::Path,
    sync,
};

// 虚数
fn main() {
    /////////////////////////////////////////////////////////

    // Displayトレイとの実装 (Debugも、同様に実装することが出来る。#[derive(Debug)]ではなく)
    // struct ImaginaryNumber {
    //     real: f64,
    //     img: f64,
    // }

    // // 虚数を表示するためのDisplayトレイとを実装(適用)する
    // impl Display for ImaginaryNumber {
    //     fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    //         // write!マクロでフォーマッターに文字列を書き込むことで実装できる
    //         write!(f, "{} + j{}", self.real, self.img)
    //     }
    // }

    // let n = ImaginaryNumber {
    //     real: 3.0,
    //     img: 4.0,
    // };
    // println!("{n}");

    /////////////////////////////////////////////////////////

    // イテレータとトレイとの関係

    // // リスト
    // #[derive(Debug, Clone)]
    // enum List<T> {
    //     Node { data: T, next: Box<List<T>> },
    //     Nil,
    // }

    // impl<T> List<T> {
    //     fn new() -> List<T> {
    //         List::Nil
    //     }

    //     // リストを消費して(ムーブセマンティクス)、そのリストの先頭に引数dataを追加したリストを返す
    //     fn cons(self, data: T) -> List<T> {
    //         List::Node {
    //             data,
    //             next: Box::new(self),
    //         }
    //     }

    //     // 不変イテレータを返す
    //     fn iter<'a>(&'a self) -> ListIter<'a, T> {
    //         ListIter { elm: self }
    //     }
    // }

    // struct ListIter<'a, T> {
    //     elm: &'a List<T>,
    // }

    // // Iteratorトレイとは片方向に進むことが可能なイテレータ <= 単方向リスト
    // // 双方向リストの場合、DoubleEndedIteratorトレイトを使う
    // impl<'a, T> Iterator for ListIter<'a, T> {
    //     type Item = &'a T;

    //     fn next(&mut self) -> Option<Self::Item> {
    //         match self.elm {
    //             List::Node { data, next } => {
    //                 self.elm = next;
    //                 Some(data)
    //             }
    //             List::Nil => None,
    //         }
    //     }
    // }

    // {
    //     let list1 = List::new().cons(0).cons(1).cons(2);

    //     for x in list1.iter() {
    //         print!("{x} -> ");
    //     }
    //     println!("None");

    //     println!("----------------------------");

    //     let mut it = list1.iter();
    //     println!("{}", it.next().unwrap());
    //     println!("{}", it.next().unwrap());
    //     println!("{}", it.next().unwrap());
    // }

    //////////////////////////////////////////////

    // シリアライズ

    // #[derive(Debug, Clone, Deserialize, Serialize)]
    // enum List<T> {
    //     Node { data: T, next: Box<List<T>> },
    //     Nil,
    // }

    // impl<T> List<T> {
    //     fn new() -> List<T> {
    //         List::Nil
    //     }

    //     // リストを消費して(ムーブセマンティクス)、そのリストの先頭に引数dataを追加したリストを返す
    //     fn cons(self, data: T) -> List<T> {
    //         List::Node {
    //             data,
    //             next: Box::new(self),
    //         }
    //     }

    //     // 不変イテレータを返す
    //     fn iter<'a>(&'a self) -> ListIter<'a, T> {
    //         ListIter { elm: self }
    //     }
    // }

    // struct ListIter<'a, T> {
    //     elm: &'a List<T>,
    // }

    // // Iteratorトレイとは片方向に進むことが可能なイテレータ <= 単方向リスト
    // // 双方向リストの場合、DoubleEndedIteratorトレイトを使う
    // impl<'a, T> Iterator for ListIter<'a, T> {
    //     type Item = &'a T;

    //     fn next(&mut self) -> Option<Self::Item> {
    //         match self.elm {
    //             List::Node { data, next } => {
    //                 self.elm = next;
    //                 Some(data)
    //             }
    //             List::Nil => None,
    //         }
    //     }
    // }

    // let list2 = List::new().cons(1).cons(2).cons(3);

    // println!("\n------------------- Serialized ------------------\n");
    // // Serialize to JSON
    // let json = serde_json::to_string(&list2).unwrap();
    // println!("JSON: {} bytes", json.len());
    // println!("contents: \n{}\n", json);

    // // Serialize to YAML
    // let yml = serde_yaml::to_string(&list2).unwrap();
    // println!("YAML: {} bytes", yml.len());
    // println!("contents: \n{}\n", yml);

    // // Serialize to MessagePack
    // let msgpack = rmp_serde::to_vec(&list2).unwrap();
    // println!("MessagePack: {} bytes", msgpack.len());
    // println!("contents: \n{:?}\n", msgpack);

    // println!("------------------- Deserialized ------------------\n");

    // // Deserialize from JSON
    // let list2 = serde_json::from_str::<List<i32>>(&json).unwrap();
    // println!("{:?}", list2);

    // // Deserialize from YAML
    // let list2 = serde_yaml::from_str::<List<i32>>(&yml).unwrap();
    // println!("{:?}", list2);

    // // Deserialize from MessagePack
    // let list2 = rmp_serde::from_slice::<List<i32>>(&msgpack).unwrap();
    // println!("{:?}", list2);

    // ファイルへ書き出し (上のコード利用))

    // let list3 = List::new().cons(1).cons(2).cons(3);
    // let yml = serde_yaml::to_string(&list3).unwrap();

    // let path = Path::new("test.yml");
    // let mut f = File::create(path).unwrap();
    // f.write_all(yml.as_bytes()).unwrap();

    // // ファイルからのとれいと
    // let path = Path::new("test.yml");
    // let mut f = File::open(path).unwrap();
    // let mut yml = String::new();
    // f.read_to_string(&mut yml).unwrap();

    // // YAMLからデシリアライズ
    // let list3 = serde_yaml::from_str::<List<i32>>(&yml).unwrap();
    // println!("{:?}", lisせいやく

    /////////////////////////////////////////////////

    //  トレイト制約 (ジェネリクスに制約をつけれるのは、トレイトだけ！！構造体とかは無理だよ)

    // 2乗する関数
    // use std::ops::Mul;

    // fn square<T>(x: T) -> T
    // where
    //     T: Mul<Output = T> + Copy,
    // {
    //     x * x
    // }

    // println!("{}", square(3));

    // 以下のようにも書ける
    // fn square<T: Mul<Output = T> + Copy>(x: T) -> T {
    //     x * x
    // }

    // println!("{}", square(3));

    //// Arc: 複数スレッド間での共有可能, Rc: 不可能
    // Arcで
    // use ::std::{sync::Arc, thread};

    // let n = Arc::new(10);

    // thread::spawn(move || {
    //     println!("{n}");
    // });

    // Rcで <= 複数スレッド間での通信ができないため、エラー
    // Syncトレイト(複数のスレッドからアクセスが可能)を実装していないためというエラーが出る
    // use std::{rc::Rc, thread};

    // let n = Rc::new(10);

    // ↓ コンパイルエラー moveを書いてないから参照の方 => Syncトレイトがない
    // let thr = thread::spawn(|| {
    //     println!("{n}");
    // });

    // RcをArcで包んで密輸してやろう！！
    // エラーだ...
    // use std::{rc::Rc, sync::Arc, thread};

    // let n = Arc::new(Rc::new(10));
    // thread::spawn(move || {
    //     println!("{n}");
    // });

    // さらにmutexで包んで見る！
    // エラー !!
    // use std::{
    //     rc::Rc,
    //     sync::{Arc, Mutex},
    //     thread,
    // };

    // let n = Arc::new(Mutex::new(Rc::new(10)));
    // thread::spawn(move || {
    //     let n = n.lock().unwrap();
    //     println!("{n}");
    // });

    /////////////////////////////////////////////

    // // 動的ディスパッチ

    // trait Foo {
    //     fn foo(&self);
    // }

    // struct Bar;
    // impl Foo for Bar {
    //     fn foo(&self) {
    //         println!("Bar::foo");
    //     }
    // }

    // struct Buzz;
    // impl Foo for Buzz {
    //     fn foo(&self) {
    //         println!("Buzz::foo");
    //     }
    // }

    // // コンパイル時にTが決まる
    // fn call_foo_static<T: Foo>(arg: &T) {
    //     arg.foo();
    // }

    // // 実行時に呼び出し先が決定される
    // // &dyn はファットポインタ
    // fn call_foo_dynamic(arg: &dyn Foo) {
    //     arg.foo();
    // }

    // let bar = Bar;
    // let buzz = Buzz;

    // // 静的ディスパッチ
    // println!("---------------- static ---------------");
    // call_foo_static(&bar);
    // // call_foo_static(&buzz);
    // // 動的ディスパッチ
    // println!("--------------- dynamic --------------");
    // call_foo_dynamic(&bar);
    // call_foo_dynamic(&buzz);

    // ----------------------------------

    // // 動的ディスパッチ利用例

    // // Error: Debug + Display
    // use std::{error::Error, fmt::Display};

    // #[derive(Debug)]
    // struct ErrorA;

    // impl Display for ErrorA {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "Error A")
    //     }
    // }

    // impl Error for ErrorA {}

    // #[derive(Debug)]
    // struct ErrorB;

    // impl Display for ErrorB {
    //     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "Error B")
    //     }
    // }

    // impl Error for ErrorB {}

    // fn error_a() -> Result<(), ErrorA> {
    //     Err(ErrorA)
    // }
    // fn error_b() -> Result<(), ErrorB> {
    //     Err(ErrorB)
    // }

    // // ここで動的ディスパッチを利用することでErrorAもErrorBも同じ関数から返せる
    // // なぜBox ?? => 返り値であるエラーのライフタイムがerror_ab関数内のみのため、それを永続化する必要があるから
    // fn error_ab() -> Result<(), Box<dyn std::error::Error>> {
    //     // ? 演算子 <= Box型に変換できる
    //     error_a()?;
    //     error_b()?;
    //     Ok(())
    // }

    ////////////////////////////////////////////

    //// スーパートレイト

    // trait Location {
    //     fn address(&self) -> &str;
    // }

    // trait Person {
    //     fn name(&self) -> &str;
    // }

    // // Location・Person <= Houseのスーパートレイト
    // trait House: Location + Person {}

    // fn print_house_info(house: &dyn House) {
    //     println!("所有者: {}", house.name());
    //     println!("住所: {}", house.address());
    // }

    // struct MyHouse {
    //     owner: String,
    //     address: String,
    // }

    // impl Location for MyHouse {
    //     fn address(&self) -> &str {
    //         &self.address
    //     }
    // }

    // impl Person for MyHouse {
    //     fn name(&self) -> &str {
    //         &self.owner
    //     }
    // }

    // impl House for MyHouse {}

    // let my_house = MyHouse {
    //     owner: "Shunii".to_string(),
    //     address: "xxxx-aaaaa-111-2".to_string(),
    // };

    // print_house_info(&my_house);

    //////////////////////////////////////////

    //
}
