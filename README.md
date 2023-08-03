# rsffi

Rust FFI 를 테스트해본 과정 기록.  
https://github.com/rust-unofficial/awesome-rust#ffi  

시도해본 타겟 언어: C, C++, C#, WASM, Python, Kotlin

# C
* install `cbindgen`
```shell
$ cargo install --force cbindgen
```
* `cbindgen` 이용해서 헤더 파일 생성
```shell
$ cbindgen --config cbindgen.toml --crate <YOUR_CRATE_NAME> --output <YOUR_HEADER_NAME>.h --lang c
```
* DLL 빌드
```shell
$ cargo build
```
빌드 시 `target` 디렉토리에 DLL 이 생성되고 이 DLL 을 가져와서 C 코드 빌드 시 사용하면 된다.  
[샘플코드](./c) 의 `Makefile` 참조.

# C++
C 와 거의 유사하게 빌드하면 된다. 대신, `cbindgen` 으로 헤더 생성 시 `--lang c` 를 빼고 이렇게 헤더를 생성해준다.
```shell
$ cbindgen --config cbindgen.toml --crate <YOUR_CRATE_NAME> --output <YOUR_HEADER_NAME>.h
```

# C#
[csbindgen](https://github.com/Cysharp/csbindgen) 를 사용해서 FFI 를 생성.


# WASM
* install wasm-pack
```shell
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
* Build
```shell
$ cd ./wasm
$ wasm-pack build
```
* 생성된 wasm 모듈 사용 방법은 [샘플 코드](./wasm/index.html) 참조

# python, kotlin, swift, ruby (uniffi-rs)
* [uniffi-rs](https://github.com/mozilla/uniffi-rs) 를 사용하면 언어별로 FFI 를 만들 필요 없이 UDL 하나만 정의해서 사용할 수 있다.
* [공식 가이드](https://mozilla.github.io/uniffi-rs/Getting_started.html) 와 [샘플 코드](./udl), [uniffi-bindgen](./uniffi-bindgen) 참고해서 코드 구성.
* 가이드에 따라 코드를 구성한 다음, `uniffi-bindgen` CLI 를 이용해서 각 언어별 FFI 코드를 만들어준다.
```shell
$ cargo run -p uniffi-bindgen generate <PATH_TO_YOUR_UDL_FILE> --language <YOUR_LANGUAGE> --out-dir out
$ cargo run -p uniffi-bindgen generate ./src/awesomemodule.udl --language python --out-dir out
```
* 참고로, 생성된 binding 을 사용할 때 빌드한 dll 파일을 언어별로 적절한 곳에 위치시켜야 한다.
