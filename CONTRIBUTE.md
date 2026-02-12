# Contribute

Thank you for your interest in contributing to microcad!

The microcad community welcomes all contributions, including bug reports, feature requests, and pull requests.

## Find a first issue

If you found a bug or have an idea, please checkout the existing issues first ([https://codeberg.org/microcad/microcad/issues]).
If you do not find your problem or solution mentioned there, feel free to create a [new issue](https://codeberg.org/microcad/microcad/issues/new).

Issues that are good for new contributors are tagged with the [`first good issue` label.](https://codeberg.org/microcad/microcad/issues?q=&type=all&sort=relevance&labels=685366&state=open&milestone=0&project=0&assignee=0&poster=0)

## Our contribution guidelines

We aim to make contributing easy, welcoming, and enjoyable. The practices below help us keep the project healthy and accessible, and we’re always open to improving them together.

* **Documentation matters.** We try to keep code and documentation clear, readable, and up to date so others can easily understand and build on the project.

* **We lean on helpful tools.** We’re big fans of clippy and use it regularly to catch issues early and keep the codebase consistent.

* **Docs are tests, too.** We write tests through documentation, embedding µcad snippets directly in Markdown so examples stay close to the behavior they describe.

If you’re unsure about any of these or have ideas for doing things differently, that’s totally okay—questions, suggestions, and PRs are always welcome.

## Getting started

First, install [*Git*](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
and [*Rust*](https://www.rust-lang.org/tools/install).

### Getting and building the source Code

Clone the source code repository:

```sh
git clone https://codeberg.org/microcad/microcad
cd microcad
```

After cloning the repository, you can build microcad using Cargo:

```sh
cargo build
```

## Documentation-driven testing

Starting as a team of two, we had to chose a way to bring code and documentation into sync.
This especially important when developing a programming language!

Hence, we follow an approach called **documentation-driven development**.
This approach guarantees to test each µcad code snippet and shows the test result in a banner above the test.

### Executing tests

Those tests can be run with `cargo test`.
The produced output will be saved in folders called `.test` which is beside the source file of the test.

Outdated or removed tests will be cleaned up automatically but when in doubt use `cargo clean`
and maybe some `find -name .test | xargs rm -r`.

We also commit the results to the repository to monitor any changes in our IDEs.

### Writing markdown tests

One may insert *µcad* code into the *markdown* files, which then will get tested automatically if you run
`cargo test` and name them like:

````md
```µcad,my_test
````

The *markdown* will be searched for any *µcad* code and appropriate *rust* tests will be
[generated](https://codeberg.org/microcad/microcad/src/branch/main/tests/microcad_markdown_test).

#### Test modes

beside the name you may add a test mode (see table below):

````md
```µcad,my_test#fail
````

The tests will create `.test` folders beside the *markdown* files.
The tests will then copy an [image file (`*.svg`)](https://codeberg.org/microcad/microcad/src/branch/main/tests/images)
for every test which signals the test result into the `.test` folder.
They can be included in the *markdown*, if you use this code:

````md
![test](.test/my_test.svg)
```µcad,my_test
````

#### Accessing test logs

You may also give the reader access to the logs by clicking on the banner with:

````md
[![test](.test/my_test.svg)](.test/my_test.log)
```µcad,my_test
````

#### Automatically update test banners

There is a [script](https://codeberg.org/microcad/microcad/src/branch/main/update_md_banner.sh) which updates all
banners automatically.

#### Test results and marks

| Image                                            | MD Code Type | Mark                      | Code                                     | What do do?            |
| ------------------------------------------------ | ------------ | ------------------------- | ---------------------------------------- | ---------------------- |
| ![fail_ok](tests/images/fail_ok.svg)             | `µcad`       | `#fail`,`#warn`           | Fails intentionally                      | ok                     |
| ![fail_wrong](tests/images/fail_wrong.svg)       | `µcad`       | `#fail`,`#warn`           | Fails but with wrong errors or warnings  | fix test or code       |
| ![fail](tests/images/fail.svg)                   | `µcad`       |                           | Fails                                    | fix test or code       |
| ![not_todo_fail](tests/images/not_todo_fail.svg) | `µcad`       | `#todo_fail`,`#todo_warn` | Fails as expected but still marked to do | remove `#todo_`        |
| ![not_todo](tests/images/not_todo.svg)           | `µcad`       | `#todo`                   | Succeeds but still marked to do          | remove `#todo`         |
| ![ok_fail](tests/images/ok_fail.svg)             | `µcad`       | `#fail`                   | Succeeds but should fail                 | find out why           |
| ![ok](tests/images/ok.svg)                       | `µcad`       |                           | Succeeds                                 | ok                     |
| ![parse_fail](tests/images/parse_fail.svg)       | `µcad`       | -                         | Parsing has failed                       | fix grammar            |
| ![todo_fail](tests/images/todo_fail.svg)         | `µcad`       | `#todo_fail`,`#todo_warn` | Needs more work to fail (proper)         | create issue/implement |
| ![todo](tests/images/todo.svg)                   | `µcad`       | `#todo`                   | Needs more work to succeed               | create issue/implement |
| -                                                | `µcad`       | `#no-test`                | Ignore completely                        | yolo!                  |
| -                                                | -            | -                         | Ignore completely                        | yolo!                  |
| -                                                | *(other)*    | -                         | Ignore completely                        | yolo!                  |

#### Mark errors and warnings

Code lines which intentionally produce errors must be marked with `// error` to make the test succeed.
Code lines which shall produce warnings can be marked with `// warning` to check if those warnings are happening.
Any unmarked warnings will be ignored.

In the following example a warning and an error are marked with comments:

````md
```µcad,missed_property#fail
sketch Wheel(radius: Length) { // warning (no output)
    init( width: Length ) { } // error: Workbench plan incomplete. Missing properties: radius
}
Wheel(width = 1.0mm);
```
````

#### Generating exports

If test code produces geometry, that will be exported by running the tests too.
Tests are run in **low resolution** to make them faster.
If you require a test output to be **high resolution**, then add `(hires)` to the header line:

````md
```µcad,final(hires)
````

## Contributing to the books

The `./books` folder contains books written in [markdown](https://rust-lang.github.io/mdBook/).
Each book also contains µcad code snippets, from which tests are generated.

Currently, the following books exist:

* **[µcad Language Reference](books/language/book.toml)**.

  This book contains the µcad language reference.
  Every language feature is supposed to be documented (and tested) here.

* **[Tests book](books/tests/book.toml)**

  This book contains additional tests that are not part of the µcad language reference.

* **[Tutorials](books/tutorials/book.toml)**

  This book contains microcad tutorials dedicated to new users.
  Tutorials are important for making microcad as user-friendly as possible, hence every contribution is welcome!

* **[Concept Pool](books/concept_pool/book.toml)**

  This book contains raw concepts that are not part of the language (yet), but may be included in the roadmap at some point.
  Feel free to contribute your ideas here.

* **[Examples Book](books/examples/book.toml)**

  This book is auto-generated from all the examples in the `examples/` folder.
  Instead of contributing directly to this book, please contribute examples to the `examples/` folder instead.

## ❤️ Support the project

If you like this project, you can help us spending more time with it by donating:

<a href="https://opencollective.com/microcad/donate" target="_blank">
<img src="https://opencollective.com/microcad/donate/button@2x.png?color=blue" width=300 />
</a>
