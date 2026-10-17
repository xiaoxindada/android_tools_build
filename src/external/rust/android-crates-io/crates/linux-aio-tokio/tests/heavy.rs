use std::fs::OpenOptions;
use std::sync::Arc;
use std::time::Duration;

use rand::{Rng, thread_rng};
use tempfile::tempdir;
use tokio::task::JoinSet;
use tokio::time::sleep;

use helpers::*;
use linux_aio_tokio::AioOpenOptionsExt;
use linux_aio_tokio::{LockedBuf, ReadFlags, WriteFlags, aio_context};

const PAGE_SIZE: usize = 1024 * 1024;
const NUM_PAGES: usize = 256;

const NUM_READERS: usize = 256;
const NUM_WRITERS: usize = 4;
const NUM_AIO_THREADS: usize = 4;

pub mod helpers;

#[tokio::test(flavor = "multi_thread")]
async fn load_test() {
    if LockedBuf::with_size(PAGE_SIZE).is_err() {
        eprintln!("Skipping load_test due to memlock limitations");
        return;
    }

    let dir = tempdir().unwrap();
    let path = dir.path().join("tmp");

    let mut open_options = OpenOptions::new();
    open_options.write(true).create_new(true).read(true);

    let mut f = open_options.aio_open(path.clone(), false).await.unwrap();

    f.set_len((NUM_PAGES * PAGE_SIZE) as u64).await.unwrap();

    let file = Arc::new(f);

    let (_aio, aio_handle) = aio_context(NUM_AIO_THREADS, true).unwrap();

    let mut f = vec![];

    for _ in 0..NUM_READERS {
        let aio_handle = aio_handle.clone();
        let file = file.clone();

        f.push(tokio::spawn(async move {
            let mut buffer = match LockedBuf::with_size(PAGE_SIZE) {
                Ok(buf) => buf,
                Err(_) => return,
            };
            let aio_handle = aio_handle.clone();
            let file = file.clone();

            loop {
                let page = thread_rng().gen_range(0..NUM_PAGES);

                let res = file
                    .read_at(
                        &aio_handle,
                        (page * PAGE_SIZE) as u64,
                        &mut buffer,
                        PAGE_SIZE as _,
                        ReadFlags::empty(),
                    )
                    .await
                    .unwrap();

                assert_eq!(PAGE_SIZE, res as usize);
            }
        }));
    }

    for _ in 0..NUM_WRITERS {
        let aio_handle = aio_handle.clone();
        let file = file.clone();

        f.push(tokio::spawn(async move {
            let mut buffer = match LockedBuf::with_size(PAGE_SIZE) {
                Ok(buf) => buf,
                Err(_) => return,
            };

            let aio_handle = aio_handle.clone();
            let file = file.clone();

            loop {
                let page = thread_rng().gen_range(0..NUM_PAGES);
                thread_rng().fill(buffer.as_mut());

                let res = file
                    .write_at(
                        &aio_handle,
                        (page * PAGE_SIZE) as u64,
                        &buffer,
                        PAGE_SIZE as _,
                        WriteFlags::DSYNC,
                    )
                    .await
                    .unwrap();

                assert_eq!(PAGE_SIZE, res as usize);
            }
        }));
    }

    let stress = async {
        for handle in f {
            let _ = handle.await;
        }
    };

    let timeout = sleep(Duration::from_secs(30));

    tokio::pin!(stress);
    tokio::pin!(timeout);

    tokio::select! {
        _ = &mut stress => {
            // never ends
            panic!("stress tasks unexpectedly completed");
        },
        _ = &mut timeout => {
        },
    }

    dir.close().unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn read_many_blocks_mt() {
    const FILE_SIZE: usize = 1024 * 512;
    const BUF_CAPACITY: usize = 8192;

    if LockedBuf::with_size(BUF_CAPACITY).is_err() {
        eprintln!("Skipping read_many_blocks_mt due to memlock limitations");
        return;
    }

    let (dir, path) = create_filled_tempfile(FILE_SIZE);

    let mut open_options = OpenOptions::new();
    open_options.read(true).write(true);

    let file = Arc::new(open_options.aio_open(path.clone(), true).await.unwrap());

    let num_slots = 7;
    let (aio, aio_handle) = aio_context(num_slots, true).unwrap();

    // 50 waves of requests just going above the limit

    // Waves start here
    for _wave in 0u64..50 {
        let mut set = JoinSet::new();
        let aio_handle = aio_handle.clone();
        let file = file.clone();

        // Each wave makes 100 I/O requests
        for index in 0u64..100 {
            let file = file.clone();
            let aio_handle = aio_handle.clone();

            set.spawn(async move {
                let offset = (index * BUF_CAPACITY as u64) % FILE_SIZE as u64;
                let mut buffer = match LockedBuf::with_size(BUF_CAPACITY) {
                    Ok(buf) => buf,
                    Err(_) => return,
                };

                file.read_at(
                    &aio_handle,
                    offset,
                    &mut buffer,
                    BUF_CAPACITY as _,
                    ReadFlags::empty(),
                )
                .await
                .unwrap();

                assert!(validate_block(buffer.as_ref()));
            });
        }

        while let Some(res) = set.join_next().await {
            res.unwrap();
        }

        // all slots have been returned
        assert_eq!(num_slots, aio.available_slots().unwrap());
    }

    dir.close().unwrap();
}
