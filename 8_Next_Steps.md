# Next Steps
Congratuluations on completing this exercise! You have learned and demonstrated some new cloud skills.

I recommend you consider the following to expand on your new knowleged. Make some predictions, do the research, then test it to see for yourself.

## Look in to arm64 vs x86_64 Architecture
What are the cost implications?
- ARM64 (Graviton2) - cheaper but less x86-specific optimizations
  - how much cheaper?
  - does your code ned modifications to run on ARM64?
- x86_64 - compatibility

## Try Different Runtimes
- Nodejs
- Java
- Ruby
- .Net
- Amazon Linux with Go, Rust, C++, custom

Compare the execution time and the memory used. Since cost is based on ms multiplied by memory used, which is the least expensive?

Which runtime has the fastest cold start? (cold start time is included in the billed time)
- Compiled languages are fast because of smaller binaries. Compiled code executes directly without the need for a runtime interpreter to be initialized
  - Rust (small and efficient binaries)
  - Go (exceptionally suited for concurrency)
  - Java (with optimizations like GraalVM, not the default)
  - C#
- Mid-tier are generally good performers; your project structure can affect initialization times
  - Node.js
  - Python
- Slowest are interpreted languages that have multiple dependencies and Java
  - Java (JVM initialization, garbage collection)
  - Ruby
