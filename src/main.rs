use flights::construct_start_end;
use flights::str_arr_to_pairs;

fn main() {
    let array = [
        ["IND", "EWR"],
        ["SFO", "ATL"],
        ["GSO", "IND"],
        ["ATL", "GSO"],
    ];
    let elements = str_arr_to_pairs(&array);
    let start_end = construct_start_end(&elements);

    if let Some(val) = start_end {
        println!("{:?}", val);
    } else {
        println!("Indecisive");
    }
}
