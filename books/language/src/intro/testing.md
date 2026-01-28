# Automatic Tested Code Examples

Within this document all code examples are automatically tested.

## Test result banners

Each code example has a banner nearby which indicates the current testing
status of the code.
If you click one of these banners you will jump directly into a specific
*test report*.

The following banners indicate that the code is **tested ok**:

| Banner                           | Meaning             |
| -------------------------------- | ------------------- |
| ![ok](./images/ok.svg)           | Ok                  |
| ![ok](./images/ok_warn.svg)      | Ok (with warnings)  |
| ![fail_ok](./images/fail_ok.svg) | Fails intentionally |

The following banners occur if a test is still **marked as todo but is ok**
already.
This can be corrected by changing the documentation.

| Banner                                       | Meaning                                |
| -------------------------------------------- | -------------------------------------- |
| ![not_todo](./images/not_todo.svg)           | Marked as todo but is ok               |
| ![not_todo_fail](./images/not_todo_fail.svg) | Marked as todo but fails intentionally |

If you see one of the following banners, we did something wrong.
Either the **example may be wrong** or the **µcad interpreter might have a bug**.

| Banner                                 | Meaning                             |
| -------------------------------------- | ----------------------------------- |
| ![fail](./images/fail.svg)             | Fails with errors                   |
| ![fail_wrong](./images/fail_wrong.svg) | Fails with wrong errors or warnings |
| ![ok_fail](./images/ok_fail.svg)       | Is ok but was meant to fail         |
| ![ok_wrong](./images/ok_wrong.svg)     | Is ok but with wrong warnings       |
| ![parse_fail](./images/parse_fail.svg) | Fails early while parsing           |

The following banners occur if tests are **marked as todo** and so are not running
successful.

| Banner                                | Meaning                                      |
| ------------------------------------- | -------------------------------------------- |
| ![todo](./images/todo.svg)            | Work in progress                             |
| ![todo_fail](./images/todo_fail.svg)  | Work in progress (should fail)               |
| ![todo_wrong](./images/todo_warn.svg) | Work in progress (fails with wrong warnings) |

## Test list

See [this section](../appendix/test_list.md) for a list of tests within this document.
