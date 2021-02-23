# Data-Driven Object Management

* Start Date: 2021-02-23
* RFC Pull Request: XXX
* Tracking Issue: XXX

## Summary

Manage data-driven game objects in an easy and scalable manner.

## Motivation

Data-driven game objects are often generated from data living outside of the
game's source code.

One such example is [Cataclysm: DDA data files][cdda].

The biggest upside to such an approach is that adding/manipulating objects does
not require the game code to be modified.

However, as the game (and community) grows, the number of data files (or lines
in monolithic data packs) grows rapidly. This causes scalability problems while
maintaining the data files, especially when wanting to make far-reaching data
layout changes.

This RFC proposes a way to solve this problem, by guaranteeing the following
properties of our data management:

1. Object data lives outside of the game source code.
2. New or modified object data can be easily reviewed through Pull Requests.
3. Making data model changes is fast and easy.
4. Data model changes are tracked through Git history.

## Reference-Level Explanation

The RFC proposes a three-step process to data management:

1. Data is stored in an SQLite database.
2. Data text-files are automatically generated from the database.
3. Git repository includes both the database, and generated text-files.

### Data Scaling

To allow for scaling of data management, all data is stored in a proper
relational database.

This has obvious advantages:

1. There are many database query applications/utilities that allow you to query
   for the data you are interested in.
2. Making data layout changes can be done through regular SQL queries.
3. Through the concept of _migration files_, data migrations are easily
   reversible.

However, it also has disadvantages:

1. Storing the data in Git can come with unnecessary repository size bloat.
2. Viewing the data often requires special tooling (see advantage).
3. Git diffs are impossible to dissect.

### SQLite Database

By using SQLite, we mostly alleviate the first problem, in that its data is
contained to a single file, and doesn't contain too much added data such as
caches, etc.

### Generated Text Files

To make all data accessible outside of the repository, we'll include text-based
copies of all data in a folder structure.

For example, given a "critters" table with a column named "name", we'll create
`data/critters/<name>.<format>` files for easy access during review and cursory
data explorations.

The data format of the files is to-be-determined. It might be JSON, or it might
be a different format (for example, one that allows us to include comments).

All text files will be marked with a "auto-generated, do not change" comment at
the top, as to guide new contributors who want to change some piece of data.

## Reference-Level Explanation

### Directory Structure

We'll start by adding a new directory structure:

```
.
└─ data
   ├── README.md
   ├── database.sqlite3
   ├── generated
   └── migrations
       └── archive
```

The `README.md` file will contain all the relevant details about our data
management system and how people can read/contribute to it.

The `database.sqlite3` file is the canonical source of all the data objects used
in the game.

The `generated` directory contains text-based files of the data stored in the
database. This is mostly used for review purposes, but also allows visitors of
the repository to get an idea of the data used in the game, and allows using
external tools to query the files without having to open a database CLI or GUI.

The `migrations` directory contains a list of all migrations since the start of
the project. Since the list of migrations will likely grow, we'll also add an
`archive` sub-directory to which we move historical migrations that aren't as
relevant to readers anymore. These are still used when doing the actual
migrations, though.

### Database Layout

The exact layout of the database is to-be-determined, but in general we'll use
common relational-database best-practices, including unique row identifiers,
relationships, data normalization, etc.

### Generating Data Files

To generate data files from the database, we'll use a small Rust utility. This
utility uses the `sqlx` library to read the SQLite database and uses `serde` to
serialize the data into the relevant files.

We'll also have a separate `exodus-data` library which contains the structs used
to create instances of the data objects. This allows us to use that library both
in the game itself when loading the data, but also in the utility that generates
the data files from the database.

Generating the actual data files is hooked up into a `build.rs` file, which
detects any changes to the `database.sqlite3` file, and builds the data files
from scratch when needed. This allows us to add the data management system
without any overhead to development workflows, or having to introduce
complicated Makefile rules.

### Reading Data From Game

The exact details on how the game will read the data files is outside of the
scope of this RFC.

However, the initial idea is for the game code itself to read the generated data
files, _not_ the SQLite database directly, as that allows us to remove any
database-specific dependencies inside the game. This approach might change
however, as there could be a reason to store save game data in an SQLite
database as well, but this choice can be deferred until later.

## Drawbacks

- We're required to maintain a database of data objects. This would be needed
  regardless, but the other approach would be to manage the static data files
  directly, instead of indirectly through the database.

- We have to commit duplicate data in the repository. The database and the
  static files contain the same data, just in a different representation. This
  results in being able to use the right tool for the job (database for
  migrations and source of truth, static files for Git diff/review/exploration
  purposes), at the expense of an increased repository size footprint.


## Rationale And Alternatives

Using _only_ static files is not a viable solution, as it doesn't scale well.
Using an existing AAA game studio asset management system is too complex for our
use-case. This RFC strikes a middle-ground in allowing for using SQL queries to
manipulate our data, while still keeping the set-up/overhead as low as possible.

## Prior Art

TODO

## Unresolved Questions

None

## Future Possibilities

By having a database of data objects, we can use this data to generate other
useful resources. For example, we could have a wiki page that is always 100%
up-to-date with the latest game state.

Aside from _reading_ the data, we can also change how we _write_ the data. That
is, we could add a web-page that allows people to design/build their own objects
(critters, items, etc.) and have a back-end system automatically create a Pull
Request with the updated database and static files.
