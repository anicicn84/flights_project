# README
## _Flight project_

Returns the starting and ending destination from the list of possible fligts passed on in a json format from some request and returns a response in a form of a pair (start, end) if it can find it, otherwise and error.

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

## Running the code
> To run the main.rs file just in the root directory run the command `cargo run` which will build the project first, and then run it. If you just want to build run `cargo build`.

## Testing
> In order to run tests just run the command `cargo test`.
> 
> You can take a look at tests and edge-cases, and probably add some more examples to test them out.


# Microservice API

> The whole json file example with request and response is given in the file `req_resp_json_example.json`. You can run the server/microservice with cargo run and execure curl.sh script from console (shell script) and change curl params there in order to test it. 
The example is given from the doc with requirements (google doc). When sending the cURL request (calling curl.sh from console) you will get back either Success with pair of starting and ending airport short name, or Failure with the reason behind it. 
The reason is simple at the moment, but it can be further improved.
One improvement would be to use HTTP codes for bad cURL requests, warp library I have used has some limitations around it and newer versions of it changed some stuff, maybe it would be better to use Actix library instead of warp ([Actix](https://actix.rs/)) to get some better return codes.
