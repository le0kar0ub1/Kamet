# Kamet

`Kamet` is a set of early kernel games.

All the games are running with `VGA`. All the project is wroten `rust`.

# Dependencies

* qemu
* cargo
* rustc

# Tree

| Path             | Job                                    |
|------------------|----------------------------------------|
| `src`            | kernel entry point & main              |
| `src/drivers`    | host all drivers we need               |
| `src/kernel`     | the kernel side that we must setup     |
| `src/kamet`      | kernel games                           |

# Game list

# Build

`cargo xbuild`

# Run

cargo run the image with `qemu`.

`cargo xrun`

# Epilogue

Feel free to fork, use, improve.