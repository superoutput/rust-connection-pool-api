# rust-connection-pool-api
Rust project for database connection pool maintaining a number of open connections to a database and providing in a Functions as a Service (FaaS) model to build applications deployed as “serverless” chunks of code that run completely on the cloud provider’s platform computing infrastructure. With a FaaS platform, you don’t have to manage any server infrastructure and only pay for the computing cycles required to execute the function.

## APIs
- POST `/api/files/` Upload a file
- GET `/api/files/{name}/` Download a file with a name

## Credit

The examples on this publication are guided by [Build Your First REST API in Rust Language Using Actix Framework](https://medium.com/swlh/build-your-first-rest-api-using-rust-language-and-actix-framework-8f827175b30f) by **Mehrdad Esmaeilpour** in **Medium.com**.

Rust Document Guide:

[Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
###Crate [std](https://doc.rust-lang.org/std/index.html)
[Functional Programming in Rust](https://mmstick.gitbooks.io/rust-programming-phoronix-reader-how-to/content/chapter02.html)

Other references:

- [Web Development with Rust — 03/x: Create a REST API](https://dev.to/gruberb/web-development-with-rust-03-x-create-a-rest-api-3i82) by **Bastian Gruber**
- [Creating a REST API in Rust with warp](https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/) by **Bastian Gruber**
- [How to Build a REST API in Rust — A Step-by-Step Guide](https://betterprogramming.pub/rest-api-in-rust-step-by-step-guide-b8a6c5fcbff0) by **Asel Siriwardena**
- [Swagger Pet Store with Rust](https://medium.com/@pvinchon/swagger-pet-store-with-rust-8a8846868b42) by **Philippe**