use criterion::{criterion_group, criterion_main, Criterion};
const INPUT: &str = "tzibqaulrw
bratip
trbhia
rtiab

sgqytjiw
gkwqybtims

tufcqanysox
ovufxnaqt
aenbqfkutgjx
taufvnxqi
nuaqxfmsth

qj
jsqx

gltviuzdrkema
idclmuektgora
idayrlmgukte
xailwumktdergf
jmrniauldbthkpqge

zc
ucfhz
cisz

i
i
i
i

rqigekxlmzswycnjhfv
lznacirjfkyvxwsmeg

bohsmgujxdnrwza
pfaeqkinwgxjt

buhiwcayjsdlox
hxoiajbdyucswl
asildxjbcywuofh
iyduashjcboxwl";
pub fn imperative_benchmark(c: &mut Criterion) {
    c.bench_function("imperative", |b| b.iter(|| aoc::solve_imperative(&INPUT)));

}

pub fn functional_benchmark(c: &mut Criterion) {
    c.bench_function("functional", |b| b.iter(|| aoc::solve_functional(&INPUT)));
}


criterion_group!(benches, imperative_benchmark, functional_benchmark);
criterion_main!(benches);

