# cont-append
Continuous data appender; can be used for safe log retention.

Note that this is not meant to watch extremely large directories
due to constraints like the
[inotify watch limit](https://unix.stackexchange.com/questions/13751/kernel-inotify-watch-limit-reached),
which can be tweaked in the operating system.

We may add an option to use a 
[polling based watcher](https://docs.rs/notify/5.0.0-pre.6/notify/poll/struct.PollWatcher.html)
in various capacities. It could be run in addition to
the inotify-based watcher at some interval
to avoid [missed events](https://stackoverflow.com/questions/239988/filesystemwatcher-vs-polling-to-watch-for-file-changes),
or it could be used instead of the inotify watcher
in cases where inotify is not suitable.


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