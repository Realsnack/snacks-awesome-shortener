# Changelog

## [2.0.0](https://github.com/Realsnack/snacks-awesome-shortener/compare/data_persistor-v1.0.0...data_persistor-v2.0.0) (2026-06-05)


### ⚠ BREAKING CHANGES

* Rewrite messages into ProtoBuf from MessagePack
* Change expiration from usize to u64

### Features

* **#18:** Implement common headers function in nats ([d1eda66](https://github.com/Realsnack/snacks-awesome-shortener/commit/d1eda665d705ede79493a95bf5025a9ab5f17581))
* **#22:** TypeString trait to get string of type name ([#26](https://github.com/Realsnack/snacks-awesome-shortener/issues/26)) ([47d47ef](https://github.com/Realsnack/snacks-awesome-shortener/commit/47d47efb49d1e586a1041c79502f7ccf94a21019))
* Add retrieve short in data_persistor ([b3e3e09](https://github.com/Realsnack/snacks-awesome-shortener/commit/b3e3e09c58b9468b1e157d1dba3488ec753c40ca))
* All messages converted to ProtoBuf ([495aad3](https://github.com/Realsnack/snacks-awesome-shortener/commit/495aad33fa1323a033aa201edbb8f5e302d50d46))
* Change expiration from usize to u64 ([66ac5a7](https://github.com/Realsnack/snacks-awesome-shortener/commit/66ac5a769107454751bb0494431f4aa5bd51002c))
* Create get_header_value common function ([7c01d62](https://github.com/Realsnack/snacks-awesome-shortener/commit/7c01d62c35e9ed7df59a30289a6f8371e3c5a2a5))
* create_common_headers function ([ac09474](https://github.com/Realsnack/snacks-awesome-shortener/commit/ac09474833adc97387f51bec3120305991207220))
* created api_gateway consumer task ([54f7947](https://github.com/Realsnack/snacks-awesome-shortener/commit/54f79470113ce4b3a066ba471425262e2e1460e5))
* Created common nats_utils for consumers ([3696953](https://github.com/Realsnack/snacks-awesome-shortener/commit/3696953d9fdd5bd97aed34c8b6400351359c087d))
* data_persistor deserialize PersistenceRequest ([07bb5f4](https://github.com/Realsnack/snacks-awesome-shortener/commit/07bb5f47e7defcd7c7c503bf2c03d347c1499a37))
* Implemented base messaging logic for data_persistor ([d663f77](https://github.com/Realsnack/snacks-awesome-shortener/commit/d663f7705127a890ab0d6c9b6bb8beb5ebc83438))
* Implemented create short in short_service ([2d825a8](https://github.com/Realsnack/snacks-awesome-shortener/commit/2d825a8154ce75b5a1addadbc30a9f3e277db171))
* Persist messages in postgres ([9416141](https://github.com/Realsnack/snacks-awesome-shortener/commit/94161413dace17ad8fd89c8254fdd6ea57081872))
* RetrieveShortsCommand returns a ShortRetrievedEvent to api_gateway::response ([0214ab6](https://github.com/Realsnack/snacks-awesome-shortener/commit/0214ab65bfdfdcf3809170346f1970a38436f894))
* Reusable jetstream in consumer based services ([#30](https://github.com/Realsnack/snacks-awesome-shortener/issues/30)) ([e2121ca](https://github.com/Realsnack/snacks-awesome-shortener/commit/e2121cad3e8196a3e85fd2beb2f079981a5c6588)), closes [#24](https://github.com/Realsnack/snacks-awesome-shortener/issues/24)
* Rewrite messages into ProtoBuf from MessagePack ([0696ea7](https://github.com/Realsnack/snacks-awesome-shortener/commit/0696ea7cbd9ee97b8104266a36b9c035238d24cf))
* Unified config crate ([#32](https://github.com/Realsnack/snacks-awesome-shortener/issues/32)) ([a525db1](https://github.com/Realsnack/snacks-awesome-shortener/commit/a525db14a9168799279521d981cf5e82ad4f9670))


### Bug Fixes

* Fix clippy warning ([aa02e8a](https://github.com/Realsnack/snacks-awesome-shortener/commit/aa02e8a7f8a5599581f85467765b562cb6d181a1))
* function to mark expired short when short has never been retrieved ([645343b](https://github.com/Realsnack/snacks-awesome-shortener/commit/645343bea4e97d8286dec8ac654b14fe41fdcd68))
* return 404 response for non-existent short ([#37](https://github.com/Realsnack/snacks-awesome-shortener/issues/37)) ([a16d6cc](https://github.com/Realsnack/snacks-awesome-shortener/commit/a16d6cce55ba6a6d4b450a2c9165a55b275f5701))
* Unused variables and imports ([8d54286](https://github.com/Realsnack/snacks-awesome-shortener/commit/8d5428659e29459f969a2e077e275d2c83f0f1e8))
