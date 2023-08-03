main: sample.c cffi.h libcffi.dylib
	gcc -c sample.c
	gcc sample.o ./libcffi.dylib -o sample.out

cffi.h:
	cbindgen --config cbindgen.toml --crate c --output cffi.h --lang c

libcffi.dylib:
	cargo build
	cp ../target/debug/libcffi.dylib ./

clean:
	rm *.h
	rm *.o
	rm *.dylib
	rm *.out