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

    #[inline]
    fn append(&mut self, player: &'a str) {
        self.queue.push_back(player);
    }
    
    #[inline]
    fn nextone(&mut self) -> Option<&'a str> {
        self.queue.pop_front()
    }

    fn next(&mut self) -> Vec<&'a str> {
        let mut result = Vec::new();
        for _ in 0..self.players {
            if let Some(p) = self.nextone() {
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
    fn test_nextone() {
        let mut queue = Queue::new(0, "maimai", 2, vecdeque!["player1"]);
        assert_eq!(queue.nextone(), Some("player1"));
        assert_eq!(queue.nextone(), None);
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

    #[test]
    fn test_next() {
        let mut queue = Queue::new(0, "maimai", 2, vecdeque!["player1", "player2", "player3"]);
        assert_eq!(queue.next(), vec!["player1", "player2"]);
        assert_eq!(queue.next(), vec!["player3"]);
        assert_eq!(queue.next(), Vec::<&str>::new());
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
