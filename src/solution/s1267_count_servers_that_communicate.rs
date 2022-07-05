pub struct Solution {}

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        let mut row: Vec<i32> = vec![0; grid.len()];
        let mut col: Vec<i32> = vec![0; grid[0].len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    row[i] += 1;
                    col[j] += 1;
                    sum += 1;
                }
            }
        }

        for i in 0..row.len() {
            if row[i] == 1 {
                for j in 0..col.len() {
                    if grid[i][j] == 1 {
                        if col[j] == 1 {
                            // println!("{} {}", i, j);
                            count += 1;
                            break;
                        }
                    }
                }
            }
        }

        // println!("sum: {}", sum);
        // println!("row: {:?}", row);
        // println!("col: {:?}", col);
        // println!("count: {}", count);
        // println!("final: {}", sum - count);

        sum - count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1267() {
        // Solution::count_servers(vec![
        //     vec![1, 1, 0, 0],
        //     vec![0, 0, 1, 0],
        //     vec![0, 0, 1, 0],
        //     vec![0, 0, 0, 1],
        // ]);
        assert_eq!(0, Solution::count_servers(vec![vec![1, 0], vec![0, 1]]));
        assert_eq!(3, Solution::count_servers(vec![vec![1, 0], vec![1, 1]]));
        assert_eq!(
            4,
            Solution::count_servers(vec![
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1]
            ])
        );
    }
}
