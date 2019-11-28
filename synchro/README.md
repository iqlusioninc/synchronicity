# synchro.rs <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
![MSRV][msrv-image]
[![Apache 2.0 Licensed][license-image]][license-link]
[![Gitter Chat][gitter-image]][gitter-link]

Byzantine Fault Tolerant consensus library built on [HotStuff BFT][hotstuff]
used by the [Synchronicity] reproducible build system.

## Status

**synchro** is a work-in-progress and at an early stage of development
and is not ready to be used. Check back later!

## Minimum Supported Rust Version

- Rust **1.39+**

## Code of Conduct

We abide by the [Contributor Covenant][cc-md] and ask that you do as well.

For more information, please see [CODE_OF_CONDUCT.md][cc-md].

## License

Copyright © 2019 iqlusion

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[build-image]: https://github.com/iqlusioninc/synchronicity/workflows/Rust/badge.svg
[build-link]: https://github.com/iqlusioninc/synchronicity/actions
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety-link]: https://github.com/rust-secure-code/safety-dance/
[msrv-image]: https://img.shields.io/badge/rustc-1.39+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/synchronicity/blob/master/LICENSE
[gitter-image]: https://badges.gitter.im/badge.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (general links)

[hotstuff]: https://github.com/libra/libra/tree/master/consensus
[synchronicity]: https://github.com/iqlusioninc/synchronicity
[cc-web]: https://contributor-covenant.org/
[cc-md]: https://github.com/iqlusioninc/synchronicity/blob/develop/CODE_OF_CONDUCT.md
