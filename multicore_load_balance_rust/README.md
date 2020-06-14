# **INTRODUCTION**

Implementation of load balancing algorithms for multicore systems which are used to distribute upcoming processes to the CPU's in an efficient manner.
Simulation was coded in the **Rust** programming language. To run the the simulation you must have **Rust** installed on your device.
You can find out how to install/update **Rust** here: https://www.rust-lang.org/tools/install

# **Usage:**

1. At the top of the "main.rs" file there are a couple of constants that can be tweaked to run the simulation in different circumstances.
2. Results are printed to the console. (TODO: save them in seperate "results.txt file)
3. In terminal be sure you're inside the *multicore_load_balance_rust* directory. You can run the program using **cargo run --release** in your terminal (--release flag is optional but it will significantly boost the performance of the simulation).
