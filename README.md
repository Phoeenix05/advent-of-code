# advent of code

this a working answer for the first part of day 3

```sh
rg 'mul\([0-9]{1,3},[0-9]{1,3}\)' src/input.txt -o | rg '[0-9]+,[0-9]+' -o | awk -F ':' '{print $1;}' | awk -F ',' '{sum+=$1*$2} END {print sum}'
```
