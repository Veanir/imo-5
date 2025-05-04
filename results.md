| Instance | Algorithm                    | Cost (min - avg - max) | Time (ms, avg) | Iterations (avg) |
|----------|------------------------------|------------------------|----------------|------------------|
| kroa200 | MSLS (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), Iterations: 200) | 36327 (36907.10 - 37331) |        3606.90 |              N/A |
| kroa200 | ILS (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), Perturb: SmallPerturbation(n_moves=10)) | 35445 (36620.80 - 37074) |        3614.90 |            193.4 |
| kroa200 | LNS (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), Perturb: LargePerturbation(destroy=0.20)) (LS on Initial) | 36548 (36957.00 - 37290) |        3619.80 |            178.4 |
| kroa200 | LNSa (no LS after repair) (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), Perturb: LargePerturbation(destroy=0.20)) (LS on Initial) | 32073 (32812.20 - 34558) |        3607.70 |           2104.6 |
| kroa200 | HAE+LS (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), pop=20, min_diff=40) | 36517 (36968.60 - 37433) |        3620.20 |             95.0 |
| kroa200 | HAE (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), pop=20, min_diff=40) | 30862 (31147.20 - 31657) |        3611.00 |            313.7 |
| krob200 | MSLS (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), Iterations: 200) | 36763 (37087.80 - 37394) |        3684.20 |              N/A |
| krob200 | ILS (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), Perturb: SmallPerturbation(n_moves=10)) | 35823 (36848.40 - 37431) |        3696.40 |            199.4 |
| krob200 | LNS (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), Perturb: LargePerturbation(destroy=0.20)) (LS on Initial) | 36651 (37132.40 - 37432) |        3691.00 |            181.2 |
| krob200 | LNSa (no LS after repair) (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), Perturb: LargePerturbation(destroy=0.20)) (LS on Initial) | 32335 (33275.00 - 34879) |        3684.30 |           2158.2 |
| krob200 | HAE+LS (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), pop=20, min_diff=40) | 36573 (37313.70 - 37726) |        3697.40 |             98.2 |
| krob200 | HAE (Base: Local Search (Candidate k=10, EdgeExchange, Init: Random), pop=20, min_diff=40) | 30921 (31178.20 - 31383) |        3686.10 |            343.9 |