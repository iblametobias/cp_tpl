## Very, very spermous template for competitive programming.
### Features
#### Input parsing
**`cp::StandardInputParser`**:
- 2x faster than C++'s `cin`
- supports all types via generic `get<T: std::str::FromStr>()`
- helper methods `get_T()` for `usize`, `u32`, `u64`, `isize`, `i32`, `i64`
- parsing via default `.parse()`
  
**`cp::FastInputParser`**:
- 5% faster than C++ fast inputs using `getchar_unchecked()`
- supports only numeric types `usize`, `u32`, `u64`, `isize`, `i32`, `i64`
- handwritten parsing assuming correct input
#### Testing and testgen
Soon twinkies trust me 🔜
