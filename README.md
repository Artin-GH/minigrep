# My Mini Grep
A better version of the minigrep project in the [Rust Book - Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) and my first project in `Rust`.


## Updated sections
1. Used iterator features instead of performing a for loop through the vector items collected from iterator.

2. Added abality to pass `--ignore-case` argument to search with case-insensitive behavior beside environment variable `IGNORE_CASE`.

3. If the `file_path` argument isn't passed, it now reads from the `stdin` and beside:
    ```zsh
    minigrep query poem.txt --ignore-case
    ```
    Can be run this way as well:
    ```zsh
    cat poem.txt | minigrep query --ignore-case
    ```
