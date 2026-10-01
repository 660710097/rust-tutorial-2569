# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 02
> **Topic No.:** 02
> **Topic Name:** Rust Environment, Cargo & First Program
> **ประเด็นหลักที่ควรครอบคลุม:** การติดตั้ง, rustc, Cargo, project structure, cargo new, cargo run, cargo build

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายระพีพัทธ์ ชอบสูงเนิน | 660710097 | `@660710097` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายสุธิสิทธิ์ กลิ่นพยอม | 660710102 | `@660710102` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายกฤตภณธ์ เดชาถิรปัญญา | 660710107 | `@660710107` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายจักรกฤช คิดดี | 660710108 | `@660710108` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

## 3. Introduction

`หัวข้อนี้เป็นการแนะนำพื้นฐานของภาษา Rust ตั้งแต่การเตรียมโปรแกรมและสภาพแวดล้อมสำหรับเขียน Rust การใช้ Cargo เพื่อสร้างและจัดการโปรเจกต์ รวมถึงการเขียนโปรแกรม Rust โปรแกรมแรก เพื่อให้เข้าใจขั้นตอนการเขียน ตรวจสอบ และรันโปรแกรมได้อย่างถูกต้อง ซึ่งเป็นพื้นฐานสำคัญก่อนที่จะไปเรียนเรื่องอื่น ๆ ในภาษา Rust`

---

## 4. Key Concepts

### 4.1 `[Rust Environment]`

`Rust Environment คือเครื่องมือและสภาพแวดล้อมที่จำเป็นสำหรับการเขียนและรันโปรแกรม Rust โดยเครื่องมือหลัก ได้แก่ Rust Compiler (rustc) และ Cargo`

**ตัวอย่าง**

```rust
rustc --version
cargo --version
```

**Explanation**

`rustc --version` ใช้ตรวจสอบเวอร์ชันของ Rust Compiler และ `cargo --version` ใช้ตรวจสอบว่า Cargo ติดตั้งอยู่ในเครื่องหรือไม่ 

---

### 4.2 `[Cargo]`

`Cargo คือ **Package Manager และ Build System ของ Rust** ใช้สำหรับสร้างโปรเจกต์ จัดการ Dependencies, Compile และ Run โปรแกรม ทำให้การพัฒนา Rust เป็นระบบมากขึ้น`

```rust
cargo new hello_rust
cd hello_rust
cargo run
```
**Explanation**

`cargo new` สร้างโปรเจกต์ Rust ใหม่พร้อมโครงสร้างพื้นฐาน จากนั้น `cargo run` จะ Compile และรันโปรแกรมในโปรเจกต์นั้น`

---

### 4.3 `[First Rust Program]`

`โปรแกรม Rust แบบง่ายจะเริ่มต้นการทำงานจากฟังก์ชัน main() และสามารถใช้ println!() เพื่อแสดงข้อความออกทางหน้าจอ

```rust
fn main() {
    println!("Hello, Rust!");
}
```
**Explanation**

 `fn main()` คือจุดเริ่มต้นของโปรแกรม ส่วน `println!()` เป็น Macro ที่ใช้แสดงข้อความ `Hello, Rust!` ออกทางหน้าจอ
 
---

## 5. Important Syntax / Rules

| Syntax / Rule            | Meaning                                                             | Example                            |
| ------------------------ | ------------------------------------------------------------------- | ---------------------------------- |
| `fn main() { ... }`      | เป็นจุดเริ่มต้นของโปรแกรม Rust โปรแกรมจะเริ่มทำงานจากฟังก์ชันนี้    | `fn main() { println!("Hello"); }` |
| `println!("...");`       | ใช้แสดงข้อความหรือผลลัพธ์บนหน้าจอ                                   | `println!("Hello, Rust!");`        |
| `use std::io;`           | เรียกใช้เครื่องมือจาก Rust Standard Library เพื่อรับข้อมูลจากผู้ใช้ | `use std::io;`                     |
| `cargo new project_name` | ใช้สร้างโปรเจกต์ Rust ใหม่ พร้อมโครงสร้างพื้นฐานของโปรเจกต์         | `cargo new student_grade`          |
| `Cargo.toml`             | เป็นไฟล์หลักของ Cargo ที่เก็บข้อมูลและการตั้งค่าของโปรเจกต์         | `Cargo.toml`                       |
| `src/main.rs`            | เป็นไฟล์หลักที่เก็บโค้ดของโปรแกรม Rust                              | `src/main.rs`                      |
| `cargo run`              | ใช้ Compile และ Run โปรแกรมในโปรเจกต์                               | `cargo run`                        |

