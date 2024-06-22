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
            1 => temp.first_chunk::<1>().expect("missing 3 scores").to_vec(),
            2 => temp.first_chunk::<2>().expect("missing 3 scores").to_vec(),
            _ => temp.first_chunk::<3>().expect("missing 3 scores").to_vec(),
        }
    }
}
