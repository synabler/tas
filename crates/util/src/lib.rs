use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    hash::Hash,
};

/// Performs a breadth-first search and returns the shortest path.
///
/// Parameters:
/// - `starts`: list of starting states
/// - `neighbors`: function that takes a state and returns the list
///   of possible next states, along with a symbol to use in the output
/// - `is_goal`: function that determines whether a state is a goal state
pub fn bfs<T: Debug + Hash + Eq + Clone, SYMBOL>(
    starts: &[T],
    neighbors: impl Fn(&T) -> Vec<(SYMBOL, T)>,
    is_goal: impl Fn(&T) -> bool,
) -> Vec<SYMBOL> {
    let mut prev_map = HashMap::<T, (SYMBOL, T)>::new();
    let mut queue = VecDeque::from_iter(starts.iter().cloned());

    let mut finish_state: Option<T> = None;
    while let Some(p) = queue.pop_front() {
        for (sym, q) in neighbors(&p) {
            if prev_map.contains_key(&q) || starts.contains(&q) {
                continue;
            }
            if is_goal(&q) {
                finish_state = Some(q.clone());
            }
            prev_map.insert(q.clone(), (sym, p.clone()));
            queue.push_back(q);

            if finish_state.is_some() {
                break;
            }
        }
        if finish_state.is_some() {
            break;
        }
    }
    let mut state = finish_state.unwrap();
    let mut solution = vec![];
    while !starts.contains(&state) {
        let (symbol, prev_state) = prev_map.remove(&state).unwrap();
        solution.push(symbol);
        state = prev_state.clone();
    }

    solution.reverse();
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_sample() {
        let result = bfs(&[1], |x| vec![("*2", x * 2), ("+1", x + 1)], |x| *x == 100);
        assert_eq!(result.len(), 8);
        // first symbol can be anything
        assert_eq!(&result[1..], &["+1", "*2", "*2", "*2", "+1", "*2", "*2"]);
    }
}
