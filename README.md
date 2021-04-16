# rust-connection-pool-api
Rust project for database connection pool maintaining a number of open connections to a database and providing in a Functions as a Service (FaaS) model to build applications deployed as “serverless” chunks of code that run completely on the cloud provider’s platform computing infrastructure. With a FaaS platform, you don’t have to manage any server infrastructure and only pay for the computing cycles required to execute the function.

## APIs
- POST `/api/files/` Upload a file
- GET `/api/files/{name}/` Download a file with a name

## Credit

[Build Your First REST API in Rust Language Using Actix Framework](https://medium.com/swlh/build-your-first-rest-api-using-rust-language-and-actix-framework-8f827175b30f) by **Mehrdad Esmaeilpour** in **Medium.com**.

#### Rust Documents & Tutorials:
[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
[Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
###Crate [std](https://doc.rust-lang.org/std/index.html)
[Functional Programming in Rust](https://mmstick.gitbooks.io/rust-programming-phoronix-reader-how-to/content/chapter02.html)

#### Other references:
- [x] [Separating Modules into Different Files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)
- [x] [Move struct into a separate file without splitting into a separate module?](https://stackoverflow.com/questions/28010796/move-struct-into-a-separate-file-without-splitting-into-a-separate-module)

#### Bookmarks
- [ ] [Web Development with Rust — 03/x: Create a REST API](https://dev.to/gruberb/web-development-with-rust-03-x-create-a-rest-api-3i82) by **Bastian Gruber**
- [ ] [Creating a REST API in Rust with warp](https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/) by **Bastian Gruber**
- [How to Build a REST API in Rust — A Step-by-Step Guide](https://betterprogramming.pub/rest-api-in-rust-step-by-step-guide-b8a6c5fcbff0) by **Asel Siriwardena** (Rocket & Diesel Frameworks)
- [ ] [Swagger Pet Store with Rust](https://medium.com/@pvinchon/swagger-pet-store-with-rust-8a8846868b42) by **Philippe**
- [ ] [Rust: Project structure example step by step](https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee)
- [ ] [Closures: Anonymous Functions that Can Capture Their Environment](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [ ] [Rust By Example - Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)
