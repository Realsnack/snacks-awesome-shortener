# Changelog

## [2.0.0](https://github.com/Realsnack/snacks-awesome-shortener/compare/shorts_service-v1.0.0...shorts_service-v2.0.0) (2026-06-05)


### ⚠ BREAKING CHANGES

* Rewrite messages into ProtoBuf from MessagePack

### Features

* **#18:** Implement common headers function in nats ([d1eda66](https://github.com/Realsnack/snacks-awesome-shortener/commit/d1eda665d705ede79493a95bf5025a9ab5f17581))
* All messages converted to ProtoBuf ([495aad3](https://github.com/Realsnack/snacks-awesome-shortener/commit/495aad33fa1323a033aa201edbb8f5e302d50d46))
* Api gateway can create shorts ([ed57223](https://github.com/Realsnack/snacks-awesome-shortener/commit/ed57223b00fd4eebdf842bb0a104c202ce93d9d0))
* Create get_header_value common function ([7c01d62](https://github.com/Realsnack/snacks-awesome-shortener/commit/7c01d62c35e9ed7df59a30289a6f8371e3c5a2a5))
* create_common_headers function ([ac09474](https://github.com/Realsnack/snacks-awesome-shortener/commit/ac09474833adc97387f51bec3120305991207220))
* created api_gateway consumer task ([54f7947](https://github.com/Realsnack/snacks-awesome-shortener/commit/54f79470113ce4b3a066ba471425262e2e1460e5))
* Created common nats_utils for consumers ([3696953](https://github.com/Realsnack/snacks-awesome-shortener/commit/3696953d9fdd5bd97aed34c8b6400351359c087d))
* CreateShortCommand protobuff prototype ([7134930](https://github.com/Realsnack/snacks-awesome-shortener/commit/7134930a89f2dc81b3fa853110df86ca6db6dd5b))
* Implemented create short in short_service ([2d825a8](https://github.com/Realsnack/snacks-awesome-shortener/commit/2d825a8154ce75b5a1addadbc30a9f3e277db171))
* Persist messages in postgres ([9416141](https://github.com/Realsnack/snacks-awesome-shortener/commit/94161413dace17ad8fd89c8254fdd6ea57081872))
* Removal of unused lib ([9a3a279](https://github.com/Realsnack/snacks-awesome-shortener/commit/9a3a2792b2c38196bc3f79b7854628b8b02bbc3d))
* RetrieveShortsCommand returns a ShortRetrievedEvent to api_gateway::response ([0214ab6](https://github.com/Realsnack/snacks-awesome-shortener/commit/0214ab65bfdfdcf3809170346f1970a38436f894))
* Reusable jetstream in consumer based services ([#30](https://github.com/Realsnack/snacks-awesome-shortener/issues/30)) ([e2121ca](https://github.com/Realsnack/snacks-awesome-shortener/commit/e2121cad3e8196a3e85fd2beb2f079981a5c6588)), closes [#24](https://github.com/Realsnack/snacks-awesome-shortener/issues/24)
* Rewrite messages into ProtoBuf from MessagePack ([0696ea7](https://github.com/Realsnack/snacks-awesome-shortener/commit/0696ea7cbd9ee97b8104266a36b9c035238d24cf))
* short-service implementation ([c98fe7b](https://github.com/Realsnack/snacks-awesome-shortener/commit/c98fe7b4a320e7177804b9d52aef153583476347))
* Unified config crate ([#32](https://github.com/Realsnack/snacks-awesome-shortener/issues/32)) ([a525db1](https://github.com/Realsnack/snacks-awesome-shortener/commit/a525db14a9168799279521d981cf5e82ad4f9670))


### Bug Fixes

* Clippy warning [#87](https://github.com/Realsnack/snacks-awesome-shortener/issues/87) ([db8ee23](https://github.com/Realsnack/snacks-awesome-shortener/commit/db8ee230ef81d22264011c4dfe2ebd8f833cf496))
* Fix clippy warning ([aa02e8a](https://github.com/Realsnack/snacks-awesome-shortener/commit/aa02e8a7f8a5599581f85467765b562cb6d181a1))
* incorrect hard-coded headers ([f9d4773](https://github.com/Realsnack/snacks-awesome-shortener/commit/f9d4773925fa9636ead676d0cedb27ebed605c6e))
* Short service doesnt recognize CreateShortCommand ([3232efc](https://github.com/Realsnack/snacks-awesome-shortener/commit/3232efc2c638801a6bc834e14eb4d812d012cd0f))
* Unused variables and imports ([8d54286](https://github.com/Realsnack/snacks-awesome-shortener/commit/8d5428659e29459f969a2e077e275d2c83f0f1e8))
