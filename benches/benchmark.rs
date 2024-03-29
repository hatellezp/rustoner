/*
UMONS 2021
Horacio Alejandro Tellez Perez

LICENSE GPLV3+:
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses/.
*/

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use rustoner::dl_lite::*;
use rustoner::kb::*;

// ------------------------------------------------------
// for symbols first
pub fn read_symbols_from_native(filename: &str) -> usize {
    let name = String::from(filename);
    let mut onto = ontology::OntologyDllite::new(name);

    onto.add_symbols_from_file(filename, types::FileType::Native, false);
    1
}

pub fn criterion_benchmark_symbols(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_symbols_native");

    let duration = std::time::Duration::new(120, 0);
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
