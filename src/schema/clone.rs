use std::any::Any;
use std::collections::HashMap;

/// A trait for types that can be cloned as Any.
pub trait CloneAny {
    /// Creates a clone of the value as a Box<dyn Any>.
    fn clone_any(&self) -> Box<dyn Any>;
}

impl CloneAny for String {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}

impl CloneAny for f64 {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(*self)
    }
}

impl CloneAny for bool {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(*self)
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

impl CloneAny for Box<dyn Any> {
    fn clone_any(&self) -> Box<dyn Any> {
        if let Some(s) = self.downcast_ref::<String>() {
            Box::new(s.clone())
        } else if let Some(n) = self.downcast_ref::<f64>() {
            Box::new(*n)
        } else if let Some(b) = self.downcast_ref::<bool>() {
            Box::new(*b)
        } else if let Some(opt) = self.downcast_ref::<Option<Box<dyn Any>>>() {
            Box::new(opt.as_ref().map(|v| v.clone_any()))
        } else if let Some(vec) = self.downcast_ref::<Vec<Box<dyn Any>>>() {
            Box::new(vec.iter().map(|v| v.clone_any()).collect::<Vec<_>>())
        } else if let Some(map) = self.downcast_ref::<HashMap<String, Box<dyn Any>>>() {
            let mut new_map = HashMap::new();
            for (k, v) in map {
                new_map.insert(k.clone(), v.clone_any());
            }
            Box::new(new_map)
        } else {
            Box::new(())
        }
    }
}

impl<T: CloneAny + 'static> CloneAny for Vec<T> {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(self.iter().map(|v| v.clone_any()).collect::<Vec<_>>())
    }
}

impl CloneAny for usize {
    fn clone_any(&self) -> Box<dyn Any> {
        Box::new(*self)
    }
}

impl<K: Clone + Eq + std::hash::Hash + 'static, V: CloneAny + 'static> CloneAny for HashMap<K, V> {
    fn clone_any(&self) -> Box<dyn Any> {
        let mut map = HashMap::new();
        for (k, v) in self {
            map.insert(k.clone(), v.clone_any());
        }
        Box::new(map)
    }
}