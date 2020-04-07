# .dotted
###### Simple config management.

## Building
Clone this repo, navigate into it and run `cargo build --release`. The binary will be located inside the repo at `target/release/dotted` You can then put the binary somewhere on your `$PATH` so that you can run it from the command line. You can find cargo [here](https://www.rust-lang.org/tools/install) or through your package manager. I will eventually put up binaries when I consider this project "stable."
## Usage
The basic concept of dotted is that you create packages which hold files and where those files should be on the system. You can then enable these packages by either symlinking or copying them to their destinations. Packages can contain files that are not tracked by dotted, so you can easily use git or other tools to manage their contents.
#### Options
* `dotted -a <files> [-d <destinations>] -p <packages>`
    * add files listed to packages listed. If no destinations are specified, dotted will record the original location of each file.
* `dotted -r <files> -p <packages>` 
    * remove files listed in packages listed
* `dotted -m <files> -d <destinations> -p <packages>` 
    * move each file listed in the packages listed to the new destinations listed.
* `dotted -es <packages>`
    * enable the packages listed via symlinking
* `dotted -ec <packages>`
    * enable the packages listed via copying
* `dotted -E <packages>`
    * disable the packages listed
* `dotted -R <packages>`
    * disable and then delete the packages listed
* `dotted -i <files>`
    * move the packages located at the paths listed into the local data dir (`~/.local/share/dotted` on Linux) so that they can be referred to by name instead of by their full path.
    
#### Example Usage
   Here is an example scenario in which dotted could be used:
   
   `dotted -a ~/.vimrc ~/.vim ~/.config/nvim/init.vim -p vim_configs`
   
   Adds some vim configuration files into a new package called "vim_configs." Packages are created when files are added to a package that does not already exist.
   
   `dotted -es vim_configs`
   
   Enables the package, "vim_configs" with symlinking. If the destinations of files within the package already exist, dotted will ask for permission before overwriting them. If I decided that I want to try out some other vim configs, I can safely disable my current ones without worrying about deleting them for good using
   
   `dotted -E vim_configs`
   
   This will remove any of the files that came from the "vim_configs" package without touching anything else.
