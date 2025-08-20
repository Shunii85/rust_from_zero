use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    thread::sleep,
    time::Duration,
};

// Arc・Rc <= スマートポインタ (参照カウンタを持つ賢いポインタだから) と呼ばれる
fn main() {
    let gallery = Arc::new(RwLock::new(initialized_gallery()));

    let mut hdls = Vec::new();
    for n in 0..3 {
        // 客を表すスレッドを作成
        let gallery = gallery.clone();
        let hdl = std::thread::spawn(move || {
            for _ in 0..8 {
                {
                    let guard = gallery.read().unwrap();
                    if n == 0 {
                        for (key, value) in guard.iter() {
                            print!("{key}: {value}, ")
                        }
                        println!();
                    }
                }
                sleep(Duration::from_secs(1));
            }
        });

        // 見る客は1人だけ？(n == 0だけ？)
        hdls.push(hdl);
    }

    let staff = std::thread::spawn(move || {
        for n in 0..4 {
            // 展示内容の入れ替え
            if n % 2 == 0 {
                let mut guard = gallery.write().unwrap();
                guard.clear();
                guard.insert("ゴッホ", "星月夜");
                guard.insert("エッシャー", "滝");
            } else {
                let mut guard = gallery.write().unwrap();
                guard.clear();
                guard.insert("葛飾北斎", "富岳三十六景");
                guard.insert("ミュシャ", "黄道二宮");
            }
            sleep(Duration::from_secs(3));
        }
    });

    for hdl in hdls {
        hdl.join().unwrap();
    }
    staff.join().unwrap();
}

fn initialized_gallery() -> BTreeMap<&'static str, &'static str> {
    let mut gallery = BTreeMap::new();
    gallery.insert("葛飾北斎", "富岳三十六景");
    gallery.insert("ミュシャ", "黄道二宮");
    gallery
}
