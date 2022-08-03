A fast WASI implementation on V8. (POC to be upstreamed to Deno)

### Performance

`random_get`:

```
target/release/fastwasi

main (262144 bytes):           	n = 1000, dt = 0.166s, r = 6024/s, t = 166000ns/op
main (131334144 bytes):        	n = 1000, dt = 0.328s, r = 3049/s, t = 328000ns/op
main (262406144 bytes):        	n = 1000, dt = 0.369s, r = 2710/s, t = 368999ns/op
main (393478144 bytes):        	n = 1000, dt = 0.480s, r = 2083/s, t = 479999ns/op
main (524550144 bytes):        	n = 1000, dt = 0.542s, r = 1845/s, t = 542000ns/op
main (655622144 bytes):        	n = 1000, dt = 0.743s, r = 1346/s, t = 743000ns/op
main (786694144 bytes):        	n = 1000, dt = 1.026s, r = 975/s, t = 1026000ns/op
main (917766144 bytes):        	n = 1000, dt = 1.060s, r = 943/s, t = 1060000ns/op
main (1048838144 bytes):       	n = 1000, dt = 1.110s, r = 901/s, t = 1110000ns/op
main (1179910144 bytes):       	n = 1000, dt = 1.157s, r = 864/s, t = 1157000ns/op
main (1310982144 bytes):       	n = 1000, dt = 1.274s, r = 785/s, t = 1274000ns/op
main (1442054144 bytes):       	n = 1000, dt = 1.324s, r = 755/s, t = 1324000ns/op
main (1573126144 bytes):       	n = 1000, dt = 1.372s, r = 729/s, t = 1372000ns/op
main (1704198144 bytes):       	n = 1000, dt = 1.434s, r = 697/s, t = 1434000ns/op
main (1835270144 bytes):       	n = 1000, dt = 1.484s, r = 674/s, t = 1484000ns/op
main (1966342144 bytes):       	n = 1000, dt = 1.534s, r = 652/s, t = 1534000ns/op
main (2097414144 bytes):       	n = 1000, dt = 1.608s, r = 622/s, t = 1608000ns/op
main (2228486144 bytes):       	n = 1000, dt = 1.660s, r = 602/s, t = 1660000ns/op
main (2359558144 bytes):       	n = 1000, dt = 1.739s, r = 575/s, t = 1739000ns/op
main (2490630144 bytes):       	n = 1000, dt = 1.826s, r = 548/s, t = 1826000ns/op
main (2621702144 bytes):       	n = 1000, dt = 1.934s, r = 517/s, t = 1934000ns/op
main (2752774144 bytes):       	n = 1000, dt = 2.030s, r = 493/s, t = 2029999ns/op
main (2883846144 bytes):       	n = 1000, dt = 2.111s, r = 474/s, t = 2111000ns/op
```

```
deno run --allow-read examples/deno_std.js

main (262144 bytes):           	n = 1000, dt = 0.219s, r = 4566/s, t = 219000ns/op
main (131334144 bytes):        	n = 1000, dt = 0.432s, r = 2315/s, t = 432000ns/op
main (262406144 bytes):        	n = 1000, dt = 0.479s, r = 2088/s, t = 479000ns/op
main (393478144 bytes):        	n = 1000, dt = 0.533s, r = 1876/s, t = 533000ns/op
main (524550144 bytes):        	n = 1000, dt = 0.609s, r = 1642/s, t = 609000ns/op
main (655622144 bytes):        	n = 1000, dt = 1.299s, r = 770/s, t = 1299000ns/op
main (786694144 bytes):        	n = 1000, dt = 2.268s, r = 441/s, t = 2267999ns/op
main (917766144 bytes):        	n = 1000, dt = 2.480s, r = 403/s, t = 2480000ns/op
main (1048838144 bytes):       	n = 1000, dt = 2.794s, r = 358/s, t = 2794000ns/op
main (1179910144 bytes):       	n = 1000, dt = 2.584s, r = 387/s, t = 2584000ns/op
main (1310982144 bytes):       	n = 1000, dt = 2.625s, r = 381/s, t = 2625000ns/op
main (1442054144 bytes):       	n = 1000, dt = 2.725s, r = 367/s, t = 2725000ns/op
main (1573126144 bytes):       	n = 1000, dt = 2.793s, r = 358/s, t = 2793000ns/op
main (1704198144 bytes):       	n = 1000, dt = 2.851s, r = 351/s, t = 2851000ns/op
main (1835270144 bytes):       	n = 1000, dt = 2.984s, r = 335/s, t = 2984000ns/op
main (1966342144 bytes):       	n = 1000, dt = 3.135s, r = 319/s, t = 3134999ns/op
main (2097414144 bytes):       	n = 1000, dt = 3.371s, r = 297/s, t = 3371000ns/op
main (2228486144 bytes):       	n = 1000, dt = 3.319s, r = 301/s, t = 3319000ns/op
main (2359558144 bytes):       	n = 1000, dt = 3.598s, r = 278/s, t = 3598000ns/op
main (2490630144 bytes):       	n = 1000, dt = 3.500s, r = 286/s, t = 3500000ns/op
main (2621702144 bytes):       	n = 1000, dt = 3.655s, r = 274/s, t = 3655000ns/op
main (2752774144 bytes):       	n = 1000, dt = 3.707s, r = 270/s, t = 3707000ns/op
main (2883846144 bytes):       	n = 1000, dt = 3.776s, r = 265/s, t = 3776000ns/op
```

