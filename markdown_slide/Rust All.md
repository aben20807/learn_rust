---
slideOptions:
  transition: slide
---

# Rust 從頭到尾

---

<!-- .slide: data-background="https://2.bp.blogspot.com/-Uwg-xbVWfWc/WW93ogPHHvI/AAAAAAAALDI/AzlFOGahQ5ssowyn7IvErASdUZrm-7eFgCLcBGAs/s640/2017-07-19_231243.png" data-background-size="900px" data-background-color="#fff" data-foreground-color="#fff" -->


<div style="background: rgba(0, 0, 0, 0.8); padding: 10px;">

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
![](http://imgur.com/ovbhl3I.png =700x)

---

## Hello, world!

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

---

## [Cargo](rust-lang/cargo)

<br>

+ 內建的強大專案管理器
+ 建置時自動抓取依存模組

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

----

+ 變數預設不可變的
+ 常數
+ Shadowing

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

#### 常數

<br>

<div style="font-size: 75%; padding-left: 1.1em;">

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

#### Shadowing

+ 遮蔽
+ 可換型別

----

<div style="text-align: left; padding-left: 1.3em;">

#### e.g. 數空白 [:arrow_forward:](https://play.rust-lang.org/?gist=299ac388280f975ed709ca79ddd472d0&version=stable)

</div>

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{} 個空白", spaces);
}
```

<br>
<div style="text-align: left; padding-left: 1.6em; font-size: 70%;">

#### 若換成

</div>

```rust
    let mut spaces = "   ";
    spaces = spaces.len();
```

----

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

---

## 資料型別

<div style="font-size: 80%; line-height: 140%;">

+ Rust 為靜態語言(編譯時期型別必須已知)
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

#### 整數

<br>
<div style="font-size: 80%;">

| 大小     | 有號   | 無號    |
|:------: | :----: | :----: |
| 8-bits  | i8     | u8     |
| 16-bits | i16    | u16    |
| 32-bits | i32    | u32    |
| 64-bits | i64    | u64    |
| 系統架構 | isize  | usize  |

</div>

----

+ `isize` 和 `usize` 取決於執行的電腦架構
+ 後綴型別修飾 e.g. `57u8`
+ 視覺化的分隔 e.g. `1_000`

<br>
<br>
<div style="font-size: 80%;">

| 表示             | 範例          |
| :--------:      | --------     |
| 十進位           | `98_222`     |
| 十六進位         | `0xff`       |
| 八進位           | `0o77`       |
| 二進位           | `0b1111_0000`|
| 位元組 (`u8` 限定)| `b'A'`       |

</div>