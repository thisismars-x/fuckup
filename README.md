# fuckup

What's fuckup good for? fuckup will hijack any python virtual environment to stop pythonic code from being interpreted. The code you type in remains the same, but becomes erroneous to the python3 interpreter.

fuckup is written entirely on Rust, so have rust installed on your system. Here is an awesome [tutorial](https://www.rust-lang.org/tools/install).

## Installation

Just clone this repository and run,

```bash
cargo install --path .
```

## Usage

Make sure to be in the directory of your virtual environment, where your bin and lib folders are located. Then,

```bash
fuckup
```

This will do everything by itself. Activate your environment, running any code now will be erronous

```python 
# main.py
# This code would not run
print("Hello World")

# python3 main.py will not work as expected.
```

A tiny note: the program should run on Windows, but I could not test on a Windows machine, as I run debian.

## Contributing

I intend to expand this project to all major languages, for Unix and Windows machines. Pull requests are welcome.

## License

[MIT](https://choosealicense.com/licenses/mit/)
