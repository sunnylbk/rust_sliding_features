use dyn_clone::DynClone;

pub trait View: Send + DynClone {
    fn update(&mut self, val: f64);
    fn last(&self) -> f64;
}

dyn_clone::clone_trait_object!(View);

#[derive(Clone)]
pub struct SlidingWindow {
    pub views: Vec<Box<dyn View>>,
}

impl SlidingWindow {
    pub fn new() -> SlidingWindow {
        return SlidingWindow{
            views: Vec::new(),
        }
    }

    // update propagates the newly observed candle through all views
    pub fn update(&mut self, val: f64) {
        for i in 0..self.views.len() {
            self.views[i].update(val);
        }
    }
    pub fn last(&self) -> Vec<f64> {
        let mut out: Vec<f64> = Vec::new();
        for i in 0..self.views.len() {
            let last = self.views[i].last();
            out.push(last)
        }
        return out
    }

    // register_view adds the given view to SlidingFeatures
    pub fn register_view(&mut self, view: Box<dyn View>) {
        self.views.push(view);
    }
}