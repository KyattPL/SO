# **INTRODUCTION**

Implementation of page replacement algorithms (FIFO, OPT, LRU, Second chance LRU, RANDOM) which are used by OS to implement paging for virtual memory management.
Simulation was coded in the **Rust** programming language. To run the the simulation you must have **Rust** installed on your device.
You can find out how to install/update **Rust** here: https://www.rust-lang.org/tools/install

# **Usage:**

1. At the top of the "main.rs" file there are a couple of constants that can be tweaked to run the simulation in different circumstances.
2. Opening certain files and rerandomizing requests are done via internal I/O mechanism so there are no changes in code required to do that!
3. Results are printed to the console. (TODO: save them in seperate "results.txt file)
4. In terminal be sure you're inside the *page_replacement_rust* directory. You can run the program using **cargo run --release** in your terminal (--release flag is optional but it will significantly boost the performance of the simulation).
