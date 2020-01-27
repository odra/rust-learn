struct Card {
    score: u32
}

impl Card {
    pub fn build(score: u32) -> CardBuilder {
        CardBuilder {
            score: Some(score)
        }
    }
}

struct CardBuilder {
    score: Option<u32>
}

impl CardBuilder {
    pub fn new(score: u32) -> Self {
        CardBuilder {
            score: Some(score)
        }
    }

    pub fn score(mut self, value: u32) -> Self {
        self.score = Some(value);

        self
    }

    pub fn build(self) -> Card {
        Card {
            score: self.score.unwrap_or(0)
        }
    }
}

impl Default for CardBuilder {
    fn default() -> Self {
        CardBuilder {
            score: Some(50),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cardbuilder() {
        let builder = CardBuilder::new(10);
        let card = builder.build();
        assert_eq!(10, card.score);

        let builder = CardBuilder::default();
        let card = builder.build();
        assert_eq!(50, card.score);
    }
}