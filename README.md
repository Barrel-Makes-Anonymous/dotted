# .dotted
###### Simple config management.

## Building
Clone this repo, navigate into it and run `cargo build --release`. The binary will be located inside the repo at `target/release/dotted` You can then put the binary somewhere on your `$PATH` so that you can run it from the command line. You can find cargo [here](https://www.rust-lang.org/tools/install) or through your package manager. I will eventually put up binaries when I consider this project "stable."
## Usage
The basic concept of dotted is that you create packages which hold files and where those files should be on the system. You can then enable these packages by either symlinking or copying them to their destinations. Packages can contain files that are not tracked by dotted, so you can easily use git or other tools to manage their contents.
#### Options
* `dotted -a <files> [-d <destinations>] -p <packages>`

    add files listed to packages listed. If no destinations are specified, dotted will record the original location of each file.
    
* `dotted -r <files> -p <packages>` 

    remove files listed in packages listed
    
* `dotted -m <files> -d <destinations> -p <packages>` 

    move the each file listed in the packages listed to a new destination.
    
* `dotted -es <packages>`

    enable the packages listed via symlinking

* `dotted -ec <packages>`

    enable the packages listed via copying

* `dotted -E <packages>`

    disable the packages listed

* `dotted -R <packages>`

    disable and then delete the packages listed

* `dotted -i <files>`

    move the packages located at the paths listed into the local data dir (`~/.local/share/dotted` on Linux) so that they can be referred to by name instead of by their full path.
