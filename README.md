# Computation of Flow Fields for Path Finding

Background:
 * <http://leifnode.com/2013/12/flow-field-pathfinding/>
 * <https://gamedevelopment.tutsplus.com/tutorials/understanding-goal-based-vector-field-pathfinding--gamedev-9007>
 * <http://www.gamasutra.com/view/news/288020/Game_Design_Deep_Dive_Creating_believable_crowds_in_Planet_Coaster.php>

# Plan

 * Validate that implementation is actually correct.
 * Try to improve performance via SIMD

# Status

Baseline implemantation bench results on my machine (~2013 MacBook Air, 1.7GHz Core i7):

```
$ make bench
test baseline::tests::bench_1000       ... bench:  27,441,925 ns/iter (+/- 5,546,706)
test baseline::tests::bench_100_a      ... bench:     243,498 ns/iter (+/- 152,323)
test baseline::tests::bench_100_b      ... bench:     258,394 ns/iter (+/- 131,835)
test baseline::tests::bench_flow_100   ... bench:     185,730 ns/iter (+/- 97,689)
test baseline::tests::bench_flow_1000  ... bench:  18,869,607 ns/iter (+/- 4,031,670)
test baseline::tests::bench_reset_100  ... bench:         225 ns/iter (+/- 152)
test baseline::tests::bench_reset_1000 ... bench:      76,494 ns/iter (+/- 20,227)
```

So, for a 100x100 grid, calculating the integration field (or
potential field) takes about 0.25 ms and 27 ms for a 1000x1000 grid
(with trivial movement consts).

Calculating the actual flow map then takes another 0.18ms for 100x1000
(or 18ms for 1000x1000). That seems rather seems pretty slow.

The first Leifnode article above reports 20ms for a 100x100 grid,
which is 40 times slower. Main difference is that I changed the queue
behaviour to add things to the queue unconditionally. This makes the
queue potentially a bit larger, but it avoids having to check whether
an item is already in the queue.
