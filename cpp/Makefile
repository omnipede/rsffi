main: sample.cpp cppffi.h libcppffi.dylib
	g++ -c sample.cpp
	g++ sample.o ./libcppffi.dylib -o sample.out

cffi.h:
	cbindgen --config cbindgen.toml --crate cpp --output cppffi.h

libcppffi.dylib:
	cargo build
	cp ../target/debug/libcppffi.dylib ./

clean:
	rm *.h
	rm *.o
	rm *.dylib
	rm *.out