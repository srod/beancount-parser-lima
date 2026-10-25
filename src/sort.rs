/// Sort `items` by `keyfn`, keeping items with equal keys in their original order.
///
/// Only the keys and positions are sorted, and the items are then moved into place by swaps, so
/// no second copy of the items is made. A merge sort of the items themselves would hold one, and
/// a directive is hundreds of bytes.
pub fn sort_stable_in_place<V, K, F>(items: &mut [V], keyfn: F)
where
    K: Ord,
    F: Fn(&V) -> K,
{
    let mut order: Vec<(K, usize)> = items.iter().enumerate().map(|(i, v)| (keyfn(v), i)).collect();
    // the position makes equal keys come out in their original order
    order.sort_unstable();

    // from[i] is where the item that belongs at i is now. Follow each cycle of that permutation.
    let mut from: Vec<usize> = order.into_iter().map(|(_, i)| i).collect();
    for start in 0..from.len() {
        let mut at = start;
        while from[at] != start {
            let next = from[at];
            items.swap(at, next);
            from[at] = at;
            at = next;
        }
        from[at] = at;
    }
}

mod tests;
