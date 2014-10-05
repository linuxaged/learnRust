CC = clang
CFLAGS = -std=c99 -pedantic -O3 -Weverything -pipe -fomit-frame-pointer -march=native -pthread

revcomp_rs: revcomp.rs
	rustc --opt-level=3 -o revcomp_rs

revcomp_c: revcomp.c
	$(CC) $(CFLAGS) revcomp.c -o revcomp_c

clean:
	rm -f revcomp_c revcomp_rs stack task a.out *.o *.a