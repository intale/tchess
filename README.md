# TChess

The implementation of chess engine using Rust. The main focus of this engine is to support boards with dimension of up to 2^16 x 2^16 squares while keeping acceptable time to calculate next move. The core functional is under the development currently.

The project consists of several packages:
- **libtchess** - the implementation of core game mechanics, moves evaluation, moves generator
- **tchess_classic** - the implementation of classic chess game rules

## License

This project is available as open source under the terms of the [BSD-2-Clause license](LICENSE).

## Code of Conduct

Everyone interacting in the tchess project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](CODE_OF_CONDUCT.md).
