mod a {
    struct TypeA {
        a2: a_2::TypeA2,
    }

    mod a_1 {
        struct TypeA1 {
            // 親が見えるものは見える
            a: Box<super::TypeA>,
            a2: Box<super::a_2::TypeA2>,
        }
    }
    mod a_2 {
        pub struct TypeA2 {
            // 親が見えるものは見える
            a: Box<super::TypeA>,
        }
    }
}

mod b {
    pub struct TypeB;
    mod b_1 {
        pub struct TypeB1;
    }
    pub mod b_2 {
        pub struct TypeB2;
    }
}

// ここでいう要素とは、構造体・列挙型・関数・トレイトなど
// 1. 親モジュールは、自身の子要素が見える
// 2. 親モジュールは、子モジュール内のpubな要素(子要素の子要素 => 孫要素)が見える
// 3. 親モジュールは、2. 3.で得られた要素のpubな要素がさらに見える(再帰的)
// 4. 子要素は、自身の属するモジュールが見える要素が見える

// ここはクレートルート(=> 親モジュール)であることを忘れずに！！
// 子要素であるmod a, bは必ず見れる
fn main() {
    let b = b::TypeB;

    let b2 = b::b_2::TypeB2;
}