### Important Rules

1. **Rust Program ต้องมี `main()` เป็นจุดเริ่มต้น**
   โปรแกรมของเราจะเริ่มทำงานจาก `fn main()` และทำงานตามลำดับคำสั่งภายในฟังก์ชัน

2. **โปรเจกต์ Rust สามารถจัดการได้ด้วย Cargo**
   ใช้ `cargo new` สำหรับสร้างโปรเจกต์ และ `cargo run` สำหรับ Compile และ Run โปรแกรม

3. **โปรเจกต์ที่สร้างด้วย Cargo มีโครงสร้างที่ชัดเจน**
   โดย `Cargo.toml` ใช้เก็บข้อมูลของโปรเจกต์ และ `src/main.rs` ใช้เก็บโค้ดหลักของโปรแกรม

4. **การแสดงผลเป็นพื้นฐานของ First Program**
   โปรแกรมเริ่มต้นสามารถใช้ `println!()` เพื่อแสดงข้อความและผลลัพธ์บนหน้าจอ

5. **First Program สามารถต่อยอดเป็นโปรแกรมจริงได้**
   ในโปรเจกต์นี้เริ่มจากการแสดงข้อความ แล้วต่อยอดเป็น `Student Grade Calculator PPL` ที่รับคะแนนและคำนวณเกรด
   
---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[รับชื่อจากผู้ใช้]`

**Purpose:** `ตัวอย่างนี้เป็นการรับข้อมูลจากผู้ใช้ โดยให้ผู้ใช้พิมพ์ชื่อ แล้วโปรแกรมนำชื่อที่พิมพ์มาแสดงบนหน้าจอ`

```rust
use std::io;
fn main() {
    let mut name = String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Hello, {}!", name.trim());
}
```

**Expected Output**

```text
[Enter your name:
First
Hello, First!]
```

**Explanation**

`เริ่มจาก use std::io; เพื่อเรียกใช้งานคำสั่งที่เกี่ยวกับการรับข้อมูลจากแป้นพิมพ์`

จากนั้นสร้างตัวแปร `name` เป็น `String` สำหรับเก็บชื่อที่ผู้ใช้กรอกเข้ามา

```rust
let mut name = String::new();
```

คำว่า `mut` หมายถึงตัวแปรนี้สามารถเปลี่ยนค่าได้ เพราะตอนแรก `name` ยังไม่มีข้อมูล และเราจะเอาข้อมูลที่ผู้ใช้พิมพ์มาใส่ทีหลัง

ส่วน

```rust
io::stdin().read_line(&mut name).unwrap();
```

ใช้สำหรับรอรับข้อความจากผู้ใช้ โดยข้อมูลที่กรอกจะถูกเก็บไว้ใน `name`

ที่ต้องใส่ `&mut name` เพราะ Rust ต้องการให้เราอนุญาตให้ฟังก์ชันนี้แก้ไขค่าที่อยู่ในตัวแปร `name`

สุดท้าย

```rust
println!("Hello, {}!", name.trim());
```

เอาชื่อที่รับมาแสดงผล โดย `trim()` ใช้ตัดตัวขึ้นบรรทัดใหม่ที่ติดมากับการกด Enter ออก

---

### Example 2 — `[คำนวณคะแนนและตัดเกรด]`

**Purpose:** `[ตัวอย่างนี้เป็นการนำคะแนนจากหลายส่วนมารวมกัน แล้วใช้ `if` และ `else if` เพื่อตรวจสอบว่าคะแนนรวมได้เกรดอะไร]`

```rust
fn main() {
    let midterm: f64 = 30.0;
    let final_exam: f64 = 28.0;
    let kahoot: f64 = 10.0;
    let project: f64 = 11.0;
    let attendance: f64 = 3.0;
    let typing: f64 = 3.0;

    let total = midterm
        + final_exam
        + kahoot
        + project
        + attendance
        + typing;

    let grade = if total >= 80.0 {
        "A"
    } else if total >= 75.0 {
        "B+"
    } else if total >= 70.0 {
        "B"
    } else if total >= 60.0 {
        "C+"
    } else if total >= 50.0 {
        "C"
    } else if total >= 45.0 {
        "D+"
    } else if total >= 40.0 {
        "D"
    } else {
        "F"
    };

    println!("Total: {:.2}/100", total);
    println!("Grade: {}", grade);
}

