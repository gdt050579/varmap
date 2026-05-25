mod data;
mod string_tests;
mod tracking_allocator;

use std::hint::black_box;
use std::time::Instant;
use data::TestTrait;
use tracking_allocator::AllocStats;

pub struct Entry {
    pub name: &'static str,
    pub run:  fn(count: usize, repeats: usize),
}
impl Entry {
    const fn new<T: TestTrait>() -> Self {
        Self {
            name: T::NAME,
            run: |count, repeats| run::<T>(count, repeats),
        }
    }
}

static ENTRIES: &[Entry] = &[
    Entry::new::<string_tests::StrVarMapCreateLarge>(),
    Entry::new::<string_tests::VarMapCreateLarge>(),
    Entry::new::<string_tests::EnumVarMapCreateLarge>(),
    Entry::new::<string_tests::HashMapCreateLarge>(),
    Entry::new::<string_tests::BTreeMapCreateLarge>(),
    Entry::new::<string_tests::StrVarMapCreateSmall>(),
    Entry::new::<string_tests::VarMapCreateSmall>(),
    Entry::new::<string_tests::EnumVarMapCreateSmall>(),
    Entry::new::<string_tests::HashMapCreateSmall>(),
    Entry::new::<string_tests::BTreeMapCreateSmall>(),
    // ============================== READ TEST CASES ==============================
    Entry::new::<string_tests::StrVarMapReadLarge>(),
    Entry::new::<string_tests::VarMapReadLarge>(),
    Entry::new::<string_tests::EnumVarMapReadLarge>(),
    Entry::new::<string_tests::HashMapReadLarge>(),
    Entry::new::<string_tests::BTreeMapReadLarge>(),
    Entry::new::<string_tests::StrVarMapReadSmall>(),
    Entry::new::<string_tests::VarMapReadSmall>(),
    Entry::new::<string_tests::EnumVarMapReadSmall>(),
    Entry::new::<string_tests::HashMapReadSmall>(),
    Entry::new::<string_tests::BTreeMapReadSmall>(),
];


fn run<T: TestTrait>(count: usize, repeats: usize) {
    let before_init = AllocStats::now();
    let mut test = T::init();
    let after_init = AllocStats::now();
    let mut sum = 0u128;
    // println!("Running test '{}'", T::NAME);
    for _ in 0..repeats {
        std::io::Write::flush(&mut std::io::stdout()).ok();
        let start = Instant::now();
        let _: () = test.run_test(black_box(count));
        black_box(());
        let duration = start.elapsed();
        sum += duration.as_millis();
    }
    let after_run = AllocStats::now();
    // println!("  Average: {} ms", sum / repeats as u128);
    // println!("  Memory usage: Init = {} bytes, Run = {} bytes", after_init.bytes - before_init.bytes, after_run.bytes - after_init.bytes);
    let algo =  T::NAME.split('-').next().unwrap();
    let test_name = &T::NAME[algo.len()+1..];
    println!("{:<12} | {:<20} | {:>10} ms | {:>10} bytes | {:>10} bytes |", algo, test_name, sum / repeats as u128, after_init.bytes - before_init.bytes, after_run.bytes - after_init.bytes);
}

fn usage(prog: &str) {
    eprintln!("Usage:\n{prog} LIST\n{prog} RUN <count> <repeats>\n{prog} RUN <count> <repeats> <search>\n\nExamples:\n{prog} LIST\n{prog} RUN 100 10\n{prog} RUN 100 10 strvarmap");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let prog = args[0].as_str();

    if args.len() < 2 {
        usage(prog);
        return;
    }

    match args[1].to_uppercase().as_str() {
        "LIST" => {
            for e in ENTRIES {
                println!("{}", e.name);
            }
        }

        // ── RUN ───────────────────────────────────────────────────────────
        "RUN" => {
            if args.len() < 4 {
                usage(prog);
                std::process::exit(1);
            }

            let count: usize = args[2].parse().unwrap_or_else(|_| {
                eprintln!("Error: <count> must be a positive integer.");
                std::process::exit(1);
            });
            let repeats: usize = args[3].parse().unwrap_or_else(|_| {
                eprintln!("Error: <repeats> must be a positive integer.");
                std::process::exit(1);
            });

            // Optional search filter (arg index 4)
            let filter: Option<String> = args.get(4).map(|s| s.to_lowercase());

            let matches: Vec<&Entry> = ENTRIES
                .iter()
                .filter(|e| match &filter {
                    None => true,
                    Some(q) => e.name.to_lowercase().contains(q.as_str()),
                })
                .collect();

            if matches.is_empty() {
                eprintln!(
                    "No tests matched '{}'.",
                    filter.unwrap_or_default()
                );
                std::process::exit(1);
            }

            for entry in matches {
                (entry.run)(count, repeats);
            }
        }

        _ => {
            usage(prog);
            std::process::exit(1);
        }
    }
}