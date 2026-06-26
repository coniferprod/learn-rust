# Learn Rust Programming Today

This is the source code repository for the example programs
in the book [Learn Rust Programming Today](https://www.coniferproductions.com/books/learn-rust/).

## Chapter notes

### Chapter 31: Reading events from a web service

The web server for events is live on Heroku at [https://todayserver-89bb2a1b2e80.herokuapp.com/api/v1/events](https://todayserver-89bb2a1b2e80.herokuapp.com/api/v1/events). You need to supply the
`date` query parameter as `date=06-16` (MM-DD format).

Note that it mostly contains fake test events generated with ChatGPT. They all have the
category `test/fake`, so you can either turn the web event provider off with `is_active = false`
in the configuration file (see Chapter 41), implement category exclusion (see Chapter 42),
or just use `grep` to filter them:

    today | grep -v "test/fake"

## Chapter key

The following table lets you easily look up the directories in this repository
containing the examples in the book, by the chapter number.

| Chapter | Directory |
| ------- | --------- |
| 3 | ch-compiler |
| 5 | ch-cargo |
| 6 | ch-strings-numbers |
| 7 | ch-tuples-arrays |
| 8 | ch-iter-loop |
| 9 | ch-if |
| 10 | ch-functions |
| 11 | ch-structs |
| 12 | ch-enum-match |
| 13 | ch-option |
| 14 | ch-ref |
| 15 | ch-vectors |
| 16 | ch-args |
| 17 | ch-errors |
| 18 | ch-traits |
| 19 | ch-crates |
| 20 | ch-testing |
| 22 | ch-birthday |
| 23 | ch-today |
| 24 | ch-modules |
| 25 | ch-providers |
| 26 | ch-textfile |
| 27 | ch-csvfile |
| 28 | ch-dirs |
| 29 | ch-config |
| 30 | ch-sqlite |
| 31 | ch-web |
| 32 | ch-filters |
| 33 | ch-cli | 
| 34 | ch-event-kinds |
| 35 | ch-logging |
| 36 | ch-add-events |
| 37 | ch-sorting |
| 38 | ch-today1 |
| 39 | ch-refactoring |
| 40 | ch-xml |
| 41 | ch-final |
