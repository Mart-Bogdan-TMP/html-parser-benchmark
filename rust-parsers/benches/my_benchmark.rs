extern crate core;

use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scraper::{Html, Node};
use ego_tree::Tree;
use kuchiki::{NodeRef as KuchikiNodeRef};
use html5ever::interface::LimitedQuirks;
use html5ever::{parse_document, ParseOpts, serialize};
use html5ever_arena_dom::{Arena, ArenaSink, ArenaSinkParser, Ref, Node as ArenaNode};
use html5ever::tendril::TendrilSink;
use typed_arena::Arena as RawArena;
use std::io::Write;
use std::mem;


const HTML_CONTENT: &str = include_str!("../../sample.html");
const BIG_HTML_CONTENT: &str = include_str!("../../big.html");

fn scrapper_parse(data: &str) -> Tree<Node> {
    let html = Html::parse_document(data);

    //let string = html.root_element().html();

    html.tree
}

fn kuchiki_parse(data: &str) -> KuchikiNodeRef {
    use kuchiki::traits::*;

    let document = kuchiki::parse_html().one(data);

    document
}

fn tl_parse(data: &str) {
    unsafe {
        let dom = tl::parse_owned(String::from(data), tl::ParserOptions::default()).unwrap();
        black_box(dom);
    }
}

