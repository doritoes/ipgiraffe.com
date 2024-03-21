# Next Steps
Congratulations on completing this exercise! You have learned and demonstrated some new cloud skills.

I recommend you consider the following to expand on your new knowlege. Make some predictions, do the research, then test it to see for yourself.

## Standardize on One Method
Right now we are incurring the cost of CloudFront, while we have demonstrated that Route 53 doesn't need it when we use Regional API Gateways.

Consider converting to eliminate CloudFront
- This allows us to use all 3 of our regions, not just 2 with CloudFront
- We could have multiple CloudFront instances to to use all 3 regions, but at a cose
- Currently health checks are turned off in Route 53 for our API gateways. What is the effect of this? What cost concerns are there if we turn health checks on?

## Look into arm64 vs x86_64 Architecture
What are the cost implications?
- ARM64 (Graviton2) - cheaper but less x86-specific optimizations
  - how much cheaper?
  - does your code need modifications to run on ARM64?
- x86_64 - compatibility

## Try Different Runtimes
- Nodejs - [Instructions for Node.js](Create 2_Lambda_Function_Node.md)
- Java
- Ruby - [Instructions for Ruby](2_Lambda_Function_Ruby.md)
- .Net - [Instructions for .NET 8 C#](2_Lambda_Function_DotNet.md)
- Amazon Linux with
  - Go - [Instructions for Go](2_Lambda_Function_Go.md)
  - Rust
  - C++
  - custom

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

## Compare the cost of AWS Lamba versus other technologies
- Heroku - https://www.rdegges.com/2018/to-30-billion-and-beyond/
- AWS Fargate https://www.cloudzero.com/blog/fargate-vs-lambda/
- https://www.pricekite.io/
