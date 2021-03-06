use std::collections::VecDeque;

use super::sliding_window::View;

#[derive(Clone)]
pub struct Normalizer {
    view: Box<dyn View>,
    window_len: usize,
    q_vals: VecDeque<f64>,
    min: f64,
    max: f64,
    last: f64,
    init: bool,
}

impl Normalizer {
    pub fn new(view: Box<dyn View>, window_len: usize) -> Normalizer {
        return Normalizer {
            view,
            window_len,
            q_vals: VecDeque::new(),
            min: 0.0,
            max: 0.0,
            last: 0.0,
            init: true,
        }
    }
}

pub fn extent_queue(q: &VecDeque<f64>) -> (f64, f64) {
    let mut min: &f64 = q.front().unwrap();
    let mut max: &f64 = q.front().unwrap();

    for i in 0..q.len() {
        let val = q.get(i).unwrap();
        if val > max {
            max = val;
        }
        if val < min {
            min = val;
        }
    }
    return (*min, *max)
}

impl View for Normalizer {
    fn update(&mut self, val: f64) {
        self.view.update(val);
        let view_last = self.view.last();

        if self.init {
            self.init = false;
            self.min = view_last;
            self.max = view_last;
            self.last = view_last;
        }
        if self.q_vals.len() >= self.window_len {
            let old = *self.q_vals.front().unwrap();
            if old <= self.min || old >= self.max {
                let (min, max) = extent_queue(&self.q_vals);
                self.min = min;
                self.max = max;
            }
            self.q_vals.pop_front();
        }
        self.q_vals.push_back(view_last);
        if view_last > self.max {
            self.max = view_last;
        }
        if view_last < self.min {
            self.min = view_last;
        }
        self.last = view_last;
    }

    fn last(&self) -> f64 {
        if self.last == self.min && self.last == self.max {
            return 0.0;
        }
        return -1.0 + (((self.last - self.min) * 2.0) / (self.max - self.min))
    }
}

#[cfg(test)]
mod tests {
    extern crate rust_timeseries_generator;

    use self::rust_timeseries_generator::gaussian_process::gen;
    use self::rust_timeseries_generator::plt;
    use super::*;
    use crate::trend_flex::TrendFlex;
    use crate::center_of_gravity::CenterOfGravity;
    use crate::cyber_cycle::CyberCycle;
    use crate::re_flex::ReFlex;
    use crate::roc::ROC;
    use crate::rsi::RSI;
    use crate::echo::Echo;

    #[test]
    fn normalizer() {
        let vals = gen(1024, 100.0);
        let mut n = Normalizer::new(Box::new(Echo::new()), 16);
        for i in 0..vals.len() {
            n.update(vals[i]);
            let last = n.last();
            assert!(last <= 1.0);
            assert!(last >= -1.0);
        }
    }

    #[test]
    fn normalizer_center_of_gravity() {
        let vals = gen(1024, 100.0);
        let window_len = 16;
        let cgo = CenterOfGravity::new(window_len);
        let mut n = Normalizer::new(Box::new(cgo), vals.len());
        let mut out: Vec<f64> = Vec::new();

        for i in 0..vals.len() {
            n.update(vals[i]);
            out.push(n.last());
        }

        let filename = "img/center_of_gravity_normalized.png";
        plt::plt(out, filename).unwrap();
    }

    #[test]
    fn normalizer_cyber_cycle() {
        let vals = gen(1024, 100.0);
        let window_len = 16;
        let cc = CyberCycle::new(window_len);
        let mut n = Normalizer::new(Box::new(cc), vals.len());
        let mut out: Vec<f64> = Vec::new();

        for i in 0..vals.len() {
            n.update(vals[i]);
            out.push(n.last());
        }

        let filename = "img/cyber_cycle_normalized.png";
        plt::plt(out, filename).unwrap();
    }

    #[test]
    fn normalizer_re_flex() {
        let vals = gen(1024, 100.0);
        let window_len = 16;
        let rf = ReFlex::new(window_len);
        let mut n = Normalizer::new(Box::new(rf), vals.len());
        let mut out: Vec<f64> = Vec::new();

        for i in 0..vals.len() {
            n.update(vals[i]);
            out.push(n.last());
        }

        let filename = "img/re_flex_normalized.png";
        plt::plt(out, filename).unwrap();
    }

    #[test]
    fn normalizer_roc() {
        let vals = gen(1024, 100.0);
        let window_len = 16;
        let r = ROC::new(window_len);
        let mut n = Normalizer::new(Box::new(r), vals.len());
        let mut out: Vec<f64> = Vec::new();

        for i in 0..vals.len() {
            n.update(vals[i]);
            out.push(n.last());
        }

        let filename = "img/roc_normalized.png";
        plt::plt(out, filename).unwrap();
    }

    #[test]
    fn normalizer_rsi() {
        let vals = gen(1024, 100.0);
        let window_len = 16;
        let r = RSI::new(window_len);
        let mut n = Normalizer::new(Box::new(r), vals.len());
        let mut out: Vec<f64> = Vec::new();

        for i in 0..vals.len() {
            n.update(vals[i]);
            out.push(n.last());
        }

        let filename = "img/rsi_normalized.png";
        plt::plt(out, filename).unwrap();
    }

    #[test]
    fn normalizer_trend_flex() {
        let vals = gen(1024, 100.0);
        let window_len = 16;
        let tf = TrendFlex::new(window_len);
        let mut n = Normalizer::new(Box::new(tf), vals.len());
        let mut out: Vec<f64> = Vec::new();

        for i in 0..vals.len() {
            n.update(vals[i]);
            out.push(n.last());
        }

        let filename = "img/trend_flex_normalized.png";
        plt::plt(out, filename).unwrap();
    }
}
