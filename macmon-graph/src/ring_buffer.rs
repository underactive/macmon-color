/// Fixed-capacity ring buffer for time-series data.
///
/// Stores newest values at index 0 (matching macmon's `Vec::insert(0)`
/// convention). Uses a plain `Vec` internally — for ≤128 elements the
/// O(n) shift on push is negligible, and we get free `&[f64]` access
/// without the two-slice problem of `VecDeque`.

#[derive(Debug, Clone)]
pub struct RingBuffer {
    data: Vec<f64>,
    capacity: usize,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, value: f64) {
        self.data.insert(0, value);
        self.data.truncate(self.capacity);
    }

    pub fn as_slice(&self) -> &[f64] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn max(&self) -> f64 {
        self.data.iter().cloned().reduce(f64::max).unwrap_or(0.0)
    }

    pub fn min(&self) -> f64 {
        self.data.iter().cloned().reduce(f64::min).unwrap_or(0.0)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Exponential moving average, ported from macmon's `TempStore::trend_ema`.
    /// `alpha` controls smoothing (0.0 = full history, 1.0 = latest only).
    /// Iterates oldest→newest so the EMA weights recent values more heavily.
    pub fn ema(&self, alpha: f64) -> f64 {
        if self.data.len() < 2 {
            return self.data.first().copied().unwrap_or(0.0);
        }
        let mut iter = self.data.iter().rev();
        let mut ema = *iter.next().unwrap();
        for &val in iter {
            ema = alpha * val + (1.0 - alpha) * ema;
        }
        ema
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_truncate() {
        let mut rb = RingBuffer::new(3);
        rb.push(1.0);
        rb.push(2.0);
        rb.push(3.0);
        rb.push(4.0);
        assert_eq!(rb.as_slice(), &[4.0, 3.0, 2.0]);
        assert_eq!(rb.len(), 3);
    }

    #[test]
    fn newest_first() {
        let mut rb = RingBuffer::new(10);
        rb.push(10.0);
        rb.push(20.0);
        rb.push(30.0);
        assert_eq!(rb.as_slice()[0], 30.0);
        assert_eq!(rb.as_slice()[2], 10.0);
    }

    #[test]
    fn max_min() {
        let mut rb = RingBuffer::new(10);
        for v in &[5.0, 2.0, 8.0, 1.0, 9.0] {
            rb.push(*v);
        }
        assert_eq!(rb.max(), 9.0);
        assert_eq!(rb.min(), 1.0);
    }

    #[test]
    fn empty_buffer() {
        let rb = RingBuffer::new(10);
        assert!(rb.is_empty());
        assert_eq!(rb.max(), 0.0);
        assert_eq!(rb.ema(0.5), 0.0);
    }

    #[test]
    fn ema_smoothing() {
        let mut rb = RingBuffer::new(10);
        for v in &[10.0, 10.0, 10.0, 10.0, 10.0] {
            rb.push(*v);
        }
        let result = rb.ema(0.5);
        assert!((result - 10.0).abs() < 0.001);
    }
}
