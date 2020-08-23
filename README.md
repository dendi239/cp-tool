# cp-tool

Competitive programming tool to submit your sources. 
It was inspired by [cf-tool](https://github.com/xalanq/cf-tool), but that one works only with Codeforces.

## Features (TODO)

- [x] Stores config in project-related directory
- [x] Guesses `problem_id` from either filename or directory name
- [ ] Current contest status
- [ ] Current standings
- [ ] Submit response feature

Works with various judge systems
- [x] ejudge
- [ ] CodeForces
- [ ] Yandex.Contest
- [ ] AtCoder
- [ ] TopCoder
- [ ] Algotester
- [ ] DDOTS

## Usage

Basic usage is following: 
```sh
$ cpt login ejudge --url <YOUR_URL> --contest-id <CONTEST_ID>
username: awesomeyou
password: 🔑
$ mkdir a && cd a
$ code source.cpp
$ cpt submit a.cpp
```

That's it! Note that there's some work under the hood: after login's called, cpt creates file `.cp-tool.config` with some json-based config (there's json to allow you edit it manually). Since that when you call `cpt submit` it scans enviroment to find suitable configuration and uses it to perform actual submitting.

It's quite hard to choose how to store your sources: 
```plain
your-contest-dir
├── a.cpp
├── b.cpp
└—— c.py

or

your-contest-dir
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
However, `cpt` doesn't make any assumptions about your project structure and scans all parent directories to find login credentials. 
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

### As crate from [crates.io](https://crates.io)

TODO: Add this library to [crates.io](https://crates.io).
After it's been published, you may use:
```sh
$ cargo install cp-tool
```

### Build from source

Assuming you already have `rust` alongside `cargo` installed.: 
```sh
$ git clone https://github.com/dendi239/cp-tool.git
$ cd cp-tool
$ cargo install --path .
```

## Contributing

Contributing is highly welcomed and recommended.
This is my first rust project, so feel free to leave any issue, even about code style, or suggestions to make it better in terms of traits infrastructure.

## Credits

Made with :heart: by [Denys Smirnov](mailto:dendi239+github+cpt@gmail.com)
