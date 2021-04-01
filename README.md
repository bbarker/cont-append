# cont-append
Continuous data appender; can be used for safe log retention.

Note that this is not meant to watch extremely large directories
due to constraints like the
[inotify watch limit](https://unix.stackexchange.com/questions/13751/kernel-inotify-watch-limit-reached),
which can be tweaked in the operating system.


## Algorithm Overview

1. Given a directory
2. Watch for events recursively in the directory
3.
  1. If the event is `Create`, (re)initiate tail on the file.
     That is, if we're already tailing, kill the old tail and
     start a new one.
     `TODO`: (Is this necessary after fixing tail)?
  2. If there is a `Delete`, stop the `tail`.
  3. Otherwise, presmuably we have appended data to the file,
     and let `tail` do its thing.