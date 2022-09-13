pub fn filter_map_test(vector: Vec<i32>) -> Vec<i32> {
    return vector
        .into_iter()
        .filter_map(|a| if a % 2 == 0 { Some(a) } else { None })
        .filter_map(|a| if a % 3 == 0 { Some(a) } else { None })
        .filter_map(|a| if a % 4 == 0 { Some(a) } else { None })
        .collect();
}

pub fn collect_test(vector: Vec<i32>) -> Vec<i32> {
    return vector
        .into_iter()
        .collect::<Vec<i32>>()
        .into_iter()
        .collect::<Vec<i32>>()
        .into_iter()
        .collect::<Vec<i32>>();
}

pub fn fold_test(vector: Vec<i32>) -> Vec<i32> {
    return vector
        .into_iter()
        .fold(Vec::new(), |mut acc, a| {
            acc.push(a);
            acc
        })
        .into_iter()
        .fold(Vec::new(), |mut acc, a| {
            acc.push(a);
            acc
        })
        .into_iter()
        .fold(Vec::new(), |mut acc, a| {
            acc.push(a);
            acc
        })
        .into_iter()
        .collect();
}

pub fn try_fold_test(vector: Vec<i32>) -> Result<i32, i32> {
    return vector
        .into_iter()
        .try_fold(0, |acc, a| if a % 2 == 0 { Ok(acc + a) } else { Err(a) });
}

pub fn flatmap_test(vector: Vec<i32>) -> Vec<i32> {
    return vector
        .into_iter()
        .flat_map(|a| [a.pow(2)])
        .flat_map(|a| [a.pow(2)])
        .flat_map(|a| [a.pow(2)])
        .collect();
}

pub fn for_each_test(vector: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    vector.into_iter().for_each(|a| result.push(a));
    return result;
}

pub fn filter_test(vector: Vec<i32>) -> Vec<i32> {
    return vector
        .into_iter()
        .filter(|a| a % 2 == 0)
        .filter(|a| a % 3 == 0)
        .filter(|a| a % 4 == 0)
        .collect();
}

pub fn find_test(vector: Vec<i32>) -> Vec<i32> {
    return vector
        .into_iter()
        .find(|a| a % 2 == 0)
        .into_iter()
        .find(|a| a % 3 == 0)
        .into_iter()
        .find(|a| a % 4 == 0)
        .into_iter()
        .collect();
}

pub fn cycle_test(vector: Vec<i32>) -> Vec<i32> {
    return vector.into_iter().cycle().cycle().cycle().take(1).collect();
}

pub fn max_by_test(vector: Vec<i32>) -> Vec<i32> {
    return vector
        .into_iter()
        .max_by(|a, b| a.cmp(b))
        .into_iter()
        .max_by(|a, b| a.cmp(b))
        .into_iter()
        .max_by(|a, b| a.cmp(b))
        .into_iter()
        .collect();
}

pub fn map_test(vector: Vec<i32>) -> Vec<i32> {
    return vector
        .into_iter()
        .map(|a| a * 2)
        .map(|a| a * 3)
        .map(|a| a * 4)
        .collect();
}

pub fn rposition_test(vector: Vec<i32>) -> Option<usize> {
    return vector.into_iter().rposition(|a| a % 3 == 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_test() {
        let vector = vec![2, 5, 7];
        let result = collect_test(vector);
        assert_eq!(result, vec![2, 5, 7]);
    }

    #[test]
    fn test_map_test() {
        let vector = vec![6, 12, 17];
        let result = map_test(vector);
        assert_eq!(result, vec![144, 288, 408]);
    }

    #[test]
    fn flatmap_test_() {
        let result = flatmap_test(vec![1, 2, 3, 4]);
        assert_eq!(result, vec![1, 256, 6561, 65536]);
    }

    #[test]
    fn for_each_test_() {
        let result = for_each_test(vec![3, 6, 12]);
        assert_eq!(result, vec![3, 6, 12]);
    }

    #[test]
    fn filter_test_() {
        let result = filter_test(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ]);
        assert_eq!(result, vec![12]);
    }

    #[test]
    fn find_test_() {
        let result = find_test(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        assert_eq!(result, vec![]);
    }
    #[test]
    fn filter_map_test_() {
        let result = filter_map_test(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ]);
        assert_eq!(result, vec![12]);
    }

    #[test]
    fn fold_test_() {
        let result = fold_test(vec![5, 8, 9, 11, 14]);
        assert_eq!(result, vec![5, 8, 9, 11, 14]);
    }

    #[test]
    fn cycle_test_() {
        let result = cycle_test(vec![1, 2, 3]);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn try_fold_test_() {
        let result = try_fold_test(vec![1, 2, 3, 4]);
        assert_eq!(result, Err(1));
    }

    #[test]

    fn max_by_test_() {
        let result = max_by_test(vec![7, 14, 67, 195]);
        assert_eq!(result, vec![195]);
    }
    #[test]
    fn rposition_test_() {
        let result = rposition_test(vec![1, 2, 3, 4]);
        assert_eq!(result, Some(2));
    }
}
