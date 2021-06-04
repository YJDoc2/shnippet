# Shnippet

---

A minimal commandline shell snippet manager. [With a minimal readme.]

### What is this?

Sometimes we have to do some quick command execution which we search off the net, and even though we use it many times, we don't want to create a script for it, manage the script folder, put it on path etc. Shnippet is a commandline manager which does all that for you, and you only have to give a name, description and the command/script. It can also execute a shnippet with a command.
PRs and issues are welcome.

```C
USAGE:
    shnippet [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    delete  <name>    Delete an existing shnippet
    edit    <name>    Edit an existing shnippet
    exec    <name>    Run a shnippet in shell
    help              Prints this message or the help of the given subcommand(s)
    list              List all shnippets
    new               Add new shnippet
```

### Name

It is a SHell sNIPPET manager : Shnippet.
Why this? Because I was lazy when finding name, and shell+snippet = shnippet.

---

### Licence

Shnippet is distributed under the terms of the MIT license.Opening a pull requests is assumed to signal agreement with these licensing terms.
