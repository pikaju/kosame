<div align="center">
  <img width="256" src="./misc/readme/logo-white.svg#gh-dark-mode-only" />
  <img width="256" src="./misc/readme/logo-black.svg#gh-light-mode-only" />
</div>

<br />

<div align="center">
  <h3>Macro-based Rust ORM focused on developer ergonomics</h3> 
</div>

Kosame (小雨, Japanese for "light rain" or "drizzle") is a Rust ORM inspired by [Prisma](https://github.com/prisma/prisma) and [Drizzle](https://github.com/drizzle-team/drizzle-orm).
Most Rust ORMs ask the developer to write both the query they want to perform as well as the resulting struct type to store the query rows in, even though they are tightly coupled. Some TypeScript ORMs manage to solve this by inferring the row types from the query itself. They also offer relational queries, allowing developers to go from flat tables to a nested struct hirarchy. Kosame was born out of a desire to have this level of developer ergonomics in Rust, using macro magic.

Kosame requires no active database connection during development and has no build step. Despite this, Kosame offers strong typing and rust-analyzer auto-completions.

**Kosame is currently a prototype and not recommended for production use.**

## Showcase