```

**Expected Output**

```text
[Total: 85.00/100
Grade: A]
```

**Explanation**

`ในตัวอย่างนี้กำหนดคะแนนของแต่ละส่วนไว้ก่อน เช่น Midterm 30 คะแนน, Final 28 คะแนน และคะแนนส่วนอื่นๆจากนั้นนำคะแนนทั้งหมดมาบวกกันในตัวแปร`total 

```rust
let total = midterm
    + final_exam
    + kahoot
    + project
    + attendance
    + typing;
```

หลังจากได้คะแนนรวมแล้ว โปรแกรมจะตรวจสอบว่าได้เกรดอะไร โดยใช้ `if` และ `else if`

เช่น ถ้าคะแนนรวมตั้งแต่ 80 คะแนนขึ้นไป จะได้ `"A"`

```rust
if total >= 80.0 {
    "A"
}
```

ถ้าไม่ถึง 80 โปรแกรมจะไปตรวจสอบเงื่อนไขต่อไปว่าได้ตั้งแต่ 75 หรือไม่ ถ้ายังไม่ถึงก็จะตรวจสอบเงื่อนไขถัดไปเรื่อย ๆ

ส่วน

```rust
else {
    "F"
}
```

หมายถึงถ้าคะแนนไม่ตรงกับเงื่อนไขไหนเลย ก็ให้เกรด F

สุดท้ายใช้ `println!` แสดงคะแนนรวมและเกรดออกมา

```rust
println!("Total: {:.2}/100", total);
println!("Grade: {}", grade);
```

`{:.2}` ใช้สำหรับแสดงตัวเลขทศนิยม 2 ตำแหน่ง ส่วน `{}` ใช้แสดงค่าทั่วไป เช่นข้อความเกรด

---

## 7. Common Mistakes

### Mistake 1 — `รัน cargo run นอกโฟลเดอร์โปรเจกต์`

**Problem**

`cargo new สร้างโฟลเดอร์ใหม่ให้ แต่ไม่ได้ย้าย directory ให้ ถ้ารัน cargo run ทันทีจะเกิด error`

**Incorrect Code**

```rust
cargo new hello_cargo
cargo run
```

**Correct Code**

```rust
cargo new hello_cargo
cd hello_cargo
cargo run
```

**Why?**

`Cargo อ่านค่ากำหนดโปรเจกต์จากไฟล์ Cargo.toml จึงต้องรันคำสั่งในโฟลเดอร์ของโปรเจกต์ (หรือโฟลเดอร์ย่อยของมัน)`

---

### Mistake 2 — `ลืมส่ง argument ผ่าน -- ตอนใช้ cargo run`

**Problem**

`ต้องการส่ง argument ให้โปรแกรมของเรา แต่พิมพ์ต่อท้าย cargo run ตรงๆ โดยเฉพาะ argument ที่ขึ้นต้นด้วย - Cargo จะตีความว่าเป็น option ของ Cargo เอง และแจ้ง error`

**Incorrect Code**

```rust
cargo run --name Silpakorn
```

**Correct Code**

```rust
cargo run -- --name Silpakorn
```

**Why?**

`เครื่องหมาย -- คั่นระหว่าง option ของ Cargo กับ argument ที่จะส่งต่อให้โปรแกรมของเรา สิ่งที่อยู่หลัง -- Cargo จะไม่ตีความเอง`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `คอมไพล์ผ่านหรือไม่? (Immutable Variable)`

**Problem**

`โค้ดต่อไปนี้คอมไพล์ผ่านหรือไม่ ถ้าผ่านผลลัพธ์คืออะไร ถ้าไม่ผ่านให้บอกว่าบรรทัดไหนผิดเพราะอะไร`

```rust
fn main() {
    let x = 10;
    let mut y = 20;
    y = y + x;
    x = y - 5;
    println!("{} {}", x, y);
}
```

**Hint**

`ตัวแปรใน Rust เป็น immutable โดยค่าเริ่มต้นสังเกตว่าตัวแปรใดถูกกำหนดค่าใหม่ และตัวแปรนั้นประกาศด้วย mut หรือไม่`

**Solution**

```rust
fn main() {
    let mut x = 10;
    let mut y = 20;
    y = y + x;
    x = y - 5;
    println!("{} {}", x, y);
}
```

**Explanation**

`คอมไพล์ไม่ผ่าน ไม่มีผลลัพธ์ เกิด error ที่บรรทัด x = y - 5;
x ประกาศด้วย let x = 10; จึงเป็น immutable ไม่สามารถกำหนดค่าใหม่ได้
y ประกาศด้วย let mut y จึงแก้ค่าได้ บรรทัด y = y + x; ไม่มีปัญหา
Rust ตรวจจับข้อผิดพลาดนี้ตั้งแต่ขั้นคอมไพล์ ก่อนที่โปรแกรมจะรัน
ค่าหลังแก้: y = 20 + 10 = 30 แล้ว x = 30 - 5 = 25 จึงพิมพ์ 25 30`

---

### Exercise 2 — `ผลลัพธ์คืออะไร? (Cargo และ First Program)`

**Problem**

`โปรแกรมจะแสดงผลลัพธ์ใดออกมา`

```rust
fn main() {
    let language = "Rust";
    let tool = "Cargo";

    println!("Language: {}", language);

    if tool == "Cargo" {
        println!("Tool: {}", tool);
    } else {
        println!("Unknown tool");
    }
}
```

`รันคำสั่ง cargo run`

**Hint**

`cargo run จะ Compile และ Run โปรแกรมใน Cargo project
ดูค่าของตัวแปร language และ tool
ตรวจสอบเงื่อนไข if ว่าเป็น true หรือ false
println! แต่ละคำสั่งจะทำงานตามลำดับจากบนลงล่าง`

**Solution**

```rust
    fn main() {
    let language = "Rust";
    let tool = "Cargo";

    println!("Language: {}", language);

    if tool == "Cargo" {
        println!("Tool: {}", tool);
    } else {
        println!("Unknown tool");
    }
}


