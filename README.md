# Firo Parser

Parser for the ".firo" file format. Firo is a domain specific language for the [file-rover cli tool.](https://github.com/evccyr/file-rover).

## Origin file(origin.firo)
Origin file specifies the origin paths of the files that need to be moved, renamed or removed.
Example:
```firo
/home/evccyr/file.rs
plugin/file
folder/name with space.txt
```

## Destination file(destination.firo)
Destination file is the file that specifies the destination paths for the file paths specified in the "origin.firo" file.
Example:
```firo
/home/evccyr/file <date> .rs
plugin/calendar <!> 2024.pdf
<!> /tmp/something.txt
```

## Tokens
The fundamental tokens for ".firo" files are path_part(ex: `/home/evccyr/file`) and pins(`<date>`).
