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

**คำอธิบาย**

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

`[Cargo คือ **Package Manager และ Build System ของ Rust** ใช้สำหรับสร้างโปรเจกต์ จัดการ Dependencies, Compile และ Run โปรแกรม ทำให้การพัฒนา Rust เป็นระบบมากขึ้น`

```rust
cargo new hello_rust
cd hello_rust
cargo run
```
`cargo new` สร้างโปรเจกต์ Rust ใหม่พร้อมโครงสร้างพื้นฐาน จากนั้น `cargo run` จะ Compile และรันโปรแกรมในโปรเจกต์นั้น
---

### 4.3 `[Concept 3]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

### 4.4 `[Concept 4 — ถ้ามี]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

### 4.5 `[Concept 5 — ถ้ามี]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---

## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |

### Important Rules

1. `[กฎสำคัญข้อที่ 1]`
2. `[กฎสำคัญข้อที่ 2]`
3. `[กฎสำคัญข้อที่ 3]`

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code ทีละส่วนที่สำคัญ]`

---

### Example 2 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code]`

---

## 7. Common Mistakes

### Mistake 1 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

### Mistake 2 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `คอมไพล์ผ่านหรือไม่? (Immutable Variable)`

**Problem**

`โค้ดต่อไปนี้คอมไพล์ผ่านหรือไม่ ถ้าผ่านผลลัพธ์คืออะไร ถ้าไม่ผ่านให้บอกว่าบรรทัดไหนผิดเพราะอะไร

fn main() {
    let x = 10;
    let mut y = 20;
    y = y + x;
    x = y - 5;
    println!("{} {}", x, y);
}
`

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

error[E0384]: cannot assign twice to immutable variable x

x ประกาศด้วย let x = 10; จึงเป็น immutable ไม่สามารถกำหนดค่าใหม่ได้
y ประกาศด้วย let mut y จึงแก้ค่าได้ บรรทัด y = y + x; ไม่มีปัญหา
Rust ตรวจจับข้อผิดพลาดนี้ตั้งแต่ขั้นคอมไพล์ ก่อนที่โปรแกรมจะรัน
ค่าหลังแก้: y = 20 + 10 = 30 แล้ว x = 30 - 5 = 25 จึงพิมพ์ 25 30`

---

### Exercise 2 — `ผลลัพธ์คืออะไร? (Cargo และ First Program)`

**Problem**

`โปรแกรมจะแสดงผลลัพธ์ใดออกมา

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

รันคำสั่ง
cargo run

`

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

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

`[Topic นี้เกี่ยวข้องกับ syntax อย่างไร]`

### 9.2 Semantics

`[คำสั่ง/construct เหล่านี้มีความหมายหรือพฤติกรรมอย่างไร]`

### 9.3 Type System

`[เกี่ยวข้องกับ type system อย่างไร ถ้ามี]`

### 9.4 Memory / Resource Management

`[เกี่ยวข้องกับ memory หรือ resource management อย่างไร ถ้ามี]`

### 9.5 Abstraction / Other PPL Concepts

`[อธิบาย abstraction, scope, binding, paradigm หรือแนวคิด PPL อื่นที่เกี่ยวข้อง]`

### 9.6 Why Rust?

`[Rust ใช้แนวคิดนี้เพื่อเพิ่ม safety, reliability หรือ performance อย่างไร]`

---

## 10. Rust vs. Other Language

**Comparison Language:** `[Python / C / C++ / Java / Kotlin / ...]`

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | `[อธิบาย]` | `[อธิบาย]` |
| Semantics / Behavior | `[อธิบาย]` | `[อธิบาย]` |
| Type System | `[อธิบาย]` | `[อธิบาย]` |
| Memory Management | `[อธิบาย]` | `[อธิบาย]` |
| Safety | `[อธิบาย]` | `[อธิบาย]` |

### Rust Example

```rust
// Rust code
```

### `[Other Language]` Example

```python
# Other language code
```

### Analysis

`[อธิบายความแตกต่างที่สำคัญ และเหตุผลด้านการออกแบบภาษา]`

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

`[สิ่งที่รับผิดชอบ]`

**Member 2**

`[สิ่งที่รับผิดชอบ]`

**Member 3**

`[สิ่งที่รับผิดชอบ]`

**Member 4**

`[สิ่งที่รับผิดชอบ]`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[Official documentation ที่เกี่ยวข้องกับ Topic]`
4. `[แหล่งอ้างอิงเพิ่มเติม]`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `[เช่น ChatGPT]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |
| `[AI tool]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |

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
| Member 1 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 2 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 3 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 4 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |

### Teamwork Reflection

**How did your team collaborate?**

`[อธิบายกระบวนการทำงานร่วมกัน]`

**Problems encountered**

`[ปัญหาที่พบ]`

**How did you solve them?**

`[วิธีแก้ปัญหา]`

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

**Final PR:** `#[PR number]`

**Submitted by:** `[Group XX]`

**Date:** `[YYYY-MM-DD]`

*โครงสร้างเอกสารฉบับเต็ม (Key Concepts, Runnable Code Examples, Common Mistakes, Exercises, PPL Perspective, Rust vs Other Language, References, AI Usage Declaration, GitHub Contribution, Final Checklist) ให้ทำต่อจากจุดนี้ตาม Template หลักของวิชา (`rust_tutorial_template.md`) ที่แนบมากับใบมอบหมายงาน*
