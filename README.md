# ARP - AUR Uploader ![Logo](resources/icons/arp.png)

## Check the [Rules of submission](https://wiki.archlinux.org/title/AUR_submission_guidelines#Rules_of_submission) first before considering uploading a new package

## Description

GUI Application that let's the user upload packages to the AUR

## Package Types

There are different options how the package can be built. Depending on your selection, different sections have to be filled out or are automatically generated.

### Binary

If the project already provides a binary file, this is the easiest option. You just have to provide the binary file and the programm will generate the code for you. The package will be saved under `/usr/bin`.

### Make File

If the project provides a make file, this option will run the make command and you don't have to provide anything else.

### Rust Cargo

If the project is a rust crate, you can select this option to compile the code for you. The package will be saved under `/usr/bin`.

### Custom

If none of the above options match your package, you can provide custom scripts.

## Dependencies

- git
- [SSH key pair](https://wiki.archlinux.org/title/AUR_submission_guidelines#Authentication)
- hicolor-icon-theme
