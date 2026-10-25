#![cfg(test)]

use super::*;
use proptest::prelude::*;
use test_case::test_case;

#[test_case(vec![(2, 'a'), (2, 'b'), (3, 'c'), (1, 'd'), (3, 'e')], vec![(1, 'd'), (2, 'a'), (2, 'b'), (3, 'c'), (3, 'e')])]
fn test_sort_stable_in_place_manually(mut unsorted: Vec<(u8, char)>, expected: Vec<(u8, char)>) {
    sort_stable_in_place(&mut unsorted, |(i, _)| *i);

    assert_eq!(unsorted, expected);
}

proptest! {
    // the same result as the standard library's stable sort
    #[test]
    fn test_sort_stable_in_place(unsorted in proptest::collection::vec((0..10u8, any::<i32>()), 0..200)) {
        let mut sorted = unsorted.clone();
        sort_stable_in_place(&mut sorted, |(k, _)| *k);

        let mut expected = unsorted;
        expected.sort_by_key(|(k, _)| *k);

        assert_eq!(sorted, expected);
    }
}
