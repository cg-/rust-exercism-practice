#[derive(Debug)]
pub struct HighScores{
    scores: Vec<u32>,
}

impl HighScores{
    pub fn new(scores: &[u32]) -> Self {
        return HighScores{
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores[..]
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut temp = self.scores.clone();
        temp.sort();
        temp.last().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut temp = self.scores.clone();
        temp.sort();
        temp.reverse();
        match temp.len() {
            0 => Vec::new(),
            1 | 2 => {
                temp
            },
            _ => {
                temp[..=2].to_vec()
            },
        }
    }
}
