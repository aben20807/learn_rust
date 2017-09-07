---
slideOptions:
  transition: slide

---

<style>
    #unsplash {
        text-align: right;
        font-size: 30%;
        padding-right: 2.3em;
        color: #fff;
	}
    #left_text {
        text-align: left;
        padding-left: 1.3em;
    }
</style>

# Rust All

<br>

Huang Po Hsuan

<div style="font-size: 60%; padding-right: 2.3em;">

大部分來源：[The Rust Programming Language - 2nd](https://doc.rust-lang.org/book/second-edition/)

</div>
<br>

![創用CC](https://i.imgur.com/L67RG8Y.png =100x)

---

## 簡介

----

## 三大核心
+ Ownership
+ Borrow
+ Lifetime

---

<!-- .slide: data-background="https://2.bp.blogspot.com/-Uwg-xbVWfWc/WW93ogPHHvI/AAAAAAAALDI/AzlFOGahQ5ssowyn7IvErASdUZrm-7eFgCLcBGAs/s640/2017-07-19_231243.png" data-background-size="900px" data-background-color="#fff" data-foreground-color="#fff" -->
<!-- 官方下載點截圖 -->


<div style="background: rgba(0, 0, 0, 0.8); padding: 10px; margin-left: 330px; margin-right: 330px;">

## <font color="#ffffff">安裝</font>

[官網下載點](https://www.rust-lang.org/en-US/install.html)

</div>

----


<div style="text-align: left;">

可參考[106.07.19 cygwin+vim+rust](https://aben20807.blogspot.tw/2017/07/1060719-cygwinvimrust.html)

<br>
<br>

那時候才1.18.0
2017/08/31就發布1.20.0了OAO

</div>

----

#### 若要從舊版升級OuO

<br>

```shell
$ rustup update stable
```
![自己截圖升級過程](http://imgur.com/ovbhl3I.png =700x)

---

<!-- .slide: data-background="https://imgur.com/OX54aE5.png" data-background-size=100% data-background-color="#fff" data-foreground-color="#fff" -->
<!-- 銀河 -->

## [Hello, world!](https://stackoverflow.com/a/12785204)

<div id="unsplash">
<br><br><br><br><br><br><br><br><br><br><br><br>

Photo by Thom Schneider on Unsplash

</div>

----

<div style="text-align: left; padding-left: 1.3em;">

##### 終端機：

</div>

```shell
$ vim main.rs
```

<br>
<br>

<div style="text-align: left; padding-left: 1.3em;">

##### main.rs檔案：

</div>

```rust
fn main() {
    println!("Hello, world!");
}
```



----

#### 編譯並執行

<br>

```shell
$ rustc main.rs
$ ./main
Hello, world!
```

----

有沒有看到有分號OuO
<br>
![與分號的躲貓貓遊戲](https://i.pinimg.com/564x/72/03/68/72036883039a3491283727d23a4adf9f.jpg)

---

<div style="background: rgba(0, 0, 0, 0.8); padding: 10px; color: #fff; margin-left: 160px; margin-right: 160px;">

## [Cargo](rust-lang/cargo)

<!-- .slide: data-background="https://imgur.com/KWzCRzl.png" data-background-size=100% data-background-color="#fff" data-foreground-color="#fff" -->
<!-- 貨櫃 -->

<br>

+ 內建的強大專案管理器
+ 建置時自動抓取依存模組

<div id="unsplash">
<br>

Photo by chuttersnap on Unsplash

</div>
</div>

----

#### 新增專案

<br>

```shell
$ cargo new hello_cargo --bin
$ cd hello_cargo
$ vim src/main.rs
```

<br>

+ hello_cargo 為自訂專案名
+ \-\-bin 為欲產生可執行程式，不加為函式庫

----

開啟 src/main.rs 後裡面會有預設程式

<br>

```rust
fn main() {
    println!("Hello, world!");
}
```

----

#### 執行

<br>

```shell
$ cargo build # 建置
$ cargo run   # 建置並執行
```

<br>

<div style="text-align: left; padding-left: 1.3em;">

一般可直接執行 `cargo run`

</div>

<br>
<div style="text-align: right; font-size: 50%; padding-right: 2.3em;">

[其他詳細說明在此](https://doc.rust-lang.org/book/second-edition/ch01-02-hello-world.html)

</div>

---

## 變數

<br>

![到變數未初始化](https://i.pinimg.com/564x/81/7d/31/817d317eecb8af559b39be77577c2b2e.jpg =300x)

----

+ 變數預設不可變的
+ 常數
+ Shadowing

----

## 不可變?!

![前赤壁賦](https://imgur.com/jmUqQy8.png =300x)

----

<div style="text-align: left; padding-left: 1.3em;">

#### 用 `let` 宣告預設不可變 [:arrow_forward:](https://play.rust-lang.org/?gist=4c8b9868ac62c7d0c7f1fd05e1adfc9e&version=stable)

</div>

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

<br>

<div style="text-align: left; padding-left: 1.6em; font-size: 70%;">

#### 執行結果：

</div>

```
error[E0384]: re-assignment of immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ re-assignment of immutable variable
```

----

<div style="text-align: left; padding-left: 1.3em;">

#### 讓變數可變：`mut` [:arrow_forward:](https://play.rust-lang.org/?gist=bed9e80812f8dc4a67b86c53ba3d3e76&version=stable)

</div>

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

<br>

<div style="text-align: left; padding-left: 1.6em; font-size: 70%;">

#### 執行結果：

</div>

```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

----

## 常數

<br>

![對常數微分就消失了](http://4.bp.blogspot.com/-w5Hkl25JMSk/U8VAfszayiI/AAAAAAAAtK8/ofHIzOUhBFc/s1600/1604612_247137385453986_911407242_n.jpg =400x)

----

<div style="font-size: 75%; padding-left: 1.1em;">

+ `const` 關鍵字
+ 不能使用 `mut`：常數非只有預設不可變，是永遠不可變
+ 只能由表達式賦值，不能是函式呼叫的結果、執行時期得到的值
+ 整個執行時期都是有效的
+ 命名慣例是使用全大寫英文，並使用底線連接單字

</div>

<br>

```rust
const MAX_POINTS: u32 = 100_000;
```

----

<!-- .slide: data-background="https://imgur.com/AS6RaP9.png" data-background-size=100% data-background-color="#fff" data-foreground-color="#fff" -->
<!-- 小狐狸遮右眼 -->

<div style="text-align: right;">

#### Shadowing (遮蔽)

<br><br><br>

<div style="padding-right: 0.7em; font-weight: bold;">

+ 可換型別


</div>
<div id="unsplash">
<br><br><br><br><br><br><br><br><br><br><br>
    
Photo by Ray Hennessy on Unsplash

</div>
</div>

----

<div style="text-align: left; padding-left: 1.3em;">

#### e.g. 計算空白數 [:arrow_forward:](https://play.rust-lang.org/?gist=299ac388280f975ed709ca79ddd472d0&version=stable)

<br>
</div>

```rust
fn main() {
    let spaces = "   ";        // 字串不需要儲存
    let spaces = spaces.len(); // 遮蔽
    println!("{} 個空白", spaces);
}
```

----

<br>
<div style="text-align: left; padding-left: 1.3em;">

#### 若換成

</div>

```rust
    let mut spaces = "   ";
    spaces = spaces.len();
```

<br>
<div style="text-align: left; padding-left: 1.3em;">

#### 就會吐出無法轉型的錯誤

</div>

```
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected &str, found usize
  |
  = note: expected type `&str`
             found type `usize`
```

----

[more](https://doc.rust-lang.org/book/second-edition/ch03-01-variables-and-mutability.html)

---

## 資料型別

----

<div style="font-size: 80%; line-height: 140%;">

+ Rust 為[靜態語言](https://prateekvjoshi.com/2014/10/03/static-vs-dynamic-typing/)(編譯時期型別必須已知)
+ 純量
    + 整數
    + 浮點數
    + 布林
    + 字元
+ 複合體
    + tuple (多元組)
    + 陣列

</div>

----

## 整數

<br>

![數羊overflow](https://i.pinimg.com/originals/b5/ae/1a/b5ae1a276f134199006f003c73edb923.png =700x)

----

| 大小     | 有號   | 無號    |
|:------: | :----: | :----: |
| 8-bits  | i8     | u8     |
| 16-bits | i16    | u16    |
| 32-bits | i32    | u32    |
| 64-bits | i64    | u64    |
| 系統架構 | isize  | usize  |

----

+ 預設為 `i32`
+ `isize` 和 `usize` 取決於執行的電腦架構
+ 後綴型別修飾 e.g. `57u8`
+ 視覺化的分隔 e.g. `1_000`

----

| 表示             | 範例          |
| :--------:      | --------     |
| 十進位           | `98_222`     |
| 十六進位         | `0xff`       |
| 八進位           | `0o77`       |
| 二進位           | `0b1111_0000`|
| 位元組 (`u8` 限定)| `b'A'`       |

----

## 浮點數

<br>

![0.1+0.2 != 0.3](http://stackoverflow.max-everyday.com/wp-content/uploads/2017/04/17796284_1680803348603304_4007199887125412826_n.jpg =350x)

----

<div style="text-align: left; padding-left: 1.3em;">

+ 根據 [IEEE-754標準](https://zh.wikipedia.org/wiki/IEEE_754)
+ `f32` ：單精度、`f64` ：雙精度
+ 預設為 `f64` (因為速度差不多)

</div>
<br>

```rust
fn main() {
    let x = 2.0;      // f64
    let y: f32 = 3.0; // f32
}
```

----

## 數值五(?)則運算

<br>

```rust
fn main() {
    let sum = 5 + 10;            // 加
    let difference = 95.5 - 4.3; // 減
    let product = 4 * 30;        // 乘
    let quotient = 56.7 / 32.2;  // 除
    let remainder = 43 % 5;      // 取餘數
}
```

----

## 布林

<br>

![!false == true](http://www.webdevelopersnotes.com/wp-content/uploads/joke-false-itss-funny-because-its-true.png =400x)

----

<div style="text-align: left; padding-left: 1.3em;">

+ 真 (true)
+ 假 (false)

</div>
<br>

```rust
fn main() {
    let t = true;
    let f: bool = false; // 有型別標示
}
```

----

## 字元

<br>

char
/ˋkærɪktɚ/
<audio src="https://s.yimg.com/tn/dict/dreye/live/f/character.mp3" controls preload></audio>

----

Unicode \!\!\!\!

![接龍都是沒看過的符號](http://blog.thoward37.me/articles/unicode-string-detection/andysinger_scrabbletiles.png =300x)

----

emoji 也可以 [:heart_eyes_cat:](https://play.rust-lang.org/?gist=4fbf63c048774d7e7609d23dd5c2bd85&version=stable)
```rust
fn main() {
   let c = 'z';
   let z = 'ℤ';
   let heart_eyed_cat = '😻';
}
```

----

<!-- .slide: data-background="https://1.bp.blogspot.com/-V0kkyz3f6Tg/Us1q96g-IZI/AAAAAAAAKHc/BF5BnBNgawM/s1600/s0103.jpg" data-background-size="500px" data-background-color="#fff" data-foreground-color="#fff" -->
<!-- 百獸王戰隊 -->

<div style="text-align: left;">

## 複合型別

+ Tuple
+ 陣列

</div>

----

## Tuple
![漢堡組成](https://d3bfrz6ajmxups.cloudfront.net/blog/mf/wp-content/uploads/2015/05/20175717/burger_toppings_memorial_day-628x1024.jpg =300x)

----

<div style="text-align: left; padding-left: 1.3em;">

+ 使用 `()`
+ 內部用 `,` 隔開
+ 內部型別不須相同
+ ~~根本超像漢堡~~

</div>
<br>

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

----

可用形式配對來取得內部的值 [:arrow_forward:](https://play.rust-lang.org/?gist=3925eaacfb4809403ff638c29c74df63&version=stable)

<br>

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

----

也可用 `.n` 來取得第n個位置

<br>

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    
    let five_hundred = x.0;
    
    let six_point_four = x.1;
    
    let one = x.2;
}
```

----

當然是從0開始索引OuO

<br>

![Real programmers count from 0](http://media02.hongkiat.com/programming-jokes/joke-programmer-count.jpg =600x)

----

## 陣列
![五層牛肉吉事堡](http://livedoor.blogimg.jp/news4wide/imgs/3/e/3e2bae85.jpg =300x)

----

<div style="text-align: left; padding-left: 1.3em;">

+ 使用 `[]`
+ 內部用 `,` 隔開
+ 內部型別必須相同
+ ~~根本超像只夾肉的漢堡~~
+ 存在 stack 裡，長度固定 (vector 可變

</div>
<br>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

----

同樣可用索引取值

<br>

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

----

## 索引超出大小

<br>

+ tuple --> 編譯時期：error[\[E0612\]](https://doc.rust-lang.org/error-index.html#E0612)
+ 陣列 --> 執行時期：panicked

----

[more](https://doc.rust-lang.org/book/second-edition/ch03-02-data-types.html)

---

## Functions

----

<div style="text-align: left; padding-left: 1.3em;">

+ 使用 `fn` 關鍵字，`{}` 包著
+ `main` 就是函式
+ 程式進入點

</div>
<br>

```rust
fn main() {
    println!("Hello, world!");
}
```

<br>
<div style="text-align: left; padding-left: 2.5em; font-size: 60%">

p.s. `println!` 不是函式

</div>

----

<br>
<!-- .slide: data-background="https://imgur.com/9Iq4L3q.png" data-background-size=100% data-background-color="#000" -->
<!-- 小蛇 -->

+ 使用 `fn` 關鍵字，`{}` 包著
+ 命名規則：snake case (字母小寫，底線連接單字)

<br><br><br><br><br><br>
<div id="unsplash">

Photo by Overture Creations on Unsplash

</div>

----

<div style="text-align: left; padding-left: 1.3em;">

範例 [:arrow_forward:](https://play.rust-lang.org/?gist=bb60c301ea59ca63b1a5fb858589c6a4&version=stable)

</div>
<br>

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

----

## 參數 v.s. 參數

<div style="text-align: left; padding-left: 1.3em; font-size: 80%">
<br>

##### Parameter  &nbsp;&nbsp;&nbsp;&nbsp;/pəˋræmətɚ/



</div>

```rust
fn foo(x: i32) { // formal parameter
       ^^^^^^ 
}
```

<br>
<div style="text-align: left; padding-left: 1.3em; font-size: 80%">

##### Argument

</div>

```rust
foo(4); // actual argument
    ^
```

----

+ 傳入的型別必須標示
+ 可多個、多種，用 `,` 分隔

----

## 函式主體
+ `{}` 內部
+ 陳述式 + 表達式
+ 陳述式可有可無可多可少
+ 表達式回傳數值用

----

|陳述式|表達式|
|:-:|:-:|
|Statement|Expression|
|無回傳值|有|
|結束有 `;`|無 `;`|
|`let y = 6;`|`5 + 6`|

----

<div id="left_text">
    
#### 錯誤示範

</div>

```rust
fn main() {
    let x = (let y = 6);
}
```

<br>
<div id="left_text">
    
#### 結果

</div>

```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: variable declaration using `let` is a statement
```

----

<div id="left_text">
    
利用 `{}` 進行區塊運算再回傳 [:arrow_forward:](https://play.rust-lang.org/?gist=37a5303f5048a1a40e2e9c2c5e06b82f&version=stable)

</div>
<br>

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

----

## 函式回傳值
![貓的報恩](https://i.pinimg.com/564x/a0/0d/16/a00d168cd6c7ad48ecaeeb73af999ccf.jpg =270x)

----

<div id="left_text">
    
+ `()` 後方用 `->` 定義回傳型別
+ 用表達式來回傳 [:arrow_forward:](https://play.rust-lang.org/?gist=aad902521f671b6a956b01e4aff75a10&version=stable)

</div>
<br>

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

----

[more](https://doc.rust-lang.org/book/second-edition/ch03-03-how-functions-work.html)

---

## 註解
![要刪掉還是註解起來等客戶回心轉意](http://i.imgur.com/ew4Agpt.jpg =400x)

----

<div id="left_text">

+ 大致上與 C 家族很像
+ 單行 `//`
+ 區塊 `/**/`

</div>
<br>

```rust
fn main() {
    // let x = 5;
    /*
    let y = {
        let x = 3;
        x + 1
    };
    */
}
```

----

<!-- .slide: data-background="https://imgur.com/g8975gX.png" data-background-size=100% data-background-color="#000" -->
<!-- 鳥巢 -->

<div id="left_text">
    
不同的是 `/**/` 竟然支援巢狀註解 ([C 家族不行](https://stackoverflow.com/a/6797546))

</div>
<br>
<div style="margin-left: 540px; margin-right: 0px; padding: 0.1%; background-color: #000;">

```rust
fn main() {
    // let x = 5;
    /*
    let y = {
        /*
        let x = 3;
        x + 1
        */
    };
    */
}
```

</div>
<div id="unsplash">

Photo by Jerry Kiesewetter on Unsplash

</div>

----

<div id="left_text">
    
另外還有產生文件用的 (之後再深入了解)

</div>

```rust
/// # Examples
///
/// ```
/// use std::rc::Rc;
///
/// let five = Rc::new(5);
/// ```
```

<br>

```rust
mod foo {
    //! This is documentation for the `foo` module.
    //!
    //! # Examples
}
```

----

[more](https://doc.rust-lang.org/stable/reference/comments.html)

---

<!-- .slide: data-background="https://imgur.com/dLJX4sp.png" data-background-size=100% data-background-color="#000" -->
<!-- 計算機組織hw3 -->

<div style="background: rgba(0,0,0,0.8); margin-left: 300px;  margin-right: 300px;">
    
## 流程控制

</div>

----

## `if`

----

<div id="left_text">

+ `if` 的條件只限布林 (C 家族可用數字)
+ 不須用 `()` 包住條件
+ 有 `else`、`else if`

</div>
<br>

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

----

<div id="left_text">

+ 可搭配 `let` 賦值
+ 但前提是型別相同，e.g. 若 `5`、`"six"` 會錯誤

</div>
<br>

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

----

## 迴圈
+ `loop`
+ `while`
+ `for`

----

## `loop`

![咖啡迴圈](http://wanna-joke.com/wp-content/uploads/2015/04/coffee-slepp-comics-loop.jpg =400x)

----

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

<br>
<div id="left_text">

##### 結果

</div>

```
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

<div id="left_text">
    
+ 用 ctrl+C 結束程式

</div>

----

<div id="left_text">
    
1.18.0 新功能

使用 `break` 回傳數值

</div><br>

```rust
let x = loop {
            break 7; 
        };
```

----

## `while`

![while出門，買些牛奶](https://i.redd.it/kfnatjoysbaz.jpg =500x)

----

<div id="left_text">
    
+ 條件一樣不需 `()`

</div><br>

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

----

## `for`

----

<div id="left_text">
    
+ 比較像 for-each
+ 操作有長度的較不會超出邊界而 panic

</div><br>

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

----

<div id="left_text">

傳統 `for` 用法 (~~啊還不是for-each~~
以下為模擬倒數

</div><br>

```rust
fn main() {
    for number in (1..4).rev() { // rev()為反轉
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

---

## Ownership (所有權)

<br>

![給鑰匙](http://www.law-right.com/wp-content/uploads/2016/02/Ownership-2-800x738.jpg =400x)

----

+ Rust 特有
+ 編譯時期設定好記憶體管理規則
+ 使得
    + 不需要[垃圾回收機制 (GC)](http://blog.takipi.com/7-things-you-thought-you-knew-about-garbage-collection-and-are-totally-wrong/)
    + 不需要自己 [free](https://blog.gtwang.org/programming/memory-deallocation-issues-in-c/)
    + No run-time costs

<br>

![java吉祥物丟垃圾](http://384uqqh5pka2ma24ild282mv.wpengine.netdna-cdn.com/wp-content/uploads/2014/09/Blog_Trash-1.jpg =400x)

----

## 先談談 [Stack v.s. Heap](http://androidexample.com/Use_of_Heap_and_Stack_memory_in_java/index.php?view=article_discription&aid=143&aaid=159)

<br>

+ 執行時期可用的記憶體
+ 實作方式不同

![heap_stack](http://androidexample.com/upload/content/heap_stack.png =400x)

----

## Stack

+ 使用 stack：last in, first out (LIFO)
+ push、pop
+ 較快 ∵因為不需搜尋、資料已知且大小固定

![stack push pop](https://www.tutorialspoint.com/data_structures_algorithms/images/stack_representation.jpg =400x)

----

<!-- .slide: data-background="https://imgur.com/Kni6ZzU.png" data-background-size=100% data-background-color="#000" -->
<!-- 一堆石頭 -->
<div style="background: rgba(0,0,0,0.7)">

## Heap

+ 不是用 heap [(?)](https://en.wikipedia.org/wiki/Talk:Heap_(data_structure)#Origin_of_Phrase_.22the_Heap.22)
+ 動態配置用 (資料編譯時期未知或可能改變
+ 配置 (allocating) 過程：

<div style="font-size: 70%; text-align: left; padding-left: 6em">

要求空間 -> OS尋找夠大空間並標記已使用 -> 回傳指標

<br></div></div>
<br><br>
<div id="unsplash">

Photo by Todd Downs on Unsplash

</div>

----

|Stack|Heap|
|:-:|:-:|
|LIFO|[不一定](https://stackoverflow.com/a/13784956)|
|固定大小|非固定|
|快|較慢<br>(∵需透過指標)|

----

## Ownership 規則

<br>

1. 每個數值都有一個變數，稱作 *owner*
2. 同一時間 owner 只能有一個
3. owner 離開有效範圍時，該數值就會被丟棄

----

<div id="left_text">

通常 `{}` 內就是一個有效範圍

</div>
<br>

```rust
{                      // s 無效 ∵尚未宣告
    let s = "hello";   // s 從此行開始有效，s 為 "hello" 的 owner

    // 對 s 進行操作
}                      // s 無效 ∵已離開有效範圍
```

----

<div id="left_text">

## String

+ != string literals
+ 配置於 heap
+ 從  string literal 產生 String：使用 `from` 函式
+ `::` 可使用其他命名空間的運算子

</div>

```rust
let s = String::from("hello");
```

<br>
<div id="left_text";>
<div style="font-size: 50%">
    
為了測試突然講了一堆啊 :sweat_smile:

</div></div>

----

```rust
{
    let s = String::from("hello"); // s 從此行開始有效

    // 對 s 進行操作
}                                  // s 無效 ∵已離開有效範圍
```

<br>
<div id="left_text";>
<div style="font-size: 80%;">

當變數離開有效範圍，`drop` 函式會**自動**在 `}` 被呼叫

<br>

類似 C++ 的 Resource Acquisition Is Initialization ([RAII](https://zh.wikipedia.org/wiki/RAII))：

<div style="text-align: right;">

物件建構時取得資源，解構時完成資源的釋放

</div></div></div>

----

## 變數與資料互動方式

<br>

+ Move
+ Clone
+ Copy (Stack 限定)

----

## Move

```rust
let s1 = String::from("hello");
let s2 = s1;
```