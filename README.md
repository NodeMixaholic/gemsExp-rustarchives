# <img src="gems.png" alt="*GEMS Logo*" width="5%"/> gemsExp
# Deprecation Notice
Instead of starting over, I have decided to just fork GEMS NT and make that the next version of GEMS.

## The future of GEMS is finally here.
GEMS seems built different. It now uses Rustlang!
"Remaking 99% of the functionality in GEMS Classic on the first night of development was suprisingly easy." -- Sparksammy

## Important notices!

In order to contribute, you must follow the [Sparksammy C.O.C.](https://github.com/sparksammy/sparksammycoc)

# Requirements
1. Install rust nightly using rustup
2. Run "rustup component add llvm-tools-preview" (no quotes)

# Tutorial

1. Create a new directory with a decent name "example: gems-exp-build"
2. Put my fork of "bootloader" by rust-osdev ( https://github.com/sparksammy/bootloader ) into the directory (keep it in it's folder and the name of the folder!)
3. Put this project in there too (again, keep it in the folder it comes with and it's name!)
4. cd into "gemsExp-main"
5. run compile.sh
6. if things break, try doing what the mythically magic terminal tells you to do.
7. and then repeat steps 5 and 6 until it outputs something like "Created bootable disk image at bootloader-main/target/x86_64-bootloader/release/boot-bios-GEMS-EXPERIENCE.img"
8. Binaries are compiled to the folder "bootloader-main/target/x86_64-bootloader/release/"
