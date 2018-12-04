#[macro_use]
extern crate criterion;
extern crate gu_random;

use criterion::Criterion;
use gu_random::*;

fn b_rand(c: &mut Criterion) {
    c.bench_function("rand", |b| b.iter(rand));
}

fn b_reqc(c: &mut Criterion) {
    c.bench_function("reqc", |b| b.iter(reqc));
}

fn b_igor(c: &mut Criterion) {
    c.bench_function("igor", |b| b.iter(igor));
}

fn b_mbenke(c: &mut Criterion) {
    c.bench_function("mbenke", |b| b.iter(mbenke));
}

fn b_issue_author(c: &mut Criterion) {
    c.bench_function("issue_author", |b| b.iter(issue_author));
}

fn b_igor_mbenke(c: &mut Criterion) {
    c.bench_function("igor_mbenke (incorrect)", |b| b.iter(igor_mbenke_incorrect));
}

fn b_2rec(c: &mut Criterion) {
    c.bench_function("2rec", |b| b.iter(tworec));
}

criterion_group!(
    benches,
    b_rand,
    b_reqc,
    b_igor,
    b_mbenke,
    b_issue_author,
    b_igor_mbenke,
    b_2rec
);
criterion_main!(benches);

/*
Ranking

cargo bench --bench random -- -v | grep time: | sort -n -k5,5

rand                    time:   [9.4652 ns 9.4924 ns 9.5219 ns]
igor_mbenke (incorrect) time:   [11.593 ns 11.629 ns 11.668 ns]
2rec                    time:   [11.604 ns 11.655 ns 11.712 ns]
reqc                    time:   [11.868 ns 11.911 ns 11.963 ns]
issue_author            time:   [11.876 ns 11.915 ns 11.960 ns]
igor                    time:   [11.922 ns 11.973 ns 12.025 ns]
mbenke                  time:   [22.713 ns 22.794 ns 22.878 ns]


Full Results

$ cargo bench --bench random -- -v
    Finished release [optimized] target(s) in 0.19s
     Running /Users/tworec/git/golem-unlimited/target/release/deps/random-bd195557e006c73c
Benchmarking rand
Benchmarking rand: Warming up for 3.0000 s
Benchmarking rand: Collecting 100 samples in estimated 5.0000 s (526598850 iterations)
Benchmarking rand: Analyzing
rand                    time:   [9.4652 ns 9.4924 ns 9.5219 ns]
                        change: [-1.7226% -0.7113% +0.2368%] (p = 0.16 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) low mild
  9 (9.00%) high mild
  1 (1.00%) high severe
slope  [9.4652 ns 9.5219 ns] R^2            [0.9736178 0.9734215]
mean   [9.4880 ns 9.5760 ns] std. dev.      [166.08 ps 286.18 ps]
median [9.4687 ns 9.5380 ns] med. abs. dev. [106.20 ps 174.36 ps]

Benchmarking reqc
Benchmarking reqc: Warming up for 3.0000 s
Benchmarking reqc: Collecting 100 samples in estimated 5.0001 s (416756300 iterations)
Benchmarking reqc: Analyzing
reqc                    time:   [11.868 ns 11.911 ns 11.963 ns]
                        change: [-1.1102% -0.4288% +0.2523%] (p = 0.22 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe
slope  [11.868 ns 11.963 ns] R^2            [0.9607399 0.9599549]
mean   [11.850 ns 11.959 ns] std. dev.      [212.75 ps 340.51 ps]
median [11.850 ns 11.958 ns] med. abs. dev. [146.83 ps 250.23 ps]

Benchmarking igor
Benchmarking igor: Warming up for 3.0000 s
Benchmarking igor: Collecting 100 samples in estimated 5.0001 s (417397650 iterations)
Benchmarking igor: Analyzing
igor                    time:   [11.922 ns 11.973 ns 12.025 ns]
                        change: [-0.7779% -0.1114% +0.5521%] (p = 0.75 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
slope  [11.922 ns 12.025 ns] R^2            [0.9574623 0.9573758]
mean   [11.909 ns 12.002 ns] std. dev.      [204.15 ps 270.43 ps]
median [11.882 ns 11.985 ns] med. abs. dev. [162.55 ps 270.57 ps]

Benchmarking mbenke
Benchmarking mbenke: Warming up for 3.0000 s
Benchmarking mbenke: Collecting 100 samples in estimated 5.0001 s (218634700 iterations)
Benchmarking mbenke: Analyzing
mbenke                  time:   [22.713 ns 22.794 ns 22.878 ns]
                        change: [-1.2447% -0.5548% +0.1248%] (p = 0.12 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) low mild
  4 (4.00%) high mild
slope  [22.713 ns 22.878 ns] R^2            [0.9632735 0.9631270]
mean   [22.671 ns 22.877 ns] std. dev.      [430.54 ps 614.21 ps]
median [22.673 ns 22.857 ns] med. abs. dev. [289.09 ps 489.35 ps]

Benchmarking issue_author
Benchmarking issue_author: Warming up for 3.0000 s
Benchmarking issue_author: Collecting 100 samples in estimated 5.0000 s (416195750 iterations)
Benchmarking issue_author: Analyzing
issue_author            time:   [11.876 ns 11.915 ns 11.960 ns]
                        change: [-0.4128% +0.3376% +1.1563%] (p = 0.41 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low mild
  8 (8.00%) high mild
  2 (2.00%) high severe
slope  [11.876 ns 11.960 ns] R^2            [0.9628754 0.9624114]
mean   [11.912 ns 12.060 ns] std. dev.      [235.42 ps 531.46 ps]
median [11.865 ns 11.938 ns] med. abs. dev. [125.80 ps 246.21 ps]

Benchmarking igor_mbenke (incorrect)
Benchmarking igor_mbenke (incorrect): Warming up for 3.0000 s
Benchmarking igor_mbenke (incorrect): Collecting 100 samples in estimated 5.0000 s (425144350 iterations)
Benchmarking igor_mbenke (incorrect): Analyzing
igor_mbenke (incorrect) time:   [11.593 ns 11.629 ns 11.668 ns]
                        change: [-1.0613% -0.1452% +0.6633%] (p = 0.75 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe
slope  [11.593 ns 11.668 ns] R^2            [0.9680275 0.9678682]
mean   [11.615 ns 11.733 ns] std. dev.      [219.30 ps 383.17 ps]
median [11.559 ns 11.658 ns] med. abs. dev. [132.32 ps 233.83 ps]

Benchmarking 2rec
Benchmarking 2rec: Warming up for 3.0000 s
Benchmarking 2rec: Collecting 100 samples in estimated 5.0000 s (428361200 iterations)
Benchmarking 2rec: Analyzing
2rec                    time:   [11.604 ns 11.655 ns 11.712 ns]
                        change: [-0.4243% +0.2571% +0.9461%] (p = 0.46 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
slope  [11.604 ns 11.712 ns] R^2            [0.9605365 0.9599322]
mean   [11.621 ns 11.724 ns] std. dev.      [199.92 ps 326.09 ps]
median [11.595 ns 11.679 ns] med. abs. dev. [149.45 ps 252.54 ps]


*/
