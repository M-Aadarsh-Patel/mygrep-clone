# Test fixtures

`sample.txt` is the main fixture. Many issue acceptance criteria depend on its
exact six lines, so **don't edit it**. Add a new fixture instead.

`edge/` holds one file per input hazard. Each file isolates one problem, because
some hazards contaminate a whole file (a single NUL byte makes grep treat the
entire file as binary).

Expected behaviour below is GNU grep 3.7 with `LC_ALL=C.UTF-8`. The differential
tests are the source of truth; this table is a guide.

| File | Hazard | GNU grep behaviour worth knowing |
| -- | -- | -- |
| `empty.txt` | zero bytes | `grep rust` prints nothing, exits 1 |
| `no-trailing-newline.txt` | last line has no `\n` | still matched; output gains a trailing `\n`; `-c` counts it |
| `crlf.txt` | Windows `\r\n` endings | `\r` is part of the line, so `-x rust` matches nothing |
| `invalid-utf8.txt` | bytes that aren't valid UTF-8 | treated as **binary** in a UTF-8 locale; `-a` or `LC_ALL=C` prints the lines |
| `binary.bin` | NUL byte mid-file | whole file is binary: prints `binary file matches`; `-c rust` still prints `3` |
| `unicode.txt` | multibyte text, non-ASCII case | `-i` folds `Ü/ü`, Cyrillic and `İ/i`; `ß` does not fold to `SS` |
| `regex-chars.txt` | regex metacharacters, lines starting with `-` | `a.b` matches 3 lines, `-F a.b` matches 1 |
| `words.txt` | word boundaries for `-w` | `-w rust` matches `rust`, `trust rust`, `rust-lang`, `rust.`; not `rusty`, `rust_lang`, `_rust_`, `9rust`, `rust9` |
| `many-matches.txt` | several matches per line | `-o rust` prints 5 lines; `-o aa` on `aaaa` prints 2 (no overlaps) |
| `whitespace.txt` | tabs, leading/trailing spaces, consecutive blank lines | whitespace is ordinary bytes; nothing is trimmed |
| `context.txt` | 20 numbered lines, `MATCH` on 1, 5, 6, 12, 20 | `-A1 -B1` merges 5 and 6 into one group and prints `--` between groups |
| `long-line.txt` | one 100 KB line | must match across buffer boundaries without unbounded memory |
| `-dash-name.txt` | filename starting with `-` | needs `--` (or `./`) or it's parsed as a flag |
| `name with spaces.txt` | spaces in a filename | prefix printed as-is |
| `colon:in:name.txt` | `:` in a filename | prefix is ambiguous with `file:line`; GNU prints it as-is |

## Not stored here

* **Unreadable files**: git doesn't keep permissions, so create them inside the
  test (`chmod 000` on a temp file).
* **Huge files** for memory and speed checks: generate at benchmark time,
  never commit.
* **Directories and symlink loops** for `-r`: build them in a temp dir in the test.
