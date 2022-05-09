# Copy-pst
Two simple cli tools to copy and paste files/directories. This allows you to separate the copying and pasting actions in the command line and do them accross different terminals

# Install
Install both binaries:
Compile (`cd <copy or pst> && cargo build --release`) and either sym-link or move the binaries in `$PWD/<copy or pst>/target/release/<copy or pst>` into somewhere in your path

# Usage
The copy tool copies the absolute path of the directory/file listed into your clipboard and the pst command then runs `cp` for you to paste the directory in a location
```
copy target/
cd ~/Documents
pst ./
```