```
node examples/node.mjs

main (262144 bytes):           	n = 1000, dt = 0.186s, r = 5376/s, t = 186000ns/op
main (131334144 bytes):        	n = 1000, dt = 0.405s, r = 2469/s, t = 405000ns/op
main (262406144 bytes):        	n = 1000, dt = 0.455s, r = 2198/s, t = 455000ns/op
main (393478144 bytes):        	n = 1000, dt = 0.509s, r = 1965/s, t = 509000ns/op
main (524550144 bytes):        	n = 1000, dt = 0.591s, r = 1692/s, t = 591000ns/op
main (655622144 bytes):        	n = 1000, dt = 0.644s, r = 1553/s, t = 644000ns/op
main (786694144 bytes):        	n = 1000, dt = 0.700s, r = 1429/s, t = 700000ns/op
main (917766144 bytes):        	n = 1000, dt = 0.777s, r = 1287/s, t = 777000ns/op
main (1048838144 bytes):       	n = 1000, dt = 0.850s, r = 1176/s, t = 850000ns/op
main (1179910144 bytes):       	n = 1000, dt = 0.944s, r = 1059/s, t = 944000ns/op
main (1310982144 bytes):       	n = 1000, dt = 1.014s, r = 986/s, t = 1013999ns/op
main (1442054144 bytes):       	n = 1000, dt = 1.075s, r = 930/s, t = 1075000ns/op
main (1573126144 bytes):       	n = 1000, dt = 1.141s, r = 876/s, t = 1141000ns/op
main (1704198144 bytes):       	n = 1000, dt = 1.190s, r = 840/s, t = 1189999ns/op
main (1835270144 bytes):       	n = 1000, dt = 1.273s, r = 786/s, t = 1272999ns/op
main (1966342144 bytes):       	n = 1000, dt = 1.368s, r = 731/s, t = 1368000ns/op
main (2097414144 bytes):       	n = 1000, dt = 2.820s, r = 355/s, t = 2820000ns/op
main (2228486144 bytes):       	n = 1000, dt = 3.767s, r = 265/s, t = 3767000ns/op
main (2359558144 bytes):       	n = 1000, dt = 3.885s, r = 257/s, t = 3885000ns/op
main (2490630144 bytes):       	n = 1000, dt = 4.062s, r = 246/s, t = 4062000ns/op
main (2621702144 bytes):       	n = 1000, dt = 4.256s, r = 235/s, t = 4256000ns/op
main (2752774144 bytes):       	n = 1000, dt = 4.303s, r = 232/s, t = 4302999ns/op
main (2883846144 bytes):       	n = 1000, dt = 4.401s, r = 227/s, t = 4400999ns/op
```

```
bun examples/bun.mjs

main (262144 bytes):           	n = 1000, dt = 0.209s, r = 4785/s, t = 208999ns/op
main (131334144 bytes):        	n = 1000, dt = 0.205s, r = 4878/s, t = 205000ns/op
main (262406144 bytes):        	n = 1000, dt = 0.206s, r = 4854/s, t = 206000ns/op
main (393478144 bytes):        	n = 1000, dt = 0.194s, r = 5155/s, t = 194000ns/op
main (524550144 bytes):        	n = 1000, dt = 0.190s, r = 5263/s, t = 190000ns/op
main (655622144 bytes):        	n = 1000, dt = 0.200s, r = 5000/s, t = 200000ns/op
main (786694144 bytes):        	n = 1000, dt = 0.207s, r = 4831/s, t = 207000ns/op
main (917766144 bytes):        	n = 1000, dt = 0.208s, r = 4808/s, t = 208000ns/op
main (1048838144 bytes):       	n = 1000, dt = 0.205s, r = 4878/s, t = 205000ns/op
main (1179910144 bytes):       	n = 1000, dt = 0.210s, r = 4762/s, t = 209999ns/op
main (1310982144 bytes):       	n = 1000, dt = 0.223s, r = 4484/s, t = 223000ns/op
main (1442054144 bytes):       	n = 1000, dt = 0.220s, r = 4545/s, t = 220000ns/op
main (1573126144 bytes):       	n = 1000, dt = 0.245s, r = 4082/s, t = 245000ns/op
main (1704198144 bytes):       	n = 1000, dt = 0.231s, r = 4329/s, t = 231000ns/op
main (1835270144 bytes):       	n = 1000, dt = 0.225s, r = 4444/s, t = 225000ns/op
main (1966342144 bytes):       	n = 1000, dt = 0.219s, r = 4566/s, t = 219000ns/op
main (2097414144 bytes):       	n = 1000, dt = 0.209s, r = 4785/s, t = 208999ns/op
main (2228486144 bytes):       	n = 1000, dt = 0.304s, r = 3289/s, t = 304000ns/op
main (2359558144 bytes):       	n = 1000, dt = 0.301s, r = 3322/s, t = 301000ns/op
main (2490630144 bytes):       	n = 1000, dt = 0.304s, r = 3289/s, t = 304000ns/op
main (2621702144 bytes):       	n = 1000, dt = 0.301s, r = 3322/s, t = 301000ns/op
main (2752774144 bytes):       	n = 1000, dt = 0.294s, r = 3401/s, t = 294000ns/op
main (2883846144 bytes):       	n = 1000, dt = 0.288s, r = 3472/s, t = 287999ns/op
```