use std::collections::HashSet;

fn construct_left_hashset<'a>(input: &[(&'a str, &'a str)]) -> HashSet<&'a str> {
    let mut hashset = HashSet::new();
    for (k, _) in input {
        hashset.insert(*k);
    }
    hashset
}

fn construct_right_hashset<'a>(input: &[(&'a str, &'a str)]) -> HashSet<&'a str> {
    let mut hashset = HashSet::new();
    for (_, v) in input {
        hashset.insert(*v);
    }
    hashset
}

pub fn construct_start_end<'a>(input: &[(&'a str, &'a str)]) -> Option<(String, String)> {
    let left_hash_set = construct_left_hashset(input);
    let right_hash_set = construct_right_hashset(input);

    // values which are eligible to be starting and ending airport codes
    let mut start = String::new();
    let mut end = String::new();

    for elem in input.iter() {
        if left_hash_set.contains(elem.0) && !right_hash_set.contains(elem.0) {
            if !start.is_empty() {
                return None;
            }
            start = elem.0.to_string();
        }

        if right_hash_set.contains(elem.1) && !left_hash_set.contains(elem.1) {
            if !end.is_empty() {
                return None;
            }
            end = elem.1.to_string();
        }
    }
    if start.is_empty() || end.is_empty() {
        return None;
    }
    Some((start, end))
}

// Tests

#[cfg(test)]
mod tests {

    fn str_arr_to_pairs<'a>(arr: &'a [[&str; 2]]) -> Vec<(&'a str, &'a str)> {
        arr.iter().map(|elem| (elem[0], elem[1])).collect()
    }
    use super::*;
    #[test]
    fn arr_to_pairs_okay() {
        let array = [
            ["IND", "EWR"],
            ["SFO", "ATL"],
            ["GSO", "IND"],
            ["ATL", "GSO"],
        ];
        assert_eq!(str_arr_to_pairs(&array)[0], ("IND", "EWR"));
        assert_eq!(str_arr_to_pairs(&array)[1], ("SFO", "ATL"));
        assert_eq!(str_arr_to_pairs(&array)[2], ("GSO", "IND"));
        assert_eq!(str_arr_to_pairs(&array)[3], ("ATL", "GSO"));
    }

    #[test]
    fn left_hash_map_check() {
        let array = [
            ["IND", "EWR"],
            ["SFO", "ATL"],
            ["GSO", "IND"],
            ["ATL", "GSO"],
        ];
        let elements = str_arr_to_pairs(&array);
        let left_hash_map = construct_left_hashset(&elements);
        assert!(left_hash_map.contains("IND"));
    }

    #[test]
    fn right_hash_map_check() {
        let array = [
            ["IND", "EWR"],
            ["SFO", "ATL"],
            ["GSO", "IND"],
            ["ATL", "GSO"],
        ];
        let elements = str_arr_to_pairs(&array);
        let left_hash_map = construct_right_hashset(&elements);
        assert!(left_hash_map.contains("EWR"));
    }

    #[test]
    fn cyclic_graph_two_cities() {
        let array = [["BEG", "ZRH"], ["ZRH", "BEG"]];
        let elements = str_arr_to_pairs(&array);
        let start_end = construct_start_end(&elements);
        assert_eq!(start_end, None);
    }

    #[test]
    fn cyclic_graph_multiple_cities() {
        let array = [
            ["BEG", "ZRH"],
            ["ZRH", "DOH"],
            ["DOH", "MNL"],
            ["MNL", "BEG"],
        ];
        let elements = str_arr_to_pairs(&array);
        let start_end = construct_start_end(&elements);
        assert_eq!(start_end, None);
    }

    #[test]
    fn working_example() {
        let array = [
            ["IND", "EWR"],
            ["SFO", "ATL"],
            ["GSO", "IND"],
            ["ATL", "GSO"],
        ];
        let elements = str_arr_to_pairs(&array);
        let start_end = construct_start_end(&elements);
        let start_expected = "SFO".to_string();
        let end_expected = "EWR".to_string();
        assert_eq!(start_end, Some((start_expected, end_expected)));
    }

    #[test]
    fn working_example_inner_cycle() {
        let array = [
            ["IND", "EWR"],
            ["BEG", "ZRH"],
            ["ZRH", "BEG"],
            ["SFO", "ATL"],
            ["GSO", "IND"],
            ["ATL", "GSO"],
        ];
        let elements = str_arr_to_pairs(&array);
        let start_end = construct_start_end(&elements);
        let start_expected = "SFO".to_string();
        let end_expected = "EWR".to_string();
        assert_eq!(start_end, Some((start_expected, end_expected)));
    }

    #[test]
    fn second_doc_working_example() {
        let array = [["ATL", "EWR"], ["SFO", "ATL"]];
        let elements = str_arr_to_pairs(&array);
        let start_end = construct_start_end(&elements);
        let start_expected = "SFO".to_string();
        let end_expected = "EWR".to_string();
        assert_eq!(start_end, Some((start_expected, end_expected)));
    }

    #[test]
    fn non_connected_airports_fail() {
        let array = [["BEG", "ZRH"], ["SFO", "ATL"], ["GSO", "IND"]];
        let start_end = construct_start_end(&(str_arr_to_pairs(&array)));
        assert_eq!(start_end, None);
    }
}
