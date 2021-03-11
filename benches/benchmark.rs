use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use rustoner::dl_lite::*;
use rustoner::kb::*;

// ------------------------------------------------------
// for symbols first
pub fn read_symbols_from_native(filename: &str) -> usize {
    let name = String::from(filename);
    let mut onto = ontology::Ontology_DLlite::new(name);

    onto.add_symbols_from_file(filename, types::FileType::NATIVE, false);
    1
}

pub fn criterion_benchmark_symbols(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_symbols_native");

    let duration = std::time::Duration::new(1200, 0);
    group.measurement_time(duration);

    let symbols = ["symbols10", "symbols100", "symbols1000", "symbols10000"]; // , "symbols100000" , "symbols1000000"];
    let symbols = symbols
        .iter()
        .map(|x| format!("./benches/symbols/{}", x))
        .collect::<Vec<String>>();

    for s in &symbols {
        group.bench_with_input(BenchmarkId::from_parameter(s), s, |b, s| {
            b.iter(|| read_symbols_from_native(black_box(s.as_str())));
        });
    }
}

// then read tbox
// then read abox
// then complete tbox
// then complete abox

criterion_group!(benches, criterion_benchmark_symbols);
criterion_main!(benches);
