//! Tests that are expected to be executed inside a Docker container with 100mb of memory.

use std::{thread::sleep, time::Duration};

use cgroup_memory::{memory_available, memory_max};

#[test]
#[cfg_attr(not(feature = "test_memory_max_100m"), ignore)]
fn test_memory_max_100m() {
    assert_eq!(memory_max().unwrap().unwrap(), 104857600);
}

#[test]
#[cfg_attr(not(feature = "test_memory_max_100m"), ignore)]
fn test_memory_available() {
    sleep(Duration::from_secs(1));

    let free_before_alloc = memory_available().unwrap().unwrap();

    let mut one_mb = vec![0u8; 1024 * 1024 * 10];
    for i in 0..one_mb.len() {
        one_mb[i] = i as u8;
    }

    let free_after_alloc = memory_available().unwrap().unwrap();

    println!(
        "free: {}, free_after: {}",
        free_before_alloc, free_after_alloc
    );

    assert!((free_before_alloc - free_after_alloc) >= 1024 * 1024 * 10);

    drop(one_mb);

    let free_after_drop = memory_available().unwrap().unwrap();

    println!(
        "free_after: {}, free_after_drop: {}",
        free_after_alloc, free_after_drop
    );

    assert!((free_after_drop - free_after_alloc) >= 1024 * 1024 * 10);
}

#[test]
#[cfg_attr(not(feature = "test_memory_max_max"), ignore)]
fn test_memory_max_max() {
    assert!(memory_max().unwrap().is_none());
}
