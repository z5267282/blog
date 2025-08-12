# Overview

Should newlines be put at the end of a file?

## Vim

Using `vim`, an ending lone newline will not be shown.  
Hence if you add an extra one in, there will be two lone newlines at the end of the file.

## Yes

It makes it easier to view the last line in an editor: the cursor will be not at the end.

## No

When running `cat` on the file, an extra newline will get printed out.

# Conclusion

Yes a newline should be put in.  
The `cat` command is for convenience viewing of files only.
