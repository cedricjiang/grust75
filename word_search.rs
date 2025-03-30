impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let word: Vec<char> = word.chars().collect();

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::exist_from(&mut board, &word, i, j) {
                    return true;
                }
            }
        }

        false
    }

    fn exist_from(board: &mut Vec<Vec<char>>, word: &[char], x: usize, y: usize) -> bool {
        let saved = board[x][y];

        if word[0] != saved {
            return false;
        }

        board[x][y] = ' ';

        if word.len() == 1 {
            return true;
        }

        if x > 0 && Self::exist_from(board, &word[1..], x - 1, y) {
            return true;
        }

        if y > 0 && Self::exist_from(board, &word[1..], x, y - 1) {
            return true;
        }

        if x < board.len() - 1 && Self::exist_from(board, &word[1..], x + 1, y) {
            return true;
        }

        if y < board[x].len() - 1 && Self::exist_from(board, &word[1..], x, y + 1) {
            return true;
        }

        board[x][y] = saved;
        false
    }
}
