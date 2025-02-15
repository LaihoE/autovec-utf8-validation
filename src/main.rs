#![feature(test)]
extern crate test;
use test::*;

use autovec_utf8_validator::validate_utf8;
use autovec_utf8_validator::validate_utf8_max3;
use simdutf8::basic::from_utf8;

// taken from: https://zh.wikipedia.org/wiki/Rust
const CHINESE: &str = "Rust是由Mozilla[11]主导开发的通用、编译型编程语言。设计准则为“安全、并发、实用”[12][13]，支持函数式、並行式、过程式以及面向对象的程式設計风格。
Rust語言原本是Mozilla員工Graydon Hoare的個人專案，而Mozilla於2009年開始贊助這個專案 [14]，並且在2010年首次公開[15]。也在同一年，其編譯器原始碼開始由原本的OCaml語言轉移到用Rust語言，進行自我編譯工作，稱做「rustc」[16]，並於2011年實際完成[17]。這個可自我編譯的編譯器在架構上採用了LLVM做為它的後端。
第一個有版本號的Rust編譯器於2012年1月釋出[18]。Rust 1.0是第一個穩定版本，於2015年5月15日釋出[19]。
Rust在完全公開的情況下開發，並且相當歡迎社区的回饋。在1.0穩定版之前，語言設計也因為透過撰寫Servo網頁瀏覽器排版引擎和rustc編譯器本身，而有進一步的改善。它雖然由Mozilla資助，但其實是一個共有專案，有很大部分的程式碼是來自於社区的貢獻者[20]。
Rust是由Mozilla[11]主导开发的通用、编译型编程语言。设计准则为“安全、并发、实用”[12][13]，支持函数式、並行式、过程式以及面向对象的程式設計风格。
Rust語言原本是Mozilla員工Graydon Hoare的個人專案，而Mozilla於2009年開始贊助這個專案 [14]，並且在2010年首次公開[15]。也在同一年，其編譯器原始碼開始由原本的OCaml語言轉移到用Rust語言，進行自我編譯工作，稱做「rustc」[16]，並於2011年實際完成[17]。這個可自我編譯的編譯器在架構上採用了LLVM做為它的後端。
第一個有版本號的Rust編譯器於2012年1月釋出[18]。Rust 1.0是第一個穩定版本，於2015年5月15日釋出[19]。
Rust在完全公開的情況下開發，並且相當歡迎社区的回饋。在1.0穩定版之前，語言設計也因為透過撰寫Servo網頁瀏覽器排版引擎和rustc編譯器本身，而有進一步的改善。它雖然由Mozilla資助，但其實是一個共有專案，有很大部分的程式碼是來自於社区的貢獻者[20]。
Rust的設計目標之一，是要使設計大型的網際網路客户端和伺服器的任務變得更容易[21]。因此更加強調安全性、記憶體配置、以及並行處理等方面的特性。
在效能上，具有額外安全保證的代碼會比C++慢一些，例如Rust对数组进行操作时会進行边界检查（尽管可以通过一些方式[22]绕过[23]），而C++则不会，但是如果等價的C++代碼作手工检查，則兩者效能上是相似的[24]。
比起C/C++，Rust编译器的对于代码中错误的提示更清晰明瞭，开发者可根据提示轻松地修复代码中的错误。
由於其編譯器會做出額外的安全檢查，Rust的編譯速度有時低於C/C++。
Rust的語法設計，與C語言和C++相當相似，區塊（block）使用大括號隔開，控制流程的關鍵字如if、else、while等等。在保持相似性的同時，Rust也加進了新的關鍵字，如用於模式匹配的match（與switch相似）則是使用C/C++系統程式語言的人會相對陌生的概念。儘管在語法上相似，Rust的語義（semantic）和C/C++非常不同。
為了提供内存安全，它的設計不允許空指標和懸空指標[25][26]。指針只能透過固定的初始化形態來建構，而所有這些形態都要求它們的輸入已經分析過了[27]。Rust有一個檢查指標生命期間和指標凍結的系統，可以用來預防在C++中許多的型別錯誤，甚至是用了智能指针功能之後會發生的型別錯誤。
Rust设计了一个所有权系统，其中所有值都有一个唯一的所有者，并且值的作用域与所有者的作用域相同。值可以通过不可变引用（&T）、可变引用（&mut T）或者通过值本身（T）传递。任何时候，一个变量都可以有多个不可变引用或一个可变引用，这实际上是一个显式的读写锁。Rust编译器在编译时强制执行这些规则，并检查所有引用是否有效。[28][29]
早期的Rust雖然有垃圾回收系統，但非如Java或.NET平台的全自動垃圾回收。Rust 1.0已不再使用垃圾回收器，而是全面改用基于引用计数的智能指针来管理内存。
它的型別系統直接地模仿了Haskell語言的类型类概念，並把它稱作「traits」，可以把它看成是一種特设多态。Rust的作法是透過在宣告型別變數（type variable）的時候，在上面加上限制條件。至於Haskell的高階型別變數（Higher-kinded polymorphism）則還未支援。
型別推導也是Rust提供的特性之一，使用let語法宣告的變數可以不用宣告型別，亦不需要初始值來推斷型別。但如果在稍後的程式中從未指派任何值到該變數，編譯器會發出編譯時（compile time）錯誤[30]。函數可以使用泛型化參數（generics），但是必須綁定Trait。不能使用方法或運算子而不声明它們的型別，每一項都必確明確定義。
Rust的物件系統是基於三樣東西之上的，即實作（implementation）、Trait以及結構化資料（如struct）。實作的角色類似提供Class關鍵字的程式語言所代表的意義，並使用impl關鍵字。繼承和多型則通过Trait實現，它們使得方法（method）可以在實作中被定義。結構化資料用來定義欄位。實作和（trait）都無法定義欄位，並且只有（trait）可以提供繼承，藉以躲避C++的「鑽石繼承問題」（菱型缺陷）。
它的型別系統直接地模仿了Haskell語言的类型类概念，並把它稱作「traits」，可以把它看成是一種特设多态。Rust的作法是透過在宣告型別變數（type variable）的時候，在上面加上限制條件。至於Haskell的高階型別變數（Higher-kinded polymorphism）則還未支援。
型別推導也是Rust提供的特性之一，使用let語法宣告的變數可以不用宣告型別，亦不需要初始值來推斷型別。但如果在稍後的程式中從未指派任何值到該變數，編譯器會發出編譯時（compile time）錯誤[30]。函數可以使用泛型化參數（generics），但是必須綁定Trait。不能使用方法或運算子而不声明它們的型別，每一項都必確明確定義。
Rust的物件系統是基於三樣東西之上的，即實作（implementation）、Trait以及結構化資料（如struct）。實作的角色類似提供Class關鍵字的程式語言所代表的意義，並使用impl關鍵字。繼承和多型則通过Trait實現，它們使得方法（method）可以在實作中被定義。結構化資料用來定義欄位。實作和（trait）都無法定義欄位，並且只有（trait）可以提供繼承，藉以躲避C++的「鑽石繼承問題」（菱型缺陷）。
它的型別系統直接地模仿了Haskell語言的类型类概念，並把它稱作「traits」，可以把它看成是一種特设多态。Rust的作法是透過在宣告型別變數（type variable）的時候，在上面加上限制條件。至於Haskell的高階型別變數（Higher-kinded polymorphism）則還未支援。
型別推導也是Rust提供的特性之一，使用let語法宣告的變數可以不用宣告型別，亦不需要初始值來推斷型別。但如果在稍後的程式中從未指派任何值到該變數，編譯器會發出編譯時（compile time）錯誤[30]。函數可以使用泛型化參數（generics），但是必須綁定Trait。不能使用方法或運算子而不声明它們的型別，每一項都必確明確定義。
Rust的物件系統是基於三樣東西之上的，即實作（implementation）、Trait以及結構化資料（如struct）。實作的角色類似提供Class關鍵字的程式語言所代表的意義，並使用impl關鍵字。繼承和多型則通过Trait實現，它們使得方法（method）可以在實作中被定義。結構化資料用來定義欄位。實作和（trait）都無法定義欄位，並且只有（trait）可以提供繼承，藉以躲避C++的「鑽石繼承問題」（菱型缺陷）。
型別推導也是Rust提供的特性之一，使用let語法宣告的變數可以不用宣告型別，亦不需要初始值來推斷型別。但如果在稍後的程式中從未指派任何值到該變數，編譯器會發出編譯時（compile time）錯誤[30]。函數可以使用泛型化參數（generics），但是必須綁定Trait。不能使用方法或運算子而不声明它們的型別，每一項都必確明確定義。
";

