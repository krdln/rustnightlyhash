# Rustnightlyhash

rustnightlyhash computes a git commit hash for a nightly from a given date
in YYYY-MM-HH format.
When given no date, it shows the hash for most recent nightly
(even if the build is still in progress).

## Examples

Commit hash for a specified nightly.
```
$ rustnightlyhash 2015-09-13
fd230ff12481ebeba720fb1ac1f610d93bb74920
```

Commit hash for a current nightly
```
$ date +%F
2015-09-17
$ rustnightlyhash
fc4d566b432d48933e27dd65a973c936b564d6e9
$ rustnightlyhash --output-date
fc4d566b432d48933e27dd65a973c936b564d6e9 2015-09-16
```

## Date handling

Both the input and output dates are considered to be
consistent with builds' start times reported by a buildbot,
which probably are in UTC, so they may differ from the dates in your timezone.

Date-version mapping should be consistent with [the one there](https://static.rust-lang.org/dist/).
