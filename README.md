# rust-sort

sort algorithms

## exchange sort

### bubble sort

```rust
extern crate sort;

use sort::bubble_sort;

fn main() {
    let mut data = vec![5, 4, 3, 2, 1];
    bubble_sort(&mut data);
    assert_eq!(data, [1, 2, 3, 4, 5]);
}
```