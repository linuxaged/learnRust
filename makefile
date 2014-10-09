CC = clang
CFLAGS = -std=c99 -pedantic -O3 -Weverything -pipe -fomit-frame-pointer -march=native -pthread
CFLAGS2 = -pipe -Wall -O3 -fomit-frame-pointer -march=native -std=c99 -D_GNU_SOURCE -mfpmath=sse -msse2 -fopenmp

RC= rustc
RCFLAGS = --opt-level=3

revcomp_rs: revcomp.rs
	$(RC) $(RCFLAGS) revcomp.rs -o revcomp_rs

revcomp_c: revcomp.c
	$(CC) $(CFLAGS) revcomp.c -o revcomp_c

benchmark_rs: revcomp_rs
	time ./revcomp_rs 0 < revcomp-input.txt

benchmark_c: revcomp_c
	time ./revcomp_c 0 < revcomp-input.txt

bench_rs: revcomp_rs
	$(RC) $(RCFLAGS) revcomp.rs -o bench_rs

mandelbrot_rs: mandelbrot.rs
	$(RC) $(RCFLAGS) mandelbrot.rs -o mandelbrot_rs

mandelbrot_c: mandelbrot.c
	$(CC) $(CFLAGS) revcomp.c -o revcomp_c

clean:
	rm -f bench_rs newrevcomp revcomp_c revcomp_rs stack