```

**Explanation**

`โปรแกรม คอมไพล์ผ่านและทำงานได้ตามปกติ
ผลลัพธ์คือ
Language: Rust
Tool: Cargo
cargo run จะทำการ Compile และ Run โปรแกรม
โดยเริ่มจาก println! แรกจึงแสดง Language: Rust จากนั้นตรวจสอบว่า tool มีค่าเท่ากับ "Cargo" หรือไม่ 
ซึ่งเป็นจริง จึงทำงานในส่วน if และแสดง Tool: Cargo ส่วน else จะไม่ถูกทำงาน เพราะเงื่อนไขเป็นจริง`

**Output**
```
Language: Rust
Tool: Cargo
```
---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

`เกี่ยวข้องกับ Syntax ของคำสั่ง Rust เช่น fn main(), println!(), cargo new, cargo build และ cargo run ซึ่งแต่ละคำสั่งมีรูปแบบการเขียนที่กำหนดไว้`

### 9.2 Semantics

`cargo new ใช้สร้าง Project, cargo build ใช้ Compile โปรแกรม และ cargo run ใช้ Compile และ Run โปรแกรม ส่วน rustc ทำหน้าที่ Compile โค้ด Rust
`

### 9.3 Type System

`Rust เป็น Static Typing โดย rustc ตรวจสอบ Type และข้อผิดพลาดบางส่วนตั้งแต่ Compile Time ก่อนรันโปรแกรม`

### 9.4 Memory / Resource Management

`Rust ใช้ Ownership, Borrowing และ Lifetime ในการจัดการ Memory โดย Compiler ตรวจสอบกฎเหล่านี้ก่อนรัน และไม่ต้องใช้ Garbage Collector`

### 9.5 Abstraction / Other PPL Concepts

`Cargo ช่วยจัดการ Project และ Dependencies โดยใช้ Cargo.toml และแบ่งโครงสร้างเป็น src/main.rs ทำให้เกิด Modularity และจัดการ Scope ของโปรแกรมได้เป็นระบบ`

### 9.6 Why Rust?

`Rust ใช้ Static Typing, Ownership และ Compiler Checking เพื่อเพิ่ม Memory Safety และ Reliability พร้อมรักษา Performance สูง`

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / ...]`

