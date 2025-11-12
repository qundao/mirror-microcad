# Automatic Tested Code Examples

Within this document all code examples are automatically tested.

## Test result banners

Each code example has a banner nearby which indicates the current testing
status of the code.
If you click one of these banners you will jump to to the test report.

The following banners indicate that the code is tested ok:

| Banner                                       | Meaning                     |
| -------------------------------------------- | --------------------------- |
| ![ok](./images/ok.svg)                       | Ok                          |
| ![ok](./images/ok_warn.svg)                  | Ok (with warnings)          |
| ![fail_ok](./images/fail_ok.svg)             | Ok to fail                  |
| ![not_todo](./images/not_todo.svg)           | marked as todo but is ok    |
| ![not_todo_fail](./images/not_todo_fail.svg) | todo but fail intentionally |

If you see one of the following banners, we did something wrong.
Either the example may be wrong or the microcad compiler might have a bug.

| Banner                                 | Meaning                     |
| -------------------------------------- | --------------------------- |
| ![fail](./images/fail.svg)             | Fails                       |
| ![fail_wrong](./images/fail_wrong.svg) | Fails with wrong errors     |
| ![ok_fail](./images/ok_fail.svg)       | Is ok but was meant to fail |
| ![parse_fail](./images/parse_fail.svg) | Fails early while parsing   |

The following banners occur if a test is still marked as todo but is ok
already.
This can be corrected by changing the documentation.

| Banner                               | Meaning                        |
| ------------------------------------ | ------------------------------ |
| ![todo](./images/todo.svg)           | Work in progress               |
| ![todo_fail](./images/todo_fail.svg) | Work in progress (should fail) |