// fn arenadom_parse<'x, 'arena>(data: &'x str) -> (RawArena<Ref<'arena>>, Ref<'arena>)  {
fn arenadom_parse<'x, 'arena>(arena: &'arena RawArena<ArenaNode<'arena>>, data: &'x str) -> Ref<'arena> {
    use html5ever::tree_builder::TreeSink;


    //let arena = typed_arena::Arena::new();
    let mut parser = ArenaSinkParser::new(
        &arena,
        ParseOpts {
            tree_builder: html5ever::tree_builder::TreeBuilderOpts {
                drop_doctype: false,
                quirks_mode: LimitedQuirks,
                scripting_enabled: false,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    write!(parser, "{}", data).unwrap();
    let document = parser.finish();

    // let arena:RawArena<Ref<'arena>> = RawArena::new();
    // let arena_sink:ArenaSink<'arena> = ArenaSink::new(&arena);
    // let dom = parse_document(arena_sink, opts)
    //     .one(data);

    document
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("Parse");
    {
        g.bench_function("scrapper", |b| b.iter(|| scrapper_parse(HTML_CONTENT)));
        g.bench_function("tl", |b| b.iter(|| tl_parse(HTML_CONTENT)));
        //c.bench_function("kuchiki parse", |b| b.iter(|| kuchiki_parse(HTML_CONTENT)));
        g.bench_function("ArenaDOM ", |b| b.iter(|| {
            let arena = typed_arena::Arena::new();
            let res = arenadom_parse(&arena, HTML_CONTENT);
            black_box(res);
        }));
        g.bench_function("ArenaDOM with_capacity", |b| b.iter(|| {
            let arena = typed_arena::Arena::with_capacity(4096);
            let res = arenadom_parse(&arena, HTML_CONTENT);
            black_box(res);
        }));
        // c.bench_function("scrapper parse drop", |b| b.iter_with_large_drop(|| scrapper_parse(HTML_CONTENT)));
    }
    g.finish();
    return;

    let mut g = c.benchmark_group("Parse BIG");
    {
        g.sample_size(10);
        g.bench_function("scrapper parse", |b| b.iter(|| scrapper_parse(BIG_HTML_CONTENT)));
        //c.bench_function("kuchiki parse", |b| b.iter(|| kuchiki_parse(HTML_CONTENT)));
        g.bench_function("arenadom parse", |b| b.iter(|| {
            let arena = typed_arena::Arena::new();
            let res = arenadom_parse(&arena, BIG_HTML_CONTENT);
            black_box(res);
        }));
        g.bench_function("arenadom capacity", |b| b.iter(|| {
            let arena = typed_arena::Arena::with_capacity(4096);
            let res = arenadom_parse(&arena, BIG_HTML_CONTENT);
            black_box(res);
        }));
        // c.bench_function("scrapper parse drop", |b| b.iter_with_large_drop(|| scrapper_parse(HTML_CONTENT)));
    }
    g.finish();
}


fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

const LIST_SIZE:i32 = 80000;

fn sum_loop(vec:&Vec<i32>)->i32{
    let mut sum =0;
    for x in vec {

        let i = *x;
        if i % 2 == 0 {
            let i1 = i + 3;
            sum += i1;
        }
    }
    sum
}

fn sum_iter(vec:&Vec<i32>)->i32{
    vec.iter().map(|x|*x).filter(|x|x%2==0).map(|x|x+3).sum()
}


fn sum_loop_box(vec:&Vec<Box<i32>>)->i32{
    let mut sum =0;
    for x in vec {

        let i = **x;
        if i % 2 == 0 {
            let i1 = i + 3;
            sum += i1;
        }
    }
    sum
}

fn sum_iter_box(vec:&Vec<Box<i32>>)->i32{
    vec.iter().map(|x|**x).filter(|x|x%2==0).map(|x|x+3).sum()
}

unsafe fn criterion_benchmark_streams(c: &mut Criterion) {


       let mut VEC:Vec<i32>=Vec::with_capacity(LIST_SIZE as usize);
       let mut VEC_BOX:Vec<Box<i32>>=Vec::with_capacity(LIST_SIZE as usize);


        for i in 0..LIST_SIZE  {
            VEC.push(i);
            VEC_BOX.push(Box::new(i));
        }


    let mut g = c.benchmark_group("Inline");

    g.bench_function("iter", |b| b.iter(|| sum_iter(black_box(&VEC))));
    g.bench_function("loop", |b| b.iter(|| sum_loop(black_box(&VEC))));

    g.finish();
    let mut g = c.benchmark_group("Box");

    g.bench_function("iter", |b| b.iter(|| sum_iter_box(black_box(&VEC_BOX))));
    g.bench_function("loop", |b| b.iter(|| sum_loop_box(black_box(&VEC_BOX))));

    g.finish();
}


pub fn benches() {
    let mut criterion: ::criterion::Criterion<_> =
        ::criterion::Criterion::default()
            .warm_up_time(Duration::from_secs(12))
            .measurement_time(Duration::from_secs(45))
            .sample_size(400)
            .configure_from_args();

    unsafe {criterion_benchmark_streams(&mut criterion);}
}

// you can uncomment following line if you comment previous function
// criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);


/*
results
Inline/iter             time:   [70.600 us 70.768 us 70.926 us]
Found 19 outliers among 400 measurements (4.75%)
  14 (3.50%) low severe
  5 (1.25%) low mild
Inline/loop             time:   [10.807 us 10.839 us 10.868 us]
Found 36 outliers among 400 measurements (9.00%)
  20 (5.00%) low severe
  8 (2.00%) low mild
  5 (1.25%) high mild
  3 (0.75%) high severe

Box/iter                time:   [70.498 us 70.765 us 71.011 us]
Found 50 outliers among 400 measurements (12.50%)
  31 (7.75%) low severe
  13 (3.25%) low mild
  3 (0.75%) high mild
  3 (0.75%) high severe
Box/loop                time:   [49.824 us 50.027 us 50.222 us]
Found 64 outliers among 400 measurements (16.00%)
  25 (6.25%) low severe
  27 (6.75%) low mild
  7 (1.75%) high mild
  5 (1.25%) high severe


WSL 1.60

Inline/iter             time:   [72.683 us 72.772 us 72.867 us]
Found 2 outliers among 400 measurements (0.50%)
  2 (0.50%) high mild
Inline/loop             time:   [11.183 us 11.197 us 11.211 us]
Found 10 outliers among 400 measurements (2.50%)
  10 (2.50%) high mild

Box/iter                time:   [74.071 us 74.188 us 74.314 us]
Found 7 outliers among 400 measurements (1.75%)
  4 (1.00%) high mild
  3 (0.75%) high severe
Box/loop                time:   [63.089 us 63.178 us 63.270 us]
Found 17 outliers among 400 measurements (4.25%)
  11 (2.75%) high mild
  6 (1.50%) high severe


 */
