# cp-tool

Competitive programming tool to submit your sources. 
It was inspired by [cf-tool](https://github.com/xalanq/cf-tool), but that one works only with Codeforces.

## Features (TODO)

- [x] Stores config in project-related directory
- [x] Guesses `problem_id` from either filename or directory name
- Works with various judge systems
  - [x] ejudge
  - [ ] CodeForces
  - [ ] Yandex.Contest
  - [ ] AtCoder
  - [ ] TopCoder
  - [ ] Algotester
  - [ ] DDOTS
- [ ] Current contest status
- [ ] Current standings
- [ ] Submit response feature

## Usage

It's quite hard to choose how to store your sources: 
```plain
your-contest
├── a.cpp
├── b.cpp
└—— c.py

or

your contest
├── a
│   ├── a.cpp
│   ├── CMakeLists.txt
│   ├── ans1.txt
│   └── in1.txt
├── b
│   ├── b.cpp
│   ├── CMakeLists.txt
│   ├── ans1.txt
│   └── in1.txt
└── c
    ├── source.py
    ├── gen.py
    ├── ans1.txt
    └── in1.txt
```
However, `cpt` doesn't make any assumptions about your project sturcture and scans all parent directories in order to find login credentials. 
Note that `cpt` isn't supposed to store your credentials on your disk unless it's necessary for all logins.
All you need it to call: 
```sh
$ cpt login <your judge system goes here>
# initialize your project structure
$ mkdir a && cd a && code source.cpp
$ cpt submit source.cpp
  #  credentials found by scanning parent directories
  # problem_id guessed by directory name
```

## Install

### [crates.io]()

TODO: Add this library to [crates.io]().

### Build from source

Assuming you already have `rust` alongside `cargo` installed.: 
```sh
$ git clone https://github.com/dendi239/cp-tool.git
$ cd cp-tool
$ cargo install --path .
```

## Contributing

Contributing is higly welcomed and recommendent.
This is my first rust project, so feel free to leave any issue, even about code style, or suggestions to make it better in terms of traits infrastructure.

