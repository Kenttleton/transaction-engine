# transactions-engine
## Building and Running
Build the project using `cargo build`. To run the project `cargo run -- input.csv > output.csv`. Leaving off the `>` along with the filename will cause the program to print to the console. The `output.csv` file should match the records in `answer.csv`.

## Notes on Design and Implementation
Concurrency was not considered in this project for brevity sake. A better implementation would be a multithreaded TCP stream handling solution with a thread pool for managing thread lifetimes. This would allow for scaling and processing transaction records as they came in instead of a batch process using csv files. The use of queueing systems can be added as well to manage the in and out flow of data as part of a pipeline along with container orchestration. A solution like this would not satisfy the cli requirements for testing of this project. The current implementation will most likely be limited by the size of a single thread stack size (1 MB default on Windows and varies per linux distribution). Some care was taken to maintain short lifetimes of data to reduce the load on the system resources but with any solution may not be perfect. 

Correctness is handled fairly well. As long as the transactions are ordered chronologically this solution should produced the correct outcome based on the given specs. Sample data of input and output can be found in the root of this project in the `test.csv` and `final.csv` files respectively. If given more time unit testing should be added to ensure the correctness of function behavior. Strong typing was also used to ensure correctness. Errors are handled by consuming them, outputting them to the console, and exiting the program or exiting loop logic. No errors should break the execution of the program. 

Disputes on withdrawals do not seem to be part of the spec and therefore are not handled. The spec does not specify that disputes should only be on deposits either but the chargeback descriptions only refers to reversing a deposit and not returning funds from a disputed withdrawal. It is unclear whether this was intentional or accidental but should be noted. Some code was written to support such a scenario but was commented out to resolve unused warnings and left in the project for future use.

A more elaborate interface would help developers interact with this program. Originally the `clap` library was used to generate a help and any man page documentation to describe use of the program. However, this was forgone due to the automated testing requirements was removed for a more crude method to satisfy those requirements.

## Updates
Logic was added to compensate for the reversal of withdrawal transactions as mentioned above. All numbers on the account are treated as positive regardless of transaction under dispute. This means held funds are positive in value until resolved or chargedback regardless of if it was a deposit or withdrawal. Withdrawal, like deposit funds, are not counted in the available column until a resolution scenario occurs.

I also added a thread spawn mechanism to increase the memory size to 2 MB so thread memory is consistent cross-platform and is controlled by the `STACK_SIZE` variable. This would be the beginning of a multithreaded approach to handle concurrent streams with a thread pool to manage concurrent processes.  

Extra test cases were added to my test file to test all valid scenarios. A few bad formats were added to test logging and stability of the application.

Extra logging was added to handle errors more cleanly while still processing valid data. The log file in `/log/log.txt` will show any errors that occurred during any step of the process. Logging was thrown to a file to prevent the poisoning of the output with errors or bad data.
