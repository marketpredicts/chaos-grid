use ndarray::prelude::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

use rayon::prelude::*;

use indicatif::ParallelProgressIterator;
use ndarray::iter::LanesIter;

use clap::Parser;

use fxhash::FxHashMap;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of iterations to run
    #[arg(short, long, default_value_t = 1_000_000)]
    iterations: u64,
}

fn runs_at_least(board: &Array<u8, Ix2>, at_least: u64) -> FxHashMap<u64, u64> {
    let mut runs = FxHashMap::default();

    fn process_axis(lanes: &mut LanesIter<u8, Ix1>, runs: &mut FxHashMap<u64, u64>, at_least: u64) {
        for lane in lanes {
            let mut iter = lane.iter();
            let mut run: u64 = 1;
            let mut last = iter.next().unwrap();

            for x in iter {
                if x == last {
                    run += 1;
                } else {
                    if run >= at_least {
                        runs.entry(run).and_modify(|e| *e += 1).or_insert(1);
                    }
                    run = 1;
                }
                last = x;
            }
            if run >= at_least {
                runs.entry(run).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }

    process_axis(&mut (board.rows().into_iter()), &mut runs, at_least);
    process_axis(&mut (board.columns().into_iter()), &mut runs, at_least);

    runs
}

fn score_runs(runs: &FxHashMap<u64, u64>) -> u64 {
    runs.iter().map(|(run, count)| (run - 2) * count).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let iterations = args.iterations;

    let (scores, largest_runs) = (0..iterations)
        .into_par_iter()
        .progress_count(iterations)
        .map(|_| {
            let board = Array::random((6, 6), Uniform::new(1u8, 7u8));
            let runs = runs_at_least(&board, 3);
            let score = score_runs(&runs);
            let largest_run = runs.keys().max().cloned();
            (score, largest_run)
        })
        .fold(
            || (FxHashMap::<u64, u64>::default(), FxHashMap::<u64, u64>::default()),
            |(mut scores, mut largest_runs), (score, largest_run)| {
                scores.entry(score).and_modify(|e| *e += 1).or_insert(1);
                if let Some(largest_run) = largest_run {
                    largest_runs
                        .entry(largest_run)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
                (scores, largest_runs)
            },
        )
        .reduce(
            || (FxHashMap::default(), FxHashMap::default()),
            |mut acc, cur| {
                cur.0.iter().for_each(|(k, v)| {
                    acc.0.entry(*k).and_modify(|e| *e += v).or_insert(*v);
                });
                cur.1.iter().for_each(|(k, v)| {
                    acc.1.entry(*k).and_modify(|e| *e += v).or_insert(*v);
                });
                acc
            },
        );

    let mut scores_keys: Vec<_> = scores.keys().collect();
    scores_keys.sort();
    for score in scores_keys {
        let count = scores.get(score).unwrap();

        println!(
            "Score: {score} Count: {count} ({:.2}%)",
            (count.to_owned() as f64) * 100f64 / (iterations as f64)
        );
    }

    let mut largest_runs_keys: Vec<_> = largest_runs.keys().collect();
    largest_runs_keys.sort();
    for run in largest_runs_keys {
        let count = largest_runs.get(run).unwrap();

        println!(
            "Run: {run} Count: {count} ({}%)",
            (count.to_owned() as f64) * 100f64 / (iterations as f64)
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::score_runs;

    #[test]
    fn example_1() {
        let board = ndarray::arr2(&[
            [1, 1, 1, 3, 4, 4],
            [4, 1, 6, 4, 5, 5],
            [2, 3, 5, 5, 1, 6],
            [3, 3, 3, 3, 2, 2],
            [6, 4, 2, 2, 3, 2],
            [5, 1, 4, 3, 3, 2],
        ]);
        let runs = super::runs_at_least(&board, 3);
        let score = score_runs(&runs);
        let largest_run = runs.keys().max().cloned();
        assert_eq!(score, 4);
        assert_eq!(largest_run, Some(4));
    }

    #[test]
    fn example_2() {
        let board = ndarray::arr2(&[
            [1, 3, 1, 3, 4, 4],
            [4, 1, 6, 4, 5, 5],
            [2, 3, 5, 5, 1, 6],
            [3, 3, 1, 3, 2, 2],
            [6, 4, 2, 2, 3, 6],
            [5, 1, 4, 3, 3, 2],
        ]);
        let runs = super::runs_at_least(&board, 3);
        let score = score_runs(&runs);
        let largest_run = runs.keys().max().cloned();
        assert_eq!(score, 0);
        assert_eq!(largest_run, None);
    }

    #[test]
    fn example_3() {
        let board = ndarray::arr2(&[
            [1, 4, 2, 3, 6, 5],
            [4, 1, 3, 3, 4, 1],
            [4, 6, 5, 1, 2, 4],
            [4, 4, 5, 3, 2, 3],
            [4, 5, 1, 2, 3, 3],
            [4, 5, 6, 2, 6, 2],
        ]);
        let runs = super::runs_at_least(&board, 3);
        let score = score_runs(&runs);
        let largest_run = runs.keys().max().cloned();
        assert_eq!(score, 3);
        assert_eq!(largest_run, Some(5));
    }

    #[test]
    fn example_4() {
        let board = ndarray::arr2(&[
            [1, 4, 4, 5, 5, 4],
            [4, 1, 6, 4, 5, 2],
            [2, 3, 5, 5, 5, 6],
            [3, 3, 1, 3, 5, 2],
            [6, 4, 2, 2, 5, 6],
            [5, 1, 4, 3, 5, 2],
        ]);
        let runs = super::runs_at_least(&board, 3);
        let score = score_runs(&runs);
        let largest_run = runs.keys().max().cloned();
        assert_eq!(score, 5);
        assert_eq!(largest_run, Some(6));
    }
}
