use std::collections::VecDeque;

mod macros;

#[derive(Debug)]
pub struct Queue<'a> {
    id: u32,
    game: String,
    players: u8,
    queue: VecDeque<&'a str>,
}

impl<'a> PartialEq for Queue<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> Queue<'a> {
    fn new(id: u32, game: &str, players: u8, queue: VecDeque<&'a str>) -> Self {
        Self {
            id,
            game: game.to_owned(),
            players,
            queue,
        }
    }

    fn append(&mut self, player: &'a str) {
        self.queue.push_back(player);
    }

    fn next(&mut self) -> Vec<&'a str> {
        let mut result = Vec::new();
        for i in 0..self.players {
            if let Some(p) = self.queue.pop_front() {
                result.push(p)
            }
        }
        result
    }
}

#[cfg(test)]
mod queue_tests {
    use crate::*;
    #[test]
    fn test_new() {
        assert_eq!(
            Queue::new(0, "maimai", 2, vecdeque!["test"]),
            Queue {
                id: 0,
                game: "maimai".to_owned(),
                players: 2,
                queue: vecdeque!["test"],
            }
        );
    }

    #[test]
    fn test_append() {
        let mut queue = Queue::new(0, "maimai", 2, vecdeque!["player1", "player2"]);
        queue.append("OriginCode");
        assert_eq!(
            queue,
            Queue {
                id: 0,
                game: "maimai".to_owned(),
                players: 2,
                queue: vecdeque!["player1", "player2", "OriginCode"],
            }
        );
    }

    #[test]
    fn test_next() {
        let mut queue = Queue::new(0, "maimai", 2, vecdeque!["player1"]);
        queue.next();
        assert_eq!(
            queue,
            Queue {
                id: 0,
                game: "maimai".to_owned(),
                players: 2,
                queue: VecDeque::new(),
            }
        );
    }
}
