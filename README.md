# rust-connection-pool-api
Rust project for database connection pool maintaining a number of open connections to a database and providing in a Functions as a Service (FaaS) model to build applications deployed as “serverless” chunks of code that run completely on the cloud provider’s platform computing infrastructure. With a FaaS platform, you don’t have to manage any server infrastructure and only pay for the computing cycles required to execute the function.

## APIs
- POST `/api/files/` Upload a file
- GET `/api/files/{name}/` Download a file with a name

## Credit
#### Rust Documents & Tutorials:
[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
[Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
[Functional Programming in Rust](https://mmstick.gitbooks.io/rust-programming-phoronix-reader-how-to/content/chapter02.html)
[Calling a Web API](https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html)

#### Examples
- [A Web App in Rust - 13 Connection Pooling](https://dev.to/krowemoh/a-web-app-in-rust-13-connection-pooling-3c9c) by **Nivethan**
- [Practical Rust Web Development - Connection Pool](https://dev.to/werner/practical-rust-web-development-connection-pool-46f4) by **Werner Echezuría**

#### Starter References:
- [x] [Build Your First REST API in Rust Language Using Actix Framework](https://medium.com/swlh/build-your-first-rest-api-using-rust-language-and-actix-framework-8f827175b30f) by **Mehrdad Esmaeilpour** in **Medium.com**.
- [x] [Separating Modules into Different Files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)


#### Actix-Web Middleware
- [Actix Documentation - Middleware (Advance)](https://actix.rs/docs/middleware/)
- [Actix Web JWT Example](https://github.com/emreyalvac/actix-web-jwt/blob/master/src/middlewares/auth.rs)
- [How to return an early response from an actix-web middleware?](https://stackoverflow.com/questions/57892819/how-to-return-an-early-response-from-an-actix-web-middleware)

#### JWT
- [Build an API in Rust with JWT Authentication](https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/)
- 

## Actix Examples
#### Community Showcase

- [Triox](https://github.com/AaronErhardt/Triox) : A free file hosting server that focuses on speed, reliability and security.
- [binserve](https://github.com/mufeedvh/binserve) : A fast, secure, and easy to set up static web server written on top of Actix Web with routing, templating, and various other features.
- [Roseline](https://github.com/DoumanAsh/roseline.rs) : A personal web site and discord & IRC bot to access simple SQLite database. Demonstrates usage of various Actix and Actix Web concepts.
- [lemmy](https://github.com/dessalines/lemmy) : A federated alternative to reddit in Rust.
- [MeiliSearch](https://github.com/meilisearch/MeiliSearch): Fast, Relevant and Typo-Tolerant Search Engine. Open source alternative to Algolia.
- [Dalted](https://github.com/carrascomj/dalted): Simple webapp that showcases the integration of [image-rs](https://github.com/image-rs/image) with Actix Web for color blindness simulations.

#### Community Articles, Example Apps, Starters & Boilerplate Projects

- [Jelly Starter](https://github.com/secretkeysio/jelly-actix-web-starter) : A starter template for actix-web projects that feels very Django-esque. Avoid the boring stuff and move faster.
- [Actix and SQLx User CRUD for MySQL](https://github.com/jamesjmeyer210/actix_sqlx_mysql_user_crud) : A User CRUD showcasing MySQL database interaction with full integration test coverage, designed to fit comfortably in a system of micro-services.
- [webapp.rs](https://github.com/saschagrunert/webapp.rs) : A web application completely written in Rust.
- [RealWorld Example App](https://github.com/fairingrey/actix-realworld-example-app) : Implementation of the RealWorld backend API spec in Actix.
- [Canduma](https://github.com/clifinger/canduma) : Rust authentication server boilerplate
- [Rust, Docker & GraphQL](https://github.com/jayy-lmao/rust-graphql-docker): An example of using Dataloaders, context, and a minimal docker container.
- [Complete Actix 2.x REST Server](https://github.com/ddimaria/rust-actix-example): Actix 2.x HTTP Server featuring multi-database support, auth/JWTs, caching, static files, app state, tests, coverage, and docker.
- [Actix Server Authentication with JWT and MongoDB](https://github.com/emreyalvac/actix-web-jwt/) : An implementation of JWT in Actix.
- [Production-Grade Logging in Rust Applications](https://medium.com/better-programming/production-grade-logging-in-rust-applications-2c7fffd108a6) : An article showcasing the use of [tracing](https://github.com/tokio-rs/tracing) in an Actix application

## Bookmarks
- [x] [A Basic Web Application with Rust and Actix-web](https://www.zupzup.org/rust-webapp/index.html)
- [ ] [Web Development with Rust — 03/x: Create a REST API](https://dev.to/gruberb/web-development-with-rust-03-x-create-a-rest-api-3i82) by **Bastian Gruber**
- [ ] [Creating a REST API in Rust with warp](https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/) by **Bastian Gruber**
- [ ] [How to Build a REST API in Rust — A Step-by-Step Guide](https://betterprogramming.pub/rest-api-in-rust-step-by-step-guide-b8a6c5fcbff0) by **Asel Siriwardena** (Rocket & Diesel Frameworks)
- [ ] [Swagger Pet Store with Rust](https://medium.com/@pvinchon/swagger-pet-store-with-rust-8a8846868b42) by **Philippe**
- [ ] [Rust: Project structure example step by step](https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee)
- [ ] [Closures: Anonymous Functions that Can Capture Their Environment](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [ ] [Rust By Example - Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)
