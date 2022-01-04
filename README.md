# mkscratch

Creates a directory named .scratch in your current working directory which will be ignored
by Git. Useful for keeping around miscellaneous files that you don't want to be
tracked/shared.

Some things you might want to keep in a .scratch directory:

-   A cheat sheet of useful Git commands that are maybe a little too complex to remember
-   A Docker build command you've had to spend 10 minutes figuring out one two many times
-   A JSON dump of some obscure API response that isn't well documented, to have for reference
-   A backup of your .env files in case something goes wrong, so that you don't have to hunt
    down/regenerate all of your keys and passwords
-   Maybe you want to write a super small proof-of-concept test program and keep it around
    to expand on later if it works
-   Bash scripts that you find useful but that might not be very generic, or you don't think
    others would find useful
-   A quick list of known-good IDs you can use when testing SQL queries against a database.
    Keep a couple IDs around for entries in each table that you can pull up quickly.

These are just a handful of things I've personally used .scratch directories for. The
basic idea is that if you have some piece of text you'd like to keep around for later, but
you don't know where to put it, just put it in .scratch so that you can move on and stay
focused.

### Installation

```bash
cargo install mkscratch
```

I'll probably come up with some way to install prebuilt binaries later, so you don't have
to have Rust/Cargo installed.

### Usage

```
➜  pwd
/Projects/example/
➜  ls -a
.           ..          .git        .github     .gitignore  Cargo.lock  Cargo.toml  README.md   src         target
➜  mkscratch
Created /Projects/example/.scratch
➜  ls -a
.           ..          .git        .github     .gitignore  .scratch    Cargo.lock  Cargo.toml  README.md   src         target
```

Now you have a .scratch directory to keep scratch files in!
