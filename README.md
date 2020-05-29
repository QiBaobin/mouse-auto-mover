# mouse-auto-mover

Automatically move mouse to avoid screen saver

Currently the tools will move the mouse location by 5px offset per 60 seconds.

``` sh
mouse mover 0.1.1
Mouse mover usage.

USAGE:
    mouse-mover [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --distance <distance>    set the distance delta when we want to move [default: 5]
    -i, --interval <interval>    set the time interval in seconds, how often we run [default: 60]
```

# install

* Just download latest release from https://github.com/QiBaobin/mouse-auto-mover/releases

* on mac
```sh
chmod +x mouse-mover-mac
```

# run

* on mac
```sh
mouse-mover-mac # or DIR/mouse-mover-mac if it's not in path
```

* on windows
double click to run

# stop

* Just use ctrl-c to stop the process

* on windows, closing the window also works

