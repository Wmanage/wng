<h1 align="center">Wanager</h1>
<br>
<br>
<div align="center">
<img width=200 src="assets/wng.png"/>
</div>
<br>
<div align="center">

![GitHub release (latest by date)](https://img.shields.io/github/v/release/Wafelack/wng?color=%23888800&label=Latest%20release&style=flat-square)

![Crates.io](https://img.shields.io/crates/d/wng?color=sucess&label=Downloads%20%28crates.io%29&style=flat-square)

</div>

---

<br>
<br>
<div align="center">
	
![CI](https://github.com/Wafelack/wng/workflows/Build/badge.svg) 

## Dependencies

|                 |        |
|-----------------|--------|
|    serde_json   |  1.0.X |
| lines_from_file |  0.1.1 |
|     fs_extra    |  1.2.0 |
|     colored     |  2.X.X |
|       tar       | 0.4.30 |

</div>

<br>
<br>
<br>

---

**Wanager** is packages/project manager for C & C++ written in [Rust](https://rust-lang.org). It allows you to create projects, headers, to install libraries & to compile and run fast and easily.

<br>

- [I - Setup](#i--setup)
  - [A - Prerequisties](#a--prerequisties)
  - [B - Installation](#b--installation)
- [II - Project creation](#ii--create-a-new-project)
- [III - Compile & Run](#iii--compile-and-run)
  - [A - Custom Build](#a--custom-build)
  - [B - Wngbuild library](#b--wngbuild-library)
- [IV - Features](#iv--features)
  - [A - Archive](#a--archive)
  - [B - Reinitialisation](#b--to-reinitialize-a-project)
  - [C - Header](#c--to-create-a-header-file)
  - [D - Tests](#d--to-run-tests)
- [V - Libraries](#v--libraries)
  - [A - Install](#a--to-install-a-library)
  - [B - Publish](#b--publish-your-library)
- [VI - Contributors](#vi--contributors)
- [VII - Contributing](#vii--contributing)

<br>

# Documentation

## I / Setup

### A / Prerequisties

Make sure to have [Git](https://git-scm.com) & [gcc](https://gcc.gnu.org/) installed on your computer.

### B / Installation

To install wng you can either :
- Download the latest binary in the releases
- Download updater.sh (*Nix only and needs cargo)
	- *Note : you can setup a cron to keep an up to date wng version 😉*
- Run `cargo install wng`
	- In `~/.cargo/bin/` if you are on *nix
	- In `C:\Program Files\` if you are on Windows

## II / Create a new project

Open the command prompt and run :

```
$ wng new <project_name> [--cpp]
$ cd project_name/
```

Three folders have been created, `tests/`, `src/` and `build/`

In `src/`, you'll find file `main.c[pp]` that contains a basic hello world program.

<br>

## III / Compile and Run

```
$ wng build

$ wng run <args>
Hello World
```

_NOTE : `wng build` will build a debug executable, with flags -W -Wall -Werror -Wextra. To disable this, build in release mode with : `wng build --release`_

### A / Custom build

To build with a custom build, you have to create a `build.py` or `build.rb` file with your code to build.

If you want to specify a special python / ruby interpreter path, add the section `"pyinterpreter" : "path2python"` or `"rbinterpreter" : "path2ruby"` to your project.json.

Minimal python version required : 3.5
Minimal ruby version required : 2.3

Then run your script with `wng build --custom`

### B / Wngbuild library

Wngbuild library provides some useful features to compile your project

It is available in Ruby and Python

*Note : If both build.rb & build.py files exists, build.py will be used*

```py
from wngbuild import * # Import all from wngbuild module

build = BuildProfile(files="src/*.c",output="build/custom/prog.exe" ) # setup a build profile that will compile all files in src/ and place the binary in build/custom/prog.exe
build.cc = "C:\\MinGW\\bin\\gcc.exe" # Setup the compiler (optional, by default "gcc")
build.flags = "-W -Wall -Werror -Wextra" # Setup the flags that the command will be run with (optional)

build.run() # Run the compilation command
build.runOutput() # Run the binary produced by the compilation command (Will raise an error if the compilation command fails)
```

```rb
require_relative "wngbuild"

build=BuildProfile.new("src/*.c", "build/custom/prog") * Setup a build profile
builc.cc="C:\\Program Files\\clang\\bin\\clang.exe"

build.run() * Run compilation
build.runOutput() * Run produced file (Will raise an error if compilation failed)
```

<br>

### C / Checking

You can just check if there is any errors or warnings in your code without producing any binary with `wng check`

## IV / Features

### A / Archive

To create a gunzip archive of your project files, just run `wng archive` and a file called `project.tar.gz` will be created

### B / To reinitialize a project

```
$ wng reinit
Really want to reinit ? Y/N : Y
Project renitialized !
```

### C / To create a header file

```
$ wng header foo
$ cat foo.h
#ifndef _FOO_H_
#define _FOO_H_


#endif /*_FOO_H*/
```

### D / To run tests

Tests have to be in tests/tests.c

To use functions that are in src/ files, just include the header with `#include "../src/<header>.h"`

Then you can run them with `wng test`

## V / Libraries

### A / To install a library

```
$ cd yourproject/
$ wng install <source>:<username>/<repo_name>
```

_Available sources are : `github`,`gitlab` & `bitbucket`_
_NOTE : Repository has to have a `lib/` folder inside or wng will refuse to install it_

### B / Publish your library

Create a repository on GitHub, BitBucket or GitLab with your project, library files have to be in a `lib/` folder

## VI / Contributors
<a href="https://github.com/wmanage/wng/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=wmanage/wng" />
</a>

## VII / Contributing

See our [contribution guidelines](https://github.com/wmanage/wng/blob/master/CONTRIBUTING.md).
