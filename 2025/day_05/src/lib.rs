use std::ops::RangeInclusive;

pub fn squash_intervals(mut intervals: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    intervals.sort_by_key(|x| *x.start());

    let mut squashed = Vec::<RangeInclusive<u64>>::new();
    for interval in intervals {
        if let Some(last_interval) = squashed.last_mut()
            && is_overlapping(&interval, last_interval)
        {
            *last_interval = *last_interval.start()..=*interval.end().max(last_interval.end());
        } else {
            squashed.push(interval);
        }
    }

    squashed
}

fn is_overlapping<T: Ord>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

pub fn count_members(intervals: &Vec<RangeInclusive<u64>>, values: &Vec<u64>) -> u64 {
    let mut total = 0;
    for value in values {
        for interval in intervals {
            if interval.contains(value) {
                total += 1;
                break;
            }
        }
    }

    total
}

// Must be squashed intervals
pub fn count_possible_intervals(intervals: &Vec<RangeInclusive<u64>>) -> u64 {
    let mut total = 0;
    for interval in intervals {
        total += interval.end() - interval.start() + 1
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_squashing() {
        let intervals = vec![1..=5, 7..=10];
        let overlapping_intervals = vec![3..=8, 1..=5, 7..=10];
        assert_eq!(intervals, squash_intervals(intervals.clone()));
        assert_eq!(
            vec![1..=10],
            squash_intervals(overlapping_intervals.clone())
        );
    }

    #[test]
    fn check() {
        let intervals = vec![3..=5, 10..=14, 16..=20, 12..=18];
        let values = vec![1, 5, 8, 11, 17, 32];

        assert_eq!(count_members(&intervals, &values), 3);
        assert_eq!(count_possible_intervals(&squash_intervals(intervals)), 14);
    }
}
