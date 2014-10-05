CC = clang
CFLAGS = -std=c99 -pedantic -O3 -Weverything -pipe -fomit-frame-pointer -march=native -pthread

RC= rustc
RCFLAGS = --opt-level=3

revcomp_rs: revcomp.rs
	$(RC) $(RCFLAGS) revcomp.rs -o revcomp_rs

revcomp_c: revcomp.c
	$(CC) $(CFLAGS) revcomp.c -o revcomp_c

clean:
	rm -f revcomp_c revcomp_rs stack task a.out *.o *.a