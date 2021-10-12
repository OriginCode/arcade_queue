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
use thiserror::Error;

mod macros;

#[derive(PartialEq, Debug)]
pub struct Queue<'a> {
    game: &'a str,
    players: u8,
    queue: VecDeque<&'a str>,
}

#[derive(Error, Debug)]
pub enum QueueError {
    #[error("`players` should be at least 1")]
    TooLessPlayersError,
    #[error("the player is already in the queue")]
    AlreadyInQueueError,
}

impl<'a> fmt::Display for Queue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({} player(s) each round): {}",
            self.game,
            self.players,
            self.format_queue()
        )
    }
}

impl<'a> Queue<'a> {
    /// Creates an empty queue with a name to the game and a number of players for each round.
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
    pub fn new(game: &'a str, players: u8) -> Result<Self, QueueError> {
        if players == 0 {
            return Err(QueueError::TooLessPlayersError);
        }
        Ok(Self {
            game,
            players,
            queue: VecDeque::new(),
        })
    }

    /// Adds a player to the end of the queue. The player should not be in the queue already.
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
    /// q.join("player1");
    /// q.join("player2");
    ///
    /// assert_eq!(q.format_queue(), "player1, player2");
    /// ```
    #[inline]
    pub fn join(&mut self, player: &'a str) -> Result<(), QueueError> {
        if !self.queue.contains(&player) {
            self.queue.push_back(player);
            Ok(())
        } else {
            Err(QueueError::AlreadyInQueueError)
        }
    }

    /// Remove a player from the queue.
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
    /// q.join("player1");
    /// q.join("player2");
    ///
    /// assert_eq!(q.format_queue(), "player1, player2");
    /// ```
    #[inline]
    pub fn quit(&mut self, player: &'a str) {
        self.queue.retain(|p| *p != player);
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
    /// q.join("player1");
    /// q.join("player2");
    ///
    /// assert_eq!(q.nextone().unwrap(), "player1");
    /// ```
    #[inline]
    pub fn nextone(&mut self) -> Option<&'a str> {
        self.queue.pop_front()
    }

    /// Yields the next one player and push them back to the queue.
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
    /// q.join("player1");
    /// q.join("player2");
    ///
    /// assert_eq!(q.nextone_to_back().unwrap().unwrap(), "player1");
    /// assert_eq!(q.get_queue(), vec!["player2", "player1"]);
    /// ```
    #[inline]
    pub fn nextone_to_back(&mut self) -> Result<Option<&'a str>, QueueError> {
        let player = self.nextone();
        if let Some(p) = player {
            self.join(p)?;
        }
        Ok(player)
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
    /// q.join("player1");
    /// q.join("player2");
    /// q.join("player3");
    ///
    /// assert_eq!(q.next_group(), vec!["player1", "player2"]);
    /// ```
    pub fn next_group(&mut self) -> Vec<&'a str> {
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
    /// q.join("player1");
    /// q.join("player2");
    /// q.join("player3");
    ///
    /// assert_eq!(q.next_group_to_back().unwrap(), vec!["player1", "player2"]);
    /// assert_eq!(q.get_queue(), vec!["player3", "player1", "player2"]);
    /// ```
    pub fn next_group_to_back(&mut self) -> Result<Vec<&'a str>, QueueError> {
        let mut result = Vec::new();
        for _ in 0..self.players {
            if let Some(p) = self.nextone_to_back()? {
                result.push(p);
            }
        }
        Ok(result)
    }

    /// Returns the current queue.
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
    /// q.join("player1");
    /// q.join("player2");
    /// q.join("player3");
    ///
    /// assert_eq!(q.get_queue(), vec!["player1", "player2", "player3"])
    /// ```
    pub fn get_queue(&self) -> Vec<&'a str> {
        self.queue.clone().into()
    }

    /// Returns the current formatted queue.
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
    /// q.join("player1");
    /// q.join("player2");
    /// q.join("player3");
    ///
    /// assert_eq!(q.format_queue(), "player1, player2, player3")
    /// ```
    pub fn format_queue(&self) -> String {
        self.get_queue().join(", ")
    }
}

#[cfg(test)]
mod queue_tests {
    use crate::*;

    #[test]
    fn test_new() -> Result<(), QueueError> {
        assert_eq!(Queue::new("", 1)?.get_queue(), Vec::<&str>::new());
        assert!(Queue::new("", 0).is_err());
        Ok(())
    }

    #[test]
    fn test_join() -> Result<(), QueueError> {
        let mut queue = Queue::new("", 1)?;
        queue.join("player")?;
        assert_eq!(queue.get_queue(), vec!["player"]);
        Ok(())
    }

    #[test]
    fn test_quit() -> Result<(), QueueError> {
        let mut queue = Queue::new("", 1)?;
        queue.join("player1")?;
        queue.join("player2")?;
        queue.quit("player2");
        assert_eq!(queue.get_queue(), vec!["player1"]);
        Ok(())
    }

    #[test]
    fn test_nextone() -> Result<(), QueueError> {
        let mut queue = Queue::new("test", 2)?;
        queue.join("player")?;
        assert_eq!(queue.nextone(), Some("player"));
        assert_eq!(queue.nextone(), None);
        assert_eq!(queue.get_queue(), Vec::<&str>::new());
        Ok(())
    }

    #[test]
    fn test_nextone_to_back() -> Result<(), QueueError> {
        let mut queue = Queue::new("test", 2)?;
        queue.join("player1")?;
        queue.join("player2")?;
        assert_eq!(queue.nextone_to_back()?.unwrap(), "player1");
        assert_eq!(queue.nextone_to_back()?.unwrap(), "player2");
        assert_eq!(queue.get_queue(), vec!["player1", "player2"]);
        Ok(())
    }

    #[test]
    fn test_next_group() -> Result<(), QueueError> {
        let mut queue = Queue::new("test", 2)?;
        queue.join("player1")?;
        queue.join("player2")?;
        queue.join("player3")?;
        assert_eq!(queue.next_group(), vec!["player1", "player2"]);
        assert_eq!(queue.next_group(), vec!["player3"]);
        assert_eq!(queue.next_group(), Vec::<&str>::new());
        assert_eq!(queue.get_queue(), Vec::<&str>::new());
        Ok(())
    }

    #[test]
    fn test_next_group_to_back() -> Result<(), QueueError> {
        let mut queue = Queue::new("test", 2)?;
        queue.join("player1")?;
        queue.join("player2")?;
        queue.join("player3")?;
        assert_eq!(queue.next_group_to_back()?, vec!["player1", "player2"]);
        assert_eq!(queue.next_group_to_back()?, vec!["player3", "player1"]);
        assert_eq!(queue.get_queue(), vec!["player2", "player3", "player1"]);
        Ok(())
    }

    #[test]
    fn test_get_queue() -> Result<(), QueueError> {
        let mut queue = Queue::new("test", 2)?;
        queue.join("player1")?;
        queue.join("player2")?;
        assert_eq!(queue.get_queue(), vec!["player1", "player2"]);
        Ok(())
    }

    #[test]
    fn test_format_queue() -> Result<(), QueueError> {
        let mut queue = Queue::new("test", 2)?;
        queue.join("player1")?;
        queue.join("player2")?;
        assert_eq!(queue.format_queue(), "player1, player2");
        Ok(())
    }

    #[test]
    fn test_fmt() -> Result<(), QueueError> {
        let mut queue = Queue::new("test", 2)?;
        queue.join("player1")?;
        queue.join("player2")?;
        assert_eq!(
            format!("{}", queue),
            "test (2 player(s) each round): player1, player2"
        );
        Ok(())
    }
}
