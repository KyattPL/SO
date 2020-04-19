# **INTRODUCTION**

Implementation of request algorithms (FCFS, SSTF, SCAN, C-SCAN, EDF, FD-SCAN) which are used by HDD to handle incoming requests in a most efficient way.
Simulation was coded in the **Rust** programming language. To run the the simulation you must have **Rust** installed on your device.
You can find out how to install/update **Rust** here: https://www.rust-lang.org/tools/install

# **Usage:**

1. At the top of the "main.rs" file there are a couple of constants that can be tweaked to run the simulation in different circumstances.
2. Opening certain files and rerandomizing requests are done via internal I/O mechanism so there are no changes in code required to do that!
3. Results are always saved to the "results.txt" file.
4. In terminal be sure you're inside the *disk_scheduling_rust* directory. You can run the program using **cargo run --release** in your terminal (--release flag is optional but it will significantly boost the performance of the simulation).
