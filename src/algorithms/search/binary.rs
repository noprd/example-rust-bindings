/// ----------------------------------------------------------------
/// IMPORTS
/// ----------------------------------------------------------------

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::thread::spawn;

use super::super::constants::TIMEOUT;

/// ----------------------------------------------------------------
/// STRUCTS
/// ----------------------------------------------------------------

/// Performs a the O(log(n)) binary search algorithm to determine
/// an index of a given element in a list of data.
/// If no element found, returns an error.
pub fn search(
    data: &Vec<String>,
    element: &String,
) -> Result<usize, String> {
    let n = data.len();

    // set up progress bar
    let pbar = ProgressBar::new(n as u64);
    let style = ProgressStyle::with_template("{spinner:.white} [{elapsed_precise}] [{wide_bar:.white}] {pos}/{len} ({eta_precise})");
    pbar.set_style(style.unwrap());

    // set up channels + thread
    let (tx, rx) = channel::<Option<usize>>();
    let tx_ = tx.clone();
    let data_ = data.clone();
    let element_ = element.clone();
    let offset_ = 0;
    spawn(move || search_recursive(&tx_, &data_, &element_, 0));

    // wait on threads
    let mut count: usize = 0;
    let mut index: Option<usize> = None;
    while count < n && let Ok(result) = rx.recv_timeout(TIMEOUT) {
        pbar.inc(1);
        count += 1;
        if index == None && let Some(i) = result {
            index = Some(i);
        }
    }
    pbar.finish();

    // evaluate result
    match index {
        Some(i) => {
            return Ok(i);
        },
        _ => {
            let error_msg = format!("could not find '{element}' in data");
            return Err(error_msg);
        }
    }
}


/// Single execution
fn search_recursive(
    tx: &Sender<Option<usize>>,
    data: &Vec<String>,
    element: &String,
    offset: usize,
) {
    let n = data.len();
    let error_msg = "no element found".to_string();
    match n {
        0 => {
            tx.send(None).unwrap();
        },
        1 => {
            if let Some(x) = data.get(0) && x == element {
                tx.send(Some(offset)).unwrap();
            } else {
                tx.send(None).unwrap();
            }
        },
        _ => {
            // find mid point index
            let p = (((n as f64) / 2.0) as usize);

            // divide job
            let tx_ = tx.clone();
            let tx_threads_ = tx.clone();
            let data_ = data[0..p].to_vec();
            let element_ = element.clone();
            let offset_ = offset;
            spawn(move || search_recursive(&tx_, &data_, &element_, offset_));

            let tx_ = tx.clone();
            let tx_threads_ = tx.clone();
            let data_ = data[p..n].to_vec();
            let element_ = element.clone();
            let offset_ = offset + p;
            spawn(move || search_recursive(&tx_, &data_, &element_, offset_));
        }
    }
}