10.1 Rust vs. Python
`Syntax: Rust ใช้ fn, let และโครงสร้างของ Cargo ส่วน Python มี Syntax ที่กระชับและใช้ Indentation`
`Semantics / Behavior: Rust ใช้ rustc และ Cargo ในการ Compile และ Run ส่วน Python ทำงานผ่าน Python Runtime`
`Type System: Rust เป็น Static Typing ส่วน Python เป็น Dynamic Typing`
`Memory Management: Rust ใช้ Ownership และ Borrowing ส่วน Python ใช้ Garbage Collection`
`Safety: Rust เน้น Memory Safety และตรวจสอบหลายอย่างก่อน Run ส่วน Python จัดการ Memory อัตโนมัติ`

10.2 Rust vs. C
`Syntax: Rust มี fn, let และ Syntax เฉพาะของ Rust ส่วน C ใช้ Syntax แบบภาษาระดับระบบ`
`Semantics / Behavior: Rust Compile ผ่าน rustc ส่วน C ใช้ Compiler เช่น GCC`
`Type System: ทั้งสองภาษาเป็น Static Typing`
`Memory Management: Rust ใช้ Ownership และ Borrowing ส่วน C สามารถจัดการ Memory โดยตรงด้วย malloc() และ free()`
`afety: Rust มีระบบตรวจสอบ Memory Safety จาก Compiler ส่วน C มีความเสี่ยงจากการจัดการ Memory โดยตรง`

10.3 Rust vs. C++
`Syntax: Rust มี Syntax ที่เน้นความชัดเจน ส่วน C++ มีคุณสมบัติและ Syntax ที่หลากหลาย`
`Semantics / Behavior: Rust ใช้ rustc และ Cargo ส่วน C++ ใช้ Compiler เช่น GCC หรือ Clang`
`Type System: ทั้งสองภาษาเป็น Static Typing`
`Memory Management: Rust ใช้ Ownership และ Borrowing ส่วน C++ ใช้ RAII และ Smart Pointer`
`Safety: Rust ตรวจสอบปัญหา Memory หลายประเภทตอน Compile ส่วน C++ ยังเปิดให้จัดการ Pointer และ Memory ได้อย่างอิสระ`

10.4 Rust vs. Java
`Syntax: Rust ใช้ fn และ let ส่วน Java เน้น Class และ Object`
`Semantics / Behavior: Rust Compile เป็น Native Code ส่วน Java Compile เป็น Bytecode และทำงานผ่าน JVM`
`Type System: ทั้งสองภาษาเป็น Static Typing`
`Memory Management: Rust ใช้ Ownership ส่วน Java ใช้ Garbage Collection`
`Safety: Rust เน้น Memory Safety ตั้งแต่ Compile Time ส่วน Java จัดการ Memory อัตโนมัติ`

10.5 Rust vs. Kotlin
`Syntax: Rust ใช้ fn และ let ส่วน Kotlin ใช้ fun และ val / var`
`Semantics / Behavior: Rust ใช้ rustc และ Cargo ส่วน Kotlin ทำงานบน JVM เป็นหลัก`
`Type System: ทั้งสองภาษาเป็น Static Typing`
`Memory Management: Rust ใช้ Ownership ส่วน Kotlin ใช้ Garbage Collection`
`Safety: Rust เน้น Memory Safety ผ่าน Compiler ส่วน Kotlin มี Null Safety และการจัดการ Memory อัตโนมัติ`

