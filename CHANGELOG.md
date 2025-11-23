# Changelog

## [1.0.0](https://github.com/Realsnack/snacks-awesome-shortener/compare/v0.1.0...v1.0.0) (2025-11-23)


### ⚠ BREAKING CHANGES

* Axum rewrite ([#4](https://github.com/Realsnack/snacks-awesome-shortener/issues/4))

### Features

* Add version info to startup log ([4f5540e](https://github.com/Realsnack/snacks-awesome-shortener/commit/4f5540efc7734c2001388068030a6aac252562b9))
* Axum rewrite ([#4](https://github.com/Realsnack/snacks-awesome-shortener/issues/4)) ([2e30bfa](https://github.com/Realsnack/snacks-awesome-shortener/commit/2e30bfa8210067cbedfdf10e10bc2ea2b14eb690))
* Created traits for services ([2099748](https://github.com/Realsnack/snacks-awesome-shortener/commit/2099748b78ecbe2d39d0ca70363aaffdb367dada))
* Minor startup logging change ([6ff96a1](https://github.com/Realsnack/snacks-awesome-shortener/commit/6ff96a15014e18d3810328bcb84a387510c78d2a))
* store shorts in mongodb ([a3fb025](https://github.com/Realsnack/snacks-awesome-shortener/commit/a3fb0254f3888f094e5e9e6c528073a5c2426d42))


### Bug Fixes

* clippy warnings ([ce86f90](https://github.com/Realsnack/snacks-awesome-shortener/commit/ce86f905a19052e5b1a59a28befc9ba48c8abd07))
* incorrect logging on nil key in redis ([66f4590](https://github.com/Realsnack/snacks-awesome-shortener/commit/66f4590ef33e3bfc7374779ca0438a279ece8422))

## 0.1.0 (2025-11-19)


### Features

* add environement variable for used port ([8960ef7](https://github.com/Realsnack/snacks-awesome-shortener/commit/8960ef70d92ea1122607cc37415ae3d862bb1072))
* add ShortUrl model ([00f5a3a](https://github.com/Realsnack/snacks-awesome-shortener/commit/00f5a3a01b438bfc27fce2a328380eeca02a89d6))
* added femme logging ([c5e878f](https://github.com/Realsnack/snacks-awesome-shortener/commit/c5e878f88f6acd2ce84de0144ceed31323b870de))
* Configurable redis url ([3351a38](https://github.com/Realsnack/snacks-awesome-shortener/commit/3351a386f947639679f1e74f341cf37560f02ea5))
* create shorts service and handler ([0ec40df](https://github.com/Realsnack/snacks-awesome-shortener/commit/0ec40df0a40c96adce124af9c997fcb9440447f0))
* created redis service ([532d5a1](https://github.com/Realsnack/snacks-awesome-shortener/commit/532d5a1fa58bf8ab17d9d937264a010a0bd79b04))
* created root_routes ([14296ff](https://github.com/Realsnack/snacks-awesome-shortener/commit/14296ff1028b3082fe9e56e22aee94efc99ec1fd))
* Implemented URL shortening and saving to redis. Redirect for /:short and json object for /short/:short ([526b9f7](https://github.com/Realsnack/snacks-awesome-shortener/commit/526b9f7512008aa56640be30ab4f3d0968e041f7))


### Bug Fixes

* All code warnings ([c3a8d65](https://github.com/Realsnack/snacks-awesome-shortener/commit/c3a8d658c39e92907742f345a2b69329a4b6344c))
* GET shorts error handling ([1d8ccbb](https://github.com/Realsnack/snacks-awesome-shortener/commit/1d8ccbbe46664af044215e3bf8c1e66540607b71))
* incorrect default redis url ([cdb2509](https://github.com/Realsnack/snacks-awesome-shortener/commit/cdb2509a280b11b9852f3c32e6af230fea71f660))
* refactored redis_service.rs ([563e831](https://github.com/Realsnack/snacks-awesome-shortener/commit/563e83115bbe9a113be67244830fe9c652f4cacc))
* refactored short_handler.rs and shorts_service.rs ([bd10420](https://github.com/Realsnack/snacks-awesome-shortener/commit/bd104208a0dfd4df3fe80847cfa91c1804b2f6b4))
