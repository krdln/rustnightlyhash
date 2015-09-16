rustnightlyhash computes a git commit hash for a nightly from a given date
in YYYY-MM-HH format.
When given no date, it shows the hash for most recent nightly
(even if the build is still in progress).

```
Usage:
  rustnightlyhash [options] [<date>]

Options:
  -h --help         Show this screen.
  -d --output-date  Displays the date after the hash.
```
