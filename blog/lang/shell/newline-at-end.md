# Overview

Should I put a newline at the end of a file?

## Yes

It makes it easier to view the last line in an editor: the cursor will be not at the end.

## No

When running `cat` on the file, an extra newline will get printed out.

# Conclusion

Yes a newline should be put in.  
`cat` is for command-line viewing of files only.  
I would prioritise the experience when I'm editing files.
