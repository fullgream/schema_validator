use std::any::Any;
use std::collections::HashMap;

pub trait CloneAny {
    fn clone_any(&self) -> Box<dyn Any>;
}

impl CloneAny for String {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}

impl CloneAny for &str {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(self.to_string())
    }
}

impl CloneAny for f64 {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}

impl CloneAny for i64 {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}

impl CloneAny for bool {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}

impl CloneAny for usize {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}

impl<T: CloneAny + 'static> CloneAny for Option<T> {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(self.as_ref().map(|v| v.clone_any()))
    }
}

impl<T: CloneAny + 'static> CloneAny for Box<T> {
    fn clone_any(&self) -> Box<dyn Any> {
        (**self).clone_any()
    }
}

impl<K: Clone + 'static + std::hash::Hash + Eq, V: CloneAny + 'static> CloneAny for HashMap<K, V> {
    fn clone_any(&self) -> Box<dyn Any> {
        let mut map = HashMap::new();
        for (k, v) in self {
            map.insert(k.clone(), v.clone_any());
        }
        Box::new(map)
    }
}

impl CloneAny for Box<dyn Any> {
    fn clone_any(&self) -> Box<dyn Any> {
        if let Some(s) = self.downcast_ref::<String>() {
            Box::new(s.clone())
        } else if let Some(n) = self.downcast_ref::<f64>() {
            Box::new(n.clone())
        } else if let Some(n) = self.downcast_ref::<i64>() {
            Box::new(n.clone())
        } else if let Some(b) = self.downcast_ref::<bool>() {
            Box::new(b.clone())
        } else if let Some(n) = self.downcast_ref::<usize>() {
            Box::new(n.clone())
        } else {
            Box::new(())
        }
    }
}