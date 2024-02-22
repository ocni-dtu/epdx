## [1.1.5](https://github.com/ocni-dtu/epdx/compare/v1.1.4...v1.1.5) (2024-02-22)


### Bug Fixes

* handling non-english EPDs ([87fe672](https://github.com/ocni-dtu/epdx/commit/87fe67207d712c53726314d772cf183b9206f897))

## [1.1.4](https://github.com/ocni-dtu/epdx/compare/v1.1.3...v1.1.4) (2024-02-19)


### Bug Fixes

* made Source fields public ([b3e9c05](https://github.com/ocni-dtu/epdx/commit/b3e9c05e1cfc2dbbf66e9cf44ecde0e9e45c57a4))

## [1.1.3](https://github.com/ocni-dtu/epdx/compare/v1.1.2...v1.1.3) (2024-02-19)


### Bug Fixes

* made EPD fields public ([e14238d](https://github.com/ocni-dtu/epdx/commit/e14238d2559249e38ed22e9ded8c4e86012d26fd))

## [1.1.2](https://github.com/ocni-dtu/epdx/compare/v1.1.1...v1.1.2) (2024-02-18)


### Bug Fixes

* added generics to epdx.convert_ilcd ([538ae5e](https://github.com/ocni-dtu/epdx/commit/538ae5e981438556aa3aa936061816125f72895f))
* fixed detection of en15804+a2 ([d0de2fe](https://github.com/ocni-dtu/epdx/commit/d0de2fea726b15102de8de36490fae6c6beff7c5))
* fixed issues parsing EPD 023f3b97 ([7c43544](https://github.com/ocni-dtu/epdx/commit/7c43544b93ac6ccde8a2b9ab4a26d1fd5d1492e5))

## [1.1.1](https://github.com/ocni-dtu/epdx/compare/v1.1.0...v1.1.1) (2024-02-16)


### Bug Fixes

* updated pydantic version and allowing dicts as arguments to convert_ilcd ([34fdaa9](https://github.com/ocni-dtu/epdx/commit/34fdaa94cfbc09626991bbfa8b7cc4296f39447d))

## [1.1.0](https://github.com/ocni-dtu/epdx/compare/v1.0.0...v1.1.0) (2024-02-05)


### Features

* added km and tones_km to units ([08f6480](https://github.com/ocni-dtu/epdx/commit/08f6480464a3583db6955d21d9e2b57279c9398c))
* added meta_data to conversion struct ([b4fa5c1](https://github.com/ocni-dtu/epdx/commit/b4fa5c1254691c1263b5048570675d300661bfd7))


### Bug Fixes

* refactored ILCD parsing functions and added more test cases for ILCD files ([35256b6](https://github.com/ocni-dtu/epdx/commit/35256b672c828283c262c5affdcc7f85f10575c1))

## 1.0.0 (2024-01-24)


### Features

* added error handling to ilcd parsing ([bc9beb9](https://github.com/ocni-dtu/epdx/commit/bc9beb9f478fa4f26292f741cfe4fe04e319360d))
* improved support for js and ts ([c938488](https://github.com/ocni-dtu/epdx/commit/c9384887d395adba7a53ca13ce6b14f6332f5130))
* made it optional to whether convert_ilcd should return string, dict or pydantic ([720ca62](https://github.com/ocni-dtu/epdx/commit/720ca6284597c5f79ef600a4abcc16fa311e9976))
