use std::{fmt::Debug, collections::HashMap, hash::Hash};





pub fn repeat<T: Debug, K: Eq + Hash>(
    mut state: T,
    mut next: impl FnMut(T) -> T,
    mut key_fn: impl FnMut(&T) -> K,
    n: usize
) -> T {
    let mut hist: HashMap<K, usize> = HashMap::new();
    for i in 0..n {
        let key = key_fn(&state);
        if let Some(offset) = hist.get(&key) {
            let period = i - offset;
            let n_left = (n - i) % period;
            for _ in 0..n_left {
                state = next(state);
            }
            return state;
        } else {
            hist.insert(key, i);
            state = next(state);
        }
    }
    state
}


