# parsed_to_plot


## Overview

This software plots constituency tress and dependency trees given in strings, using both the id-tree crate
[id-tree](https://crates.io/crates/id_tree) and [plotters](https://crates.io/crates/plotters).
The program was written with linguistic syntax in mind, but can work on any input (such as mathematical expressions etc).

* The API expects a string input. Multiple inputs can also be delivered via the command-line in a file.
* For constituency trees, the program takes one-liners, parsed strings. These strings can be syntactic, for example
such that represent phrases and part-of-speech (like the structure of [Berkeley Neural Parser](https://pypi.org/project/benepar/)
in python). Such strings will have "double leaves" (see an example below). Alternatively, the strings can be regular,
representing for example mathematical expressions.
* For dependency trees, the programs takes a conll format, in which every token has 10 fields, separated by tab, and
presented in a new line. Sentences are separated by an empty line. (see an example below using the output of
[spaCy](https://spacy.io/) in python).
* For multiple inputs, the program will expect 3 arguments from the command line : input type (constituency or dependency),
input file path, output path. The inputs will be of the same type. See an example below.
* The API first transforms the input to an internal conll / tree, then plots the structure with recursion. It is mostly
suitable for short sentences of up to 15-20 tokens.



