# **INTRODUCTION**

Implementation of request algorithms (FCFS, SSTF, SCAN, C-SCAN, EDF, FD-SCAN) which are used by HDD to handle incoming requests in a most efficient way.
Simulation was coded in the **Rust** programming language. To run the the simulation you must have **Rust** installed on your device.
You can find out how to install/update **Rust** here: https://www.rust-lang.org/tools/install

# **Usage:**

1. At the top of the main() function there's a variable *path* which is by default set to be "data_rand.txt". There are 2 other files
generated for testing purposes: "data_asc.txt" (values inside the file are ascending), "data_desc.txt" (values inside the file are
descending). Simply change the variable to open these files (or even your own file!).
2. If you wish to rerandomize the sequence located in "data_rand.txt" follow these steps:
   1. Uncomment line 11, the variable *v* will be then set to vector of processes returned by *generate_processes()* method.
   2. Argument passed to *generate_processes()* method defines how many processes will be generated.
3. Results are always saved to the "results.txt" file.
4. In terminal be sure you're inside the *cpu_scheduling_rust* directory. You can run the program using **cargo run --release** in your terminal (--release flag is optional but it will significantly boost the performance of the simulation).
