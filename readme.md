# 🧠 Scheduling Algorithms

This repository contains implementations of **CPU and Disk Scheduling Algorithms** written in multiple programming languages.  
The primary focus is on **learning by implementation**, with Rust as the main language, but contributions in **C, C++, or any other language** are absolutely welcome.

---

## 🎯 Goal

The main goal of this repository is to:

- Understand how different scheduling algorithms work
- Implement them from scratch in a clear and educational way
- Compare approaches across different programming languages
- Serve as a reference for students and systems programming enthusiasts

This repository prioritizes **clarity and correctness over optimization**, making it beginner-friendly.

---

## 📌 Implemented Algorithms

### ✅ CPU Scheduling

#### 🟢 First Come, First Serve (FCFS)

**First Come, First Serve (FCFS)** is one of the simplest CPU scheduling algorithms.

- Processes are executed **in the order they arrive** in the ready queue
- Similar to customers standing in line — whoever comes first is served first
- It is a **non-preemptive** algorithm:
  - Once a process gets the CPU, it runs until completion or until it blocks for I/O
  - The CPU cannot be taken away by another process

**Key Characteristics:**
- Simple to understand and implement
- No starvation
- Can suffer from the *convoy effect*, where short processes wait behind long ones

---

## 🛠 Languages Used

- 🦀 **Rust** (primary)
- C
- C++
- Any other language contributors are comfortable with

Each algorithm implementation should aim to be:
- Easy to read
- Well-commented
- Faithful to the algorithm’s definition

---

## 🤝 Contributing

Contributions are highly encouraged! You can:

- Add new scheduling algorithms
- Implement existing algorithms in a different language
- Improve documentation or examples
- Optimize or refactor existing code (without sacrificing readability)