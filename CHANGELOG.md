# Changelog

## [1.1.0](https://github.com/Realsnack/snacks-awesome-shortener/compare/v1.0.1...v1.1.0) (2026-02-06)


### Features

* add monitoring user to mongodb ([33eae39](https://github.com/Realsnack/snacks-awesome-shortener/commit/33eae3970b4403440ac2c94d113aef54d56c2ea8))
* add support for redis in sentinel mode ([e1ee9ef](https://github.com/Realsnack/snacks-awesome-shortener/commit/e1ee9ef7104f372bb4fc4cafbc4fd0e8bda07de1))
* added endpoint to get health per service ([50b1972](https://github.com/Realsnack/snacks-awesome-shortener/commit/50b1972b9f8fb48f97698f499d994a7c12f60a16))
* added mongo health check ([0d215e8](https://github.com/Realsnack/snacks-awesome-shortener/commit/0d215e821596fd5357fee4d46cab3f506b400c27))
* Added sentinel support to redis service ([669eb40](https://github.com/Realsnack/snacks-awesome-shortener/commit/669eb40781f61a39673e1af1c3ec9549ef8ee103))
* Created redis health check ([9cb9865](https://github.com/Realsnack/snacks-awesome-shortener/commit/9cb9865bee0f3e711339d77569c58e37654abb93))
* created request/response models ([9df9b44](https://github.com/Realsnack/snacks-awesome-shortener/commit/9df9b4490c7cff163b6f4eb481a40ff40374de94))
* POST Short uses Request model ([73b10ee](https://github.com/Realsnack/snacks-awesome-shortener/commit/73b10eefd90a313d1c07ac7cf2cf42f7152ec14e))
* refactored redis health check ([1234c28](https://github.com/Realsnack/snacks-awesome-shortener/commit/1234c28005890b77231de3e9471a03ce79732df4))
* Use HealthResponse for health endpoint ([192315e](https://github.com/Realsnack/snacks-awesome-shortener/commit/192315eb6c013f852f454fdd6e9fa78d717c095f))


### Bug Fixes

* Post short is unavailable only when both mongo and redis are unavailable ([385d3b8](https://github.com/Realsnack/snacks-awesome-shortener/commit/385d3b8616deaed8e9d4035e38efccecbe5571f1))

## [1.0.1](https://github.com/Realsnack/snacks-awesome-shortener/compare/v1.0.0...v1.0.1) (2025-11-30)


### Bug Fixes

* POST empty body returns string response instead of json ([c179b10](https://github.com/Realsnack/snacks-awesome-shortener/commit/c179b10f97edecc992889d89fb62859f90837a8c))
* static ip/port assignment ([3536b14](https://github.com/Realsnack/snacks-awesome-shortener/commit/3536b14eac73aee0eda164da61656efcfc398b51))

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
