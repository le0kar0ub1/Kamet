# Kamet

`Kamet` is x64 kernel wroten in rust with some VGA games integrated.

# Dependencies

* qemu-system-x86_64 (dev under V5.0.0)
* cargo (dev under V1.44.0)

# Tree

| Path             | Job                                    |
|------------------|----------------------------------------|
| `src`            | kernel entry point & main              |
| `src/drivers`    | host all drivers                       |
| `src/kernel`     | the kernel side setup                  |
| `src/kamet`      | kernel games                           |

# Game list

# Build

`cargo xbuild`

# Run

cargo run the image with `qemu`.

`cargo xrun`

# Epilogue

Feel free to fork, use, improve.