10.6 Rust vs. Ruby
`Syntax: Rust มี Syntax ที่เป็นระบบมากกว่า ส่วน Ruby เน้นความกระชับและอ่านง่าย`
`Semantics / Behavior: Rust Compile ก่อน Run ส่วน Ruby ทำงานผ่าน Ruby Runtime`
`Type System: Rust เป็น Static Typing ส่วน Ruby เป็น Dynamic Typing`
`Memory Management: Rust ใช้ Ownership และ Borrowing ส่วน Ruby ใช้ Garbage Collection`
`Safety: Rust ตรวจสอบข้อผิดพลาดหลายอย่างก่อน Run ส่วน Ruby เน้นความง่ายและจัดการ Memory อัตโนมัติ`

### Rust Example

```rust
fn main() {
    let name = "Rust";
    println!("Hello, {}!", name);
}
```

### Python Example

```python
name = "Python"
print(f"Hello, {name}!")
```

### C Example

```python
name = "Python"
print(f"Hello, {name}!")
```

### C++ Example

```python
name = "Python"
print(f"Hello, {name}!")
```

### Java Example

```python
name = "Python"
print(f"Hello, {name}!")
```

### Kotlin Example

```python
name = "Python"
print(f"Hello, {name}!")
```

### Ruby Example

```python
name = "Python"
print(f"Hello, {name}!")
```
### Analysis

`Rust เน้น Safety, Reliability และ Performance โดยใช้ Static Typing, Ownership และ Compiler Checking ขณะที่ Python และ Ruby เน้นความง่ายในการเขียน, C และ C++ เน้นการควบคุมระบบและประสิทธิภาพ และ Java กับ Kotlin เน้นการทำงานบน JVM และการจัดการ Memory อัตโนมัติ`

---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member | Responsibility | Time |
|---|---|---:|
| Member 1 | Concept + Short Code Illustration | 5 min |
| Member 2 | Detailed Code + Live Demo | 5 min |
| Member 3 | Rust vs Other Language + PPL Analysis | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

`รับผิดชอบสรุปแนวคิดหลักของ Rust Environment, Cargo และ First Program พร้อมจัดทำตัวอย่างโค้ดสั้น ๆ เพื่อให้เข้าใจการเขียนและรันโปรแกรม Rust เบื้องต้น`

**Member 2**

`รับผิดชอบเขียนโค้ด Rust แบบละเอียด อธิบายโครงสร้างและการทำงานของโปรแกรม พร้อมสาธิตการสร้างโปรเจกต์และรันโปรแกรมจริงด้วย Cargo`

**Member 3**

`รับผิดชอบเปรียบเทียบภาษา Rust กับภาษาอื่น เช่น C/C++ และวิเคราะห์แนวคิดของภาษาโปรแกรม (PPL) ในด้าน Syntax, Type System, Memory Management, Compilation และ Performance`

**Member 4**

`รับผิดชอบจัดทำแบบฝึกหัดเกี่ยวกับ Rust และ Cargo พร้อมยกตัวอย่างข้อผิดพลาดที่พบบ่อย วิธีแก้ไข และคำถามท้าทายเพื่อทดสอบความเข้าใจของผู้เรียน`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[Official documentation ที่เกี่ยวข้องกับ Topic]`
4. `https://doc.rust-lang.org/book/ch01-03-hello-cargo.html`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `ChatGPT` | `ใช้เพื่อตรวจสอบความถูกต้องของข้อมูล เพื่อคอนเฟิร์มข้อมูลให้ถูกต้อง` | `ตรวจสอบโดยการเปรียบเทียบกับแหล่งข้อมูลของที่ Ai หามาด้วยอีกที` |
| `Claude.ai` | `ใช้เป็นตัวช่วยตรวจสอบอีกชั้น เพื่อเช็กความถูกต้องของข้อมูลและโค้ดอย่างละเอียด` | `เปรียบเทียบคำตอบกับ ChatGPT และทดสอบโค้ดก่อนนำเสนอ` |

### Declaration

