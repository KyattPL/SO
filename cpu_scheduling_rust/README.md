# **INTRODUCTION**

Implementation of queuing algorithms (FCFS, SJF-preemptive, RR) which are used by a CPU to handle incoming requests in a most efficient way.
Simulation was coded in the **Rust** programming language. To run the the simulation you must have **Rust** installed on your device.
You can find out how to install/update **Rust** here: https://www.rust-lang.org/tools/install

# **Usage:**

1. At the top of the main() function there's a variable *path* which is by default set to be "data_rand.txt". There are 2 other files
generated for testing purposes: "data_asc.txt" (values inside the file are ascending), "data_desc.txt" (values inside the file are
descending). Simply change the variable to open these files (or even your own file!).
2. If you wish to rerandomize the sequence located in "data_rand.txt" follow these steps:
   1. Comment out lines no: 10, 14, 23, 31 inside the main() function.
   2. Uncomment line 11, the variable *v* will be then set to vector of processes returned by *generate_processes()* method.
   Argument passed to *generate_processes()* method defines how many processes will be generated.
3. Result are always saved to the "results.txt" file.
