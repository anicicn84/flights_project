# README Nikola Anicic
## _Flight project_

## Cases
- [['BEG', 'ZRH'], ['ZRH', 'BEG']] can be indesicive - return inconclusive result
- [['BEG', 'ZRH'], ['ZRH', 'MNL'], ['MNL', 'BEG']] is okay, but cyclic graph, result: ['BEG', 'BEG']
- Normal test case [['IND', 'EWR'], ['SFO', 'ATL'], ['GSO', 'IND'], ['ATL', 'GSO']] returns ['SFO', 'EWR']

## How to approach
> Use left and right hash set. If the value is found in the left then the right value needs to exist of the same string for the stop to be considered layover. Except in the 1st and 2nd case above where we need to check their pairs.
- To be inconclusive, all values will have equal lefts and rights in hash sets, constructing the cyclic graph. We need a value as a starting point and one as an ending point, meaning one airport must have left count > right count (starting destination), and one needs to have right count > left count (ending destination)
- if the list is cyclic then we are unable to tell the starting and ending station, for ex: [["BEG", "DOH"], ["DOH", "MNL"], ["MNL", "BEG"]] since all 3 candidates might be starting or ending airports.


## Hash sets
> Left hash set will have left pair value, while right hash set will have the right pair value.
> Example: ("BEG", "ZRH") => left hash map set: "BEG", right hash set: "ZRH"