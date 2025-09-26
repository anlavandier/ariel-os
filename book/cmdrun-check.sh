#!/bin/sh
# This file is run by CI and checks for unintended uses of 'cmdrun'.
# Currently, only calls to `gen_book.rs matrix` and `gen_book.rs board-list` will pass.

set -o nounset

# Disable the errexit shell option to avoid exiting prematurely when the `grep`
# command doesn't match anything, which means there are no forbidden commands.
set +e
# Check for forbidden commands inside `<!-- cmdrun` tags
# NOTE: this uses negative look-ahead, which may not be available on every platform.
grep -qroP '(?s)<!--\s*cmdrun\s+(?!(\.\./\.\./doc/gen_book\.rs matrix \.\./\.\./doc/support_matrix\.yml /dev/stdout --tier [123]|\.\./\.\./\.\./doc/gen_book\.rs board-list \.\./\.\./\.\./doc/support_matrix\.yml /dev/stdout --tier [123])\s*-->).*?-->' src/
res=$?
# Re-enable the errexit option to exit immediately if the `test` command fails
# (meaning the return code of `grep` was 0 -> forbidden commands were found).
set -e
test $res -eq 1

# Check that we have the expected amount of allowed commands for `gen_book.rs matrix`
test "$(grep -roP '<!--\s*cmdrun\s+\.\./\.\./doc/gen_book\.rs matrix \.\./\.\./doc/support_matrix\.yml /dev/stdout --tier [123]\s*-->' src/ | wc -l)" -eq 3

# Check that we have the expected amount of allowed commands for `gen_book.rs board-list`
test "$(grep -roP '<!--\s*cmdrun\s+\.\./\.\./\.\./doc/gen_book\.rs board-list \.\./\.\./\.\./doc/support_matrix\.yml /dev/stdout --tier [123]\s*-->' src/ | wc -l)" -eq 3
