# **Scannner**
A CLI app that adds more specificity to blockchain searching - yeilding transactions on Etherscan for multiple given parameters.

## ***Position of arguments is imperative***

- etherscanner `<ADDRESS> <BLOCKNUMBER> <BLOCKNUMBER> <CONTRACT ADDRESS> <VALUE THRESHHOLD>`

#### Added commands:
- Use `-h` to list the necessary order of arguments
- Use `""` or `''` to leave a specific argument blank
- After inputting args use `> "desired_filename.json"` to save output from scan
i.e. `etherscanner <ADDRESS> <BLOCKNUMBER> <BLOCKNUMBER> <CONTRACT ADDRESS> <VALUE THRESHHOLD> > filenamehere.json`
<sub>this will save your search in a file name output.json in the present working directory..</sub>

##### Search payload for specific fields using `| grep`
i.e. `etherscanner <ADDRESS> <BLOCKNUMBER> <BLOCKNUMBER> <CONTRACT ADDRESS> [VALUE THRESHHOLD] | grep value` or
i.e. `etherscanner <ADDRESS> <BLOCKNUMBER> <BLOCKNUMBER> <CONTRACT ADDRESS> [VALUE THRESHHOLD] | grep from`

- prints all values to terminal
