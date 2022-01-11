use crate::intset::GrowSet;
use crate::Capture;
use std::collections::VecDeque;

pub struct Queue {
    seen: GrowSet,
    deque: VecDeque<usize>,
    pub captures: Vec<Vec<Capture>>,
    empty: Vec<Capture>,
}

pub enum End {
    Front,
    Back,
}

impl Queue {
    pub fn new(program_size: usize, capture_count: usize) -> Self {
        Self {
            seen: GrowSet::with_capacity(program_size),
            deque: VecDeque::with_capacity(program_size),
            captures: (0..program_size)
                .map(|_x| {
                    (0..capture_count)
                        .map(|_y| Capture::new(None, None))
                        .collect()
                })
                .collect(),
            empty: (0..capture_count)
                .map(|_x| Capture::new(None, None))
                .collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }

    pub fn clear(&mut self) {
        self.deque.clear();
        self.seen.clear();
    }

    pub fn push_empty(&mut self, start: usize) {
        if self.seen.contains(0) {
            if start < self.captures[0][0].start.unwrap() {
                self.captures[0].copy_from_slice(&self.empty);
                self.captures[0][0].start = Some(start);
            }
            return;
        }

        self.seen.add(0);
        self.captures[0].clone_from(&self.empty);
        self.captures[0][0].start = Some(start);
        self.deque.push_back(0);
    }

    pub fn push(&mut self, end: End, pc: usize, captures: &[Capture]) {
        if self.seen.contains(pc) {
            if captures[0].start.unwrap() < self.captures[pc][0].start.unwrap() {
                self.captures[pc].copy_from_slice(captures);
            }
            return;
        }

        self.seen.add(pc);
        self.captures[pc].copy_from_slice(captures);

        match end {
            End::Front => self.deque.push_front(pc),
            End::Back => self.deque.push_back(pc),
        }
    }

    pub fn push_from_current(&mut self, end: End, pc: usize, capture_pc: usize) {
        if self.seen.contains(pc) {
            if self.captures[capture_pc][0].start.unwrap() < self.captures[pc][0].start.unwrap() {
                for index in 0..self.captures[pc].len() {
                    self.captures[pc][index] = self.captures[capture_pc][index];
                    // FIXME
                }
            }
            return;
        }

        self.seen.add(pc);
        for index in 0..self.captures[pc].len() {
            self.captures[pc][index] = self.captures[capture_pc][index]; // FIXME
        }
        match end {
            End::Front => self.deque.push_front(pc),
            End::Back => self.deque.push_back(pc),
        }
    }

    pub fn pop(&mut self) -> usize {
        self.deque.pop_front().unwrap()
    }
}
