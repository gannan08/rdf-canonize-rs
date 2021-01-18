# rdf-canonize-rust

## Performance

Performance on this branch is not better.  Perhaps if more work is pushed into each worker.

### This branch 61ff4aff9ce639a04e1ee3ca246cd771caf367b1
```
running 10 tests
test tests::test044_canonize ... ignored
test tests::test_merge_canonize ... ignored
test tests::test_veres_one_did_canonize ... ignored
test tests::bench009_canonize            ... bench:       1,055 ns/iter (+/- 45)
test tests::bench044_canonize            ... bench:   1,892,903 ns/iter (+/- 23,037)
test tests::bench044_parse               ... bench:      38,883 ns/iter (+/- 2,189)
test tests::bench_merge_canonize         ... bench:       9,651 ns/iter (+/- 334)
test tests::bench_merge_parse            ... bench:      25,242 ns/iter (+/- 1,309)
test tests::bench_veres_one_did_canonize ... bench:      22,970 ns/iter (+/- 357)
test tests::bench_veres_one_did_parse    ... bench:      89,803 ns/iter (+/- 14,285)
```

### main f6df0619fe8f20eb910696bf0c13b15346851819
```
running 10 tests
test tests::test044_canonize ... ignored
test tests::test_merge_canonize ... ignored
test tests::test_veres_one_did_canonize ... ignored
test tests::bench009_canonize            ... bench:       1,006 ns/iter (+/- 21)
test tests::bench044_canonize            ... bench:   1,694,877 ns/iter (+/- 15,876)
test tests::bench044_parse               ... bench:      30,205 ns/iter (+/- 562)
test tests::bench_merge_canonize         ... bench:       9,233 ns/iter (+/- 120)
test tests::bench_merge_parse            ... bench:      30,558 ns/iter (+/- 614)
test tests::bench_veres_one_did_canonize ... bench:      21,161 ns/iter (+/- 190)
test tests::bench_veres_one_did_parse    ... bench:      78,412 ns/iter (+/- 2,237)
```


## Run Binary

```sh
git clone --depth 1 https://github.com/json-ld/normalization.git _normalization
cargo run _normalization/tests/test061-in.nq
```

## Run Test

```sh
cargo test
```