pub fn main() {
    // Check that the result is the same as stdlib
    for i in 0..CHINESE.len() {
        let slice = &CHINESE.as_bytes()[0..i];
        let result = validate_utf8(slice);
        let correct = std::str::from_utf8(&slice).is_ok();
        if result != correct {
            panic!("INCORRECT RESULT: {} != {}", result, correct);
        }
    }

    // Check that the result is the same as stdlib (MAX 3)
    for i in 0..CHINESE.len() {
        let slice = &CHINESE.as_bytes()[0..i];
        let result = validate_utf8_max3(slice);
        let correct = std::str::from_utf8(&slice).is_ok();
        if result != correct {
            panic!("INCORRECT RESULT: {} != {}", result, correct);
        }
    }
    println!("RESULT IS OK")
}

#[bench]
fn bench_japanese_std(b: &mut Bencher) {
    b.iter(|| std::str::from_utf8(black_box(CHINESE.as_bytes())));
}

#[bench]
fn bench_simd_japanese_autovec(b: &mut Bencher) {
    b.iter(|| validate_utf8(black_box(CHINESE.as_bytes())));
}

#[bench]
fn bench_simd_japanese_autovec_max3(b: &mut Bencher) {
    b.iter(|| validate_utf8_max3(black_box(CHINESE.as_bytes())));
}

#[bench]
fn bench_simd_japanese_simdutf8(b: &mut Bencher) {
    b.iter(|| from_utf8(black_box(CHINESE.as_bytes())));
}
