<p align="center">
<img src="./.github/logo.png">
</p>
<p align="center">
<a href="https://travis-ci.org/Drogglbecher/thinline"><img alt="Travis Status" src="https://travis-ci.org/Drogglbecher/thinline.svg"></a>
<a href="https://ci.appveyor.com/project/Drogglbecher/thinline"><img alt="Travis Status" src="https://ci.appveyor.com/api/projects/status/r3ldomvr2plv336t?svg=true"></a>
<a href="https://coveralls.io/github/Drogglbecher/thinline?branch=master"><img alt="Travis Status" src="https://coveralls.io/repos/github/Drogglbecher/thinline/badge.svg?branch=master"></a>
<a href="https://drogglbecher.github.io/thinline"><img alt="Travis Status" src="https://img.shields.io/badge/master_doc-thinline-blue.svg"></a>
<a href="https://github.com/Drogglbecher/thinline/blob/master/LICENSE"><img alt="Travis Status" src="https://img.shields.io/badge/license-Apache%202-blue.svg"></a>
</p>

## Description
*Thinline* is a project for handling and executing unittests written in function comment sections for C/C++. **It's currently under development and has an early alpha state.**

## Installation
### Requirements
To use *thinline* you need a valid [Rust](https://www.rust-lang.org/en-US/install.html) installation and it's package manager [cargo](https://crates.io/install). Depending on your OS you can install them via the package manager you like the most. Besides this you can use [rustup](https://rustup.rs/) if you want but keep in mind that this can conflict with already existing installations of rust, so uninstall them first. It is also necessary to have a `g++` installed to build your unittest later on (when running *thinline* with `--dry-run` you don't need this since the testfiles are only created). When you want to execute the examples out of the box you should make sure you have `cmake` and `make` and [google test](https://github.com/google/googletest).

Besides this *thinline* uses the [rust clang implementation](https://github.com/KyleMayes/clang-rs), so please make sure to also fulfill its [requirements](https://github.com/KyleMayes/clang-sys#dependencies).
### Manual installation
Just clone this repo and then this simple installation command should be enough:
```
cargo install
```

## CLI-Usage
The usage of the CLI-tool is basically simple. When [preparation](#preparation-and-configuration) is done `thinline --help` prints the usage:
```
USAGE:
    thinline [FLAGS] [OPTIONS] <SOURCE-DIR>
FLAGS:
    -b, --build      Executes the build script steps given in the project thinline setting file.
    -d, --dry-run    Creates only the test files in the projects .thinline folder without exexcuting them.
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Set the verbosity level (`v` -> DEBUG, `vv` -> TRACE)
OPTIONS:
    -p, --project-config <YAML_FILE>    The name of the yaml file where the C/C++ project parameters for thinline are
                                        stored. This file has to be at <SOURCE-DIR>. [default: .thinline.yml]
ARGS:
    <SOURCE-DIR>    The directory where the sources for test-extraction are located
```
### Example
To see a quick example how it works you can use *thinline* with the c or cpp sample project you find in the `examples` folder. After the installation of the neccesay tools just do
```
thinline -b <thinline-path>/examples/cpp_project
```
and you should see an output like this:
```
[thinlinelib] [INFO] Log level set to: INFO

[thinlinelib] [INFO] Execute build step 'mkdir build'
[thinlinelib] [INFO] Execute build step 'cmake -Bbuild -H.'
[thinlinelib] [INFO] Execute build step 'make -C build'

[thinlinelib::testcase] [WARN] Test command '#TL_LT[this->class_inst->TL_FCT(no1: no1, no2: 10) => 30]' not found in 'google_test' environments function description ==> Skipping.
[thinlinelib] [INFO] Execute build step 'g++ header1.cpp -I../include -Wl,-rpath,../build/ -L../build/ -lcpp_example -lgtest -lgtest_main -o thinline_test'

[thinlinelib] [INFO] Execute build step './thinline_test'
[==========] Running 2 tests from 2 test cases.
[----------] Global test environment set-up.
[----------] 1 test from c2Test
[ RUN      ] c2Test.AddThreeNumbers
[       OK ] c2Test.AddThreeNumbers (0 ms)
[----------] 1 test from c2Test (0 ms total)

[----------] 1 test from c1Test
[ RUN      ] c1Test.AddTwoNumbers
[       OK ] c1Test.AddTwoNumbers (0 ms)
[----------] 1 test from c1Test (0 ms total)

[----------] Global test environment tear-down
[==========] 2 tests from 2 test cases ran. (0 ms total)
[  PASSED  ] 2 tests.
```

## Preparation and configuration
To use the project there are some preparation & configuration steps to go before you can use it for your project.
### Setup Project
Within the main folder of your project you should place a file named `.thinline.yml`. This file in [YAML](http://yaml.org/)-format describes several parameters, so *thinline* knows how to act. This usually looks like somehow like this:
```yaml
---
test_env: google_test
src_dirs:
  - src
  - include
include_dirs:
  - include
build_script:
  linux:
    - mkdir build
    - cmake -Bbuild -H.
    - make -C build
libs:
  - build/libcpp_example.so
language_features:
  cpp:
    whitelist_classes:
      - c1
      - c2
```
* **`test_env`:**

   The test environment which is used to build your unittest infrastructure, e.g. `google_test`. This entry is closely connected to the pool of environment stubs which can be found at `stubs` or at the several [home path](https://doc.rust-lang.org/std/env/fn.home_dir.html) after executed once (`~/.config/thinline` for linux, `~/thinline` for windows). Here you can find `env_stubs.yml` describing the stub paths of several test environemts. You can extend this for every test framework you want. For further information about extending environments, see chapter [Setup Environments](#setup-environments).
* **`src_dirs`:**

   The source directories. All files within these and their subdirectories are scanned for *thinline* test commands. How they look like and how to configure is described in the chapter [Setup Tests](#setup-tests).
* **`include_dirs`:**

   These are the include directories which are necessary to resolve the function signatures when bulilding the unittest executables.
* **`build_script`:**

   Several steps can be specified to build your project first under linux and windows (and so are the subparameters). Pay attention that at the moment every step is executed with the <SOURCE-DIR> you specified when starting *thinline* as working directory.
* **`libs`:**

   At this point you should specify the libs you need to build the unittest executable.
* **`language_features`:** Special features for several languages

   * **`cpp`:**

      * **`blacklist_namespaces`:**

         Specify namespaces whose classes and functions are excluded from parsing.
      * **`whitelist_namespaces`:**

         When specified namespaces here *thinline* only parses classes and functions within them.

      * **`blacklist_classes`:**

         Specify classes whose functions are excluded from parsing.
      * **`whitelist_classes`:**

         When specified classes here *thinline* only parses functions within them.

### Setup Tests
Since this tool is about writing unittests in function comment sections, let's have a look how to do that. First of all, *thinline* recognizes comment sections with [doxygen notation](https://www.stack.nl/~dimitri/doxygen/manual/docblocks.html) or beginning with `///`. Since there can occur complications with your normal doxygen documentation (when using it) it is recommented to use the 3-slash-notation. A function signature with its *thinline* test notation can e.g. look like this:
```cpp
/**
* #TL_TESTCASE(c1::AddTwoNumbers)
*     unsigned int no1 = 5;
*     #TL_EQ[TL_FCT(no1: no1, no2: 10) => 15]
*     #TL_NE[add_two_numbers(no1, 10) => 30]
* #!TL_TESTCASE
*/
unsigned int add_two_numbers(unsigned int no1, unsigned int no2);
```
#### Setup Testcases
The beginning of a testcase (which will at the end lead into one testfunction) is marked with
```
#TL_TESTCASE(<testclass>::<name of test function>)
```
In the brackets after `#TL_TESTCASE` you specify the name of the test function and the testclass it should contain to in a class-method-notation separated with `::`. A testcase end gets detected when the following line was seen:
```
!#TL_TESTCASE
```
Between these both identifiers you can write your testcases. Every line between starting with `#` will invoke a *thinline* argument followed by the testfunction ID. Where and how there are defined will be explained in chapter [Setup Environments](#setup-environments) at this description. The rest of the lines are handled as *raw* lines and get transferred directly to the unittest as they are. In the example above e.g. `unsigned int no1 = 5;` is such a raw line.

##### Special Notations:
* **`TL_FCT`:**
   This notation you can conceive like a macro expanding to the function signature you write the current testcase for. This avoids to rewrite the test every time the name of your signature changes. Of course you still have to change the parameter notations when they change. This notation can only be used inside a *thinline* argument.

##### Notes:
* When you use a testclass which was not defined yet (since you don't use a special behaviou, ...) then the one you specify is created as blanko class. Whenever a testclass setup is found for this class, the necessary information are added.

#### Setup Testclasses
A testclass you can specify exactly one time. It is the representation of the class in the several test environment where the methods you specify are assigned later on. Basically it's not important in which comment section you specify a testclass but it should be one of a constructor, desctructor or a function/method since comments of other entities are not parsed. Best practice is to simply write it into the constructor comment of the class related to the testclass you want to create. For the Testcase example above a testclass setup can e.g. look like this:
```cpp
/**
* #TL_TESTCLASS(c1)
*     #TL_SET_UP_CONTEXT:
*         this->class_inst = new c1();
*     #TL_TEAR_DOWN_CONTEXT:
*         delete this->class_inst;
*         this->class_inst = nullptr;
*     #TL_CLASS_CONTEXT:
*         c1 * class_inst;
* #!TL_TESTCLASS
*/
c1() {};
```
Unlike testcases there are no parameterizable *thinline* arguments here but defined ones filling the testclass when create the unittest file. The basic architecture of such a class depending on the test environment you can see inside the dedicated `class.stub` file or raw string in the `env_stubs.yml` (You will get further information about this within the next chapter). Anyhow the arguments which can be specified inside such a testclass are the following:

* **`#TL_CONSTRUCTOR_CONTEXT`:** These lines get transferred into the testclass constructor.
* **`#TL_DESTRUCTOR_CONTEXT`:** These lines get transferred into the testclass destructor.
* **`#TL_SET_UP_CONTEXT`:** These lines get transferred into the testclass SetUp-method.
* **`#TL_TEAR_DOWN_CONTEXT`:** These lines get transferred into the testclass TearDown-method.
* **`#TL_CLASS_CONTEXT`:** These lines are written into the function but in no certain method. A good place to create class members and so on.

### Setup Environments
After executing *thinline* once you should have a directory like this in your home/config dir:
```
thinline
├── environment
│   ├── env_stubs.yml
│   ├── google_test
│   │   ├── class.stub
│   │   ├── fct.stub
│   │   └── file.stub
│   └── …
└── system
    ├── …
    └── tlg.h
```

* **`env_stubs.yml`:**

   Here you can find the several unittest environment descriptions, e.g. how there file, class and functions stubs look like and *thinline* test signatures can be defined. Such a file looks usually like this:
```yaml
---
google_test:
  file:
    file: google_test/file.stub
  class:
    file: google_test/class.stub
  function:
    raw: |
      TEST_F(//#TEST_CLASS#/Test, //#TEST_NAME#/) {
      //#TEST_CONTEXT#/
      }
  output_format: cpp
  test_signatures:
    TL_EQ:
      inline: //#ARG_0#/ => //#ARG_1#/
      stub: EXPECT_EQ(//#ARG_0#/, //#ARG_1#/);
    TL_NE:
      inline: //#ARG_0#/ => //#ARG_1#/
      stub: EXPECT_NE(//#ARG_0#/, //#ARG_1#/);
```
   * **`file`, `class` & `function`:**

      The stub strings are for the test environments file, class and function are defined here. This is the frame where the parsed tests are written in. With `file: <file path>` you can specify a file containing the particular stub or you can write it directly as text with `raw: <stub string>`. To fill in the necessary information parsed from your comments there are several identifiers for eachs stub which have a special meaning:

      * **For `function` stub:**
         * **`//#TEST_CLASS#/`:** Name of the test class.
         * **`//#TEST_NAME#/`:** Name of the test.
         * **`//#TEST_CONTEXT#/`:** All parsed, transformed and prepared lines regarding one test case (this means within one test function)

      * **For `class` stub:**
         * **`//#TEST_CLASS#/`:** Name of the test class.
         * **`//#CONSTRUCTOR_CONTEXT#/`:** The constructor context parsed from the lines you specify at `#TL_CONSTRUCTOR_CONTEXT`.
         * **`//#DESTRUCTOR_CONTEXT#/`:** The destructor context parsed from the lines you specify at `#TL_DESTRUCTOR_CONTEXT`.
         * **`//#SET_UP_CONTEXT#/`:** The context of the setUp function parsed from the lines you specify at `#TL_SET_UP_CONTEXT`.
         * **`//#TEAR_DOWN_CONTEXT#/`:** The context of the tearDown parsed from the lines you specify at `#TL_TEAR_DOWN_CONTEXT`.
         * **`//#CLASS_CONTEXT#/`:** The class context parsed from the lines you specify at `#TL_CLASS_CONTEXT`.

      * **For `file` stub:**
         * **`//#TEST_NAMESPACES#/`:** This line gets replaced with all namespace usages for this test.
         * **`//#PRECLASS_CONTEXT#/`:** The things you specify in a testfiles `#TL_PRECLASS_CONTEXT`.
         * **`//#TEST_CLASSES#/`:** The collection of all testclasses needed in this testfile.
         * **`//#TEST_CASES#/`:** The collection of all testcases needed in this testfile.

   * **`output_format`:**

      Defines the fileformat which a testfile using the specified framework has usually.

   * **`test_signatures`:**

      Here you can specify stubs for test signatures you use later on when specifying test cases in your function comments. Basically you can name them whowever you want, as long as they have a `inline` and `stub` parameter. The `inline` parameter specifies how the notation for this test signature looks like in your comment-section-test and `stub` is the related signature for the particular test environment. In the above examples context you could write something like
      ```
      #TL_EQ[function_returns_1(arg1: 1) => 1]
      ```
      in your comment section. The expression in square brackets is the `inline` parameter of `TL_EQ`, the function call is `//#ARG_0#/` and the expected value 1 `//#ARG_1#/`. In the testfile created by *thinline* this line gets transformed to 
      ```cpp
      EXPECT_EQ(function_returns_1(1), 1);
      ```

## Current Features
* Basic features
   * [x] Parsing of Testcase setups
   * [x] Parsing of Testclass setups
   * [x] Automatic integration of used namespaces
   * [x] Manual implementation of new test environments
   * [x] Pre-build before build unittest infrastructure
   * [x] `dry-run` flag to integrate in own unittest infrastructure
* Shipped test environment stubs
   * [x] [Google Test](https://github.com/google/googletest)

## Planned Features
* [ ] More function tests and stability
* [ ] Notations to simplify the generation of easy testclasses
* [ ] Automatic creation of a web-based documentation

## Contribution
If you want to contribute you will earn my deep gratitude and everlasting honor ;)