- [ ] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [ ] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [ ] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[อธิบายว่าใช้ AI ในขั้นตอนใด และสมาชิกตรวจสอบผลลัพธ์อย่างไร]`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `0` | `0` | `0` | `0` | `เขียน Concept + Short Code Illustration และจัดการ Repository รวบรวมงานกลุ่ม` |
| Member 2 | `0` | `0` | `0` | `0` | `[รายละเอียด]` |
| Member 3 | `0` | `0` | `0` | `0` | `[รายละเอียด]` |
| Member 4 | `0` | `0` | `1` | `0` | `[รายละเอียด]` |

### Teamwork Reflection

**How did your team collaborate?**

`กลุ่มของเราแบ่งหน้าที่กันรับผิดชอบตามหัวข้อย่อย และทำงานร่วมกันผ่าน GitHub โดยเริ่มต้นจากให้ตัวแทนกลุ่ม ทำการ Fork Repository หลักของรายวิชามาไว้ที่บัญชีของตนเอง จากนั้นได้ทำการเชิญ (Invite) สมาชิกคนอื่นๆ ในกลุ่มเข้ามาเป็น Collaborator ใน Repository นั้น เพื่อให้ทุกคนสามารถเข้ามาแก้ไขไฟล์และกด Commit โค้ดลงใน Branch "main" ร่วมกันได้โดยตรง ทำให้สามารถรวบรวมงานทั้งหมดไว้ในที่เดียวกันได้อย่างเป็นระบบ`

**Problems encountered**

`ในช่วงแรก กลุ่มของเราพบปัญหาเรื่องความเข้าใจในการทำ Pull Request (PR) และการจัดการ Branch โดยสมาชิกมีการแก้ไขไฟล์และ Commit แยกกันไปคนละ Branch (เช่น patch-1, patch-4) และต่างคนต่างเปิด PR ซ้อนกัน ทำให้โค้ดของสมาชิกแต่ละคนไม่มารวมอยู่ใน PR เดียวกัน นอกจากนี้ยังพบปัญหาความสับสนในการเลือกเป้าหมาย (Base/Compare repository) ในการรวมไฟล์ ทำให้หน้าต่างเปรียบเทียบไม่แสดงผลอัปเดตล่าสุด`

**How did you solve them?**

`เราแก้ปัญหาโดยการปรับโครงสร้างการทำงานใหม่ทั้งหมด โดยเข้าไปตั้งค่าในเมนู Settings > Collaborators เพื่อมอบสิทธิ์ให้เพื่อนทุกคนสามารถเข้าถึง Repository ของตัวแทนกลุ่มได้โดยตรง เมื่อทุกคนกดยอมรับคำเชิญแล้ว เราได้ตกลงกันให้สมาชิกทุกคนทำการแก้ไขและ Commit งานลงใน Branch "main" เพียงที่เดียว เมื่อไฟล์งานของทุกคนรวมกันเสร็จสมบูรณ์แล้ว ตัวแทนกลุ่มจึงทำการเปิด Pull Request ใหม่ที่ดึงข้อมูลจาก Branch "main" ของกลุ่ม ส่งไปยัง Branch "main" ของอาจารย์ ทำให้สามารถรวบรวมงานเป็นชิ้นเดียวได้สำเร็จ`

---

## 15. Final Checklist

- [ ] Learning Objectives ครบ 3–4 ข้อ
- [ ] Key Concepts ครบถ้วน
- [ ] Syntax / Rules
- [ ] Runnable Code Examples
- [ ] Code Compile และ Run ได้จริง
- [ ] Common Mistakes
- [ ] Exercises 2 ข้อ พร้อม Solutions
- [ ] PPL Perspective
- [ ] Rust vs Other Language
- [ ] References อย่างน้อย 4 แหล่ง
- [ ] AI Usage Declaration
- [ ] GitHub Contribution
- [ ] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [ ] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `[GitHub repository URL]`

**Chapter Path:** `[เช่น chapters/01-introduction/]`

**Final PR:** `#21`

**Submitted by:** `[Group 02]`

**Date:** `[2036-09-30]`

*โครงสร้างเอกสารฉบับเต็ม (Key Concepts, Runnable Code Examples, Common Mistakes, Exercises, PPL Perspective, Rust vs Other Language, References, AI Usage Declaration, GitHub Contribution, Final Checklist) ให้ทำต่อจากจุดนี้ตาม Template หลักของวิชา (`rust_tutorial_template.md`) ที่แนบมากับใบมอบหมายงาน*
