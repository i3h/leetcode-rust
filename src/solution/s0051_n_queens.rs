pub struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = Vec::new();

        Solution::backtrack(&mut res, &mut vec![], n, 0);

        res
    }

    fn backtrack(res: &mut Vec<Vec<String>>, cur: &mut Vec<String>, n: i32, row: i32) {
        if row == n {
            res.push(cur.clone());
            return;
        }

        for col in 0..n {
            if Solution::is_valid(cur, row, col) {
                let mut s = String::new();
                for i in 0..n {
                    if i == row {
                        s.push('Q');
                    } else {
                        s.push('.');
                    }
                }
                cur.push(s);
                Solution::backtrack(res, cur, n, row + 1);
                cur.pop();
            }
        }
    }

    fn is_valid(cur: &Vec<String>, row: i32, col: i32) -> bool {
        for i in 0..row {
            if cur[i as usize].chars().nth(col as usize).unwrap() == 'Q' {
                return false;
            }
        }

        let mut row = row;
        let mut col = col;
        while row >= 0 && col >= 0 {
            if cur[row as usize].chars().nth(col as usize).unwrap() == 'Q' {
                return false;
            }
            row -= 1;
            col -= 1;
        }

        let mut row = row;
        let mut col = col;
        while row >= 0 && col < cur[0].len() as i32 {
            if cur[row as usize].chars().nth(col as usize).unwrap() == 'Q' {
                return false;
            }
            row -= 1;
            col += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_51() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."],
            ]
        );
        assert_eq!(Solution::solve_n_queens(8).len(), 92);
    }
}
