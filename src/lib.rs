//! A player queue for arcade games
//!
//! # Examples
//!
//! You can create a queue by using the associated function `Queue::new()`.
//!
//! ```
//! use arcade_queue::Queue;
//! 
//! let q = Queue::new("", 1).unwrap();
//! ```

use std::{collections::VecDeque, fmt};

mod macros;

#[derive(PartialEq, Debug)]
pub struct Queue<'a> {
    game: &'a str,
    players: u8,
    queue: VecDeque<&'a str>,
}

#[derive(Debug)]
pub enum Error {
    TooLessPlayersError(String),
}

impl<'a> fmt::Display for Queue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} player(s) each round): {}",
            self.game, self.players, self.format_queue()
        )
    }
}

impl<'a> Queue<'a> {
    /// Creates an empty queue with a game name and a number of players for each round.
    /// 
    /// `players` cannot be less than 1.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use arcade_queue::Queue;
    /// 
    /// let q = Queue::new("", 1).unwrap();
    /// ```
    pub fn new(game: &'a str, players: u8) -> Result<Self, Error> {
        if players == 0 {
            return Err(Error::TooLessPlayersError(
                "`players` should be at least 1".to_owned(),
            ));
        }
        Ok(Self {
            game,
            players,
            queue: VecDeque::new(),
        })
    }

    /// Appends a player to the end of the queue.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use arcade_queue::Queue;
    /// 
    /// let mut q = Queue::new("", 1).unwrap();
    /// 
    /// q.append("player1");
    /// q.append("player2");
    /// 
    /// assert_eq!(q.format_queue(), "player1 player2");
    /// ```
    #[inline]
    pub fn append(&mut self, player: &'a str) {
        self.queue.push_back(player);
    }

    /// Yields the next one player.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use arcade_queue::Queue;
    /// 
    /// let mut q = Queue::new("", 1).unwrap();
    /// 
    /// q.append("player1");
    /// q.append("player2");
    /// 
    /// assert_eq!(q.nextone().unwrap(), "player1");
    /// ```
    #[inline]
    pub fn nextone(&mut self) -> Option<&'a str> {
        self.queue.pop_front()
    }

    /// Yields the next group of players.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use arcade_queue::Queue;
    /// let mut q = Queue::new("", 2).unwrap();
    /// 
    /// q.append("player1");
    /// q.append("player2");
    /// q.append("player3");
    /// 
    /// assert_eq!(q.next(), vec!["player1", "player2"]);
    /// ```
    pub fn next(&mut self) -> Vec<&'a str> {
        let mut result = Vec::new();
        for _ in 0..self.players {
            if let Some(p) = self.nextone() {
                result.push(p);
            }
        }
        result
    }

    /// Yields the next group of players and push them back to the queue.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use arcade_queue::Queue;
    /// let mut q = Queue::new("", 2).unwrap();
    /// 
    /// q.append("player1");
    /// q.append("player2");
    /// q.append("player3");
    /// 
    /// assert_eq!(q.next_to_back(), vec!["player1", "player2"]);
    /// assert_eq!(q.get_queue(), vec!["player3", "player1", "player2"]);
    /// ```
    pub fn next_to_back(&mut self) -> Vec<&'a str> {
        let mut result = Vec::new();
        for _ in 0..self.players {
            if let Some(p) = self.nextone() {
                result.push(p);
                self.append(p);
            }
        }
        result
    }

    /// Get the current queue.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use arcade_queue::Queue;
    /// 
    /// let mut q = Queue::new("", 1).unwrap();
    /// 
    /// q.append("player1");
    /// q.append("player2");
    /// q.append("player3");
    /// 
    /// assert_eq!(q.get_queue(), vec!["player1", "player2", "player3"])
    /// ```
    pub fn get_queue(&self) -> Vec<&'a str> {
        self.queue.clone().into()
    }

    /// Get the current formatted queue.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use arcade_queue::Queue;
    /// 
    /// let mut q = Queue::new("", 1).unwrap();
    /// 
    /// q.append("player1");
    /// q.append("player2");
    /// q.append("player3");
    /// 
    /// assert_eq!(q.format_queue(), "player1 player2 player3")
    /// ```
    pub fn format_queue(&self) -> String {
        let mut s = String::new();
        for p in self.queue.iter() {
            s.push_str(p);
            s.push(' ');
        }
        s.pop();
        s
    }
}

#[cfg(test)]
mod queue_tests {
    use crate::*;

    #[test]
    fn test_append() -> Result<(), Error> {
        let mut queue = Queue::new("", 1)?;
        queue.append("player");
        assert_eq!(
            queue,
            Queue {
                game: "",
                players: 1,
                queue: vecdeque!["player"],
            }
        );
        Ok(())
    }

    #[test]
    fn test_nextone() {
        let mut queue = Queue {
            game: "test",
            players: 2,
            queue: vecdeque!["player1"],
        };
        assert_eq!(queue.nextone(), Some("player1"));
        assert_eq!(queue.nextone(), None);
        assert_eq!(
            queue,
            Queue {
                game: "test",
                players: 2,
                queue: VecDeque::new(),
            }
        );
    }

    #[test]
    fn test_next() {
        let mut queue = Queue {
            game: "test",
            players: 2,
            queue: vecdeque!["player1", "player2", "player3"],
        };
        assert_eq!(queue.next(), vec!["player1", "player2"]);
        assert_eq!(queue.next(), vec!["player3"]);
        assert_eq!(queue.next(), Vec::<&str>::new());
        assert_eq!(
            queue,
            Queue {
                game: "test",
                players: 2,
                queue: VecDeque::new(),
            }
        );
    }

    #[test]
    fn test_next_to_back() {
        let mut queue = Queue {
            game: "test",
            players: 2,
            queue: vecdeque!["player1", "player2", "player3"],
        };
        assert_eq!(queue.next_to_back(), vec!["player1", "player2"]);
        assert_eq!(queue.next_to_back(), vec!["player3", "player1"]);
        assert_eq!(
            queue,
            Queue {
                game: "test",
                players: 2,
                queue: vecdeque!["player2", "player3", "player1"],
            }
        );
    }

    #[test]
    fn test_get_queue() -> Result<(), Error> {
        let mut queue = Queue::new("test", 2)?;
        queue.append("player1");
        queue.append("player2");
        assert_eq!(
            queue.get_queue(),
            vec!["player1", "player2"]
        );
        Ok(())
    }

    #[test]
    fn test_format_queue() -> Result<(), Error> {
        let mut queue = Queue::new("test", 2)?;
        queue.append("player1");
        queue.append("player2");
        assert_eq!(
            queue.format_queue(),
            "player1 player2"
        );
        Ok(())
    }

    #[test]
    fn test_fmt() -> Result<(), Error> {
        let mut queue = Queue::new("test", 2)?;
        queue.append("player1");
        queue.append("player2");
        assert_eq!(
            format!("{}", queue),
            "test (2 player(s) each round): player1 player2"
        );
        Ok(())
    }
}
