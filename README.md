# Rust 시작

```shell
# 프로젝트 생성
cargo new rust_feature

# 프로젝트 빌드(바이너리 실행파일 출력)
cargo build

# cargo-expand 설치
cargo install cargo-expand

# 컴파일러 내부에서 어떻게 동작하는지 확인
cargo expand

```


- 참고 : https://rinthel.github.io/rust-lang-book-ko/foreword.html


# Cargo.toml

- 의존 라이브러리를 명시할 수 있다
    - 메타데이터, 컴파일러 셋팅 등
- Rust에서는 패키지를 크레이트라고 부른다
    - crates.io 에서 확인 할 수 있다
    - vscode의 경우 crates extension을 통해 보조받을 수 있음