# Changelog

## [2.0.0](https://github.com/Realsnack/snacks-awesome-shortener/compare/common-v1.0.0...common-v2.0.0) (2026-06-05)


### ⚠ BREAKING CHANGES

* Rewrite messages into ProtoBuf from MessagePack
* Change expiration from usize to u64

### Features

* **#18:** Implement common headers function in nats ([d1eda66](https://github.com/Realsnack/snacks-awesome-shortener/commit/d1eda665d705ede79493a95bf5025a9ab5f17581))
* **#22:** TypeString trait to get string of type name ([#26](https://github.com/Realsnack/snacks-awesome-shortener/issues/26)) ([47d47ef](https://github.com/Realsnack/snacks-awesome-shortener/commit/47d47efb49d1e586a1041c79502f7ccf94a21019))
* Add database config and common pg lib ([59afa3d](https://github.com/Realsnack/snacks-awesome-shortener/commit/59afa3d12048f96f12a95496726e4eaef0554e32))
* Add DeliveryPolicy::New to pull consumers ([dc9beee](https://github.com/Realsnack/snacks-awesome-shortener/commit/dc9beee328fd5ca9c88e037b0039aa0ab3f70c5a))
* Add protobuff support for all messaging models ([8f0ff9b](https://github.com/Realsnack/snacks-awesome-shortener/commit/8f0ff9bc066d8156b2e5ef19c35d9dcb9e1fd4c5))
* All messages converted to ProtoBuf ([495aad3](https://github.com/Realsnack/snacks-awesome-shortener/commit/495aad33fa1323a033aa201edbb8f5e302d50d46))
* Api gateway can create shorts ([ed57223](https://github.com/Realsnack/snacks-awesome-shortener/commit/ed57223b00fd4eebdf842bb0a104c202ce93d9d0))
* api_gateway PoC ([7e23fae](https://github.com/Realsnack/snacks-awesome-shortener/commit/7e23faeb4ef3ae50ad137a6f5ca073a09795e546))
* APIs to retrieve shorts ([#33](https://github.com/Realsnack/snacks-awesome-shortener/issues/33)) ([d10f6ad](https://github.com/Realsnack/snacks-awesome-shortener/commit/d10f6ade270f5a32d30c9499e7ba4f1b0f590115))
* Change expiration from usize to u64 ([66ac5a7](https://github.com/Realsnack/snacks-awesome-shortener/commit/66ac5a769107454751bb0494431f4aa5bd51002c))
* Configurable nats common lib ([b88c9c4](https://github.com/Realsnack/snacks-awesome-shortener/commit/b88c9c406ae67a1295c24a0ff946af8f0aabc1eb))
* Create get_header_value common function ([7c01d62](https://github.com/Realsnack/snacks-awesome-shortener/commit/7c01d62c35e9ed7df59a30289a6f8371e3c5a2a5))
* create_common_headers function ([ac09474](https://github.com/Realsnack/snacks-awesome-shortener/commit/ac09474833adc97387f51bec3120305991207220))
* created api_gateway consumer task ([54f7947](https://github.com/Realsnack/snacks-awesome-shortener/commit/54f79470113ce4b3a066ba471425262e2e1460e5))
* Created common config ([bd8269e](https://github.com/Realsnack/snacks-awesome-shortener/commit/bd8269ec11d1377d28824a0023c9d0e3849db38c))
* Created common library ([3f866ef](https://github.com/Realsnack/snacks-awesome-shortener/commit/3f866ef7a86dec87f771c86dfc5fdef74d21a3d9))
* Created common nats_utils for consumers ([3696953](https://github.com/Realsnack/snacks-awesome-shortener/commit/3696953d9fdd5bd97aed34c8b6400351359c087d))
* Created commong logging ([8ecb54e](https://github.com/Realsnack/snacks-awesome-shortener/commit/8ecb54e6044bb10f5220305eb980d3cedac10798))
* Created proper rest request model ([4cf2a61](https://github.com/Realsnack/snacks-awesome-shortener/commit/4cf2a6195da8ddd2adb0a84980adde4538713877))
* CreateShortCommand protobuff prototype ([7134930](https://github.com/Realsnack/snacks-awesome-shortener/commit/7134930a89f2dc81b3fa853110df86ca6db6dd5b))
* Implemented create short in short_service ([2d825a8](https://github.com/Realsnack/snacks-awesome-shortener/commit/2d825a8154ce75b5a1addadbc30a9f3e277db171))
* Models for shorts service ([9f1dd9e](https://github.com/Realsnack/snacks-awesome-shortener/commit/9f1dd9e128345768a1f235ea3307218424c7fdb8))
* Persist messages in postgres ([9416141](https://github.com/Realsnack/snacks-awesome-shortener/commit/94161413dace17ad8fd89c8254fdd6ea57081872))
* RetrieveShortsCommand returns a ShortRetrievedEvent to api_gateway::response ([0214ab6](https://github.com/Realsnack/snacks-awesome-shortener/commit/0214ab65bfdfdcf3809170346f1970a38436f894))
* Reusable jetstream in consumer based services ([#30](https://github.com/Realsnack/snacks-awesome-shortener/issues/30)) ([e2121ca](https://github.com/Realsnack/snacks-awesome-shortener/commit/e2121cad3e8196a3e85fd2beb2f079981a5c6588)), closes [#24](https://github.com/Realsnack/snacks-awesome-shortener/issues/24)
* Rewrite messages into ProtoBuf from MessagePack ([0696ea7](https://github.com/Realsnack/snacks-awesome-shortener/commit/0696ea7cbd9ee97b8104266a36b9c035238d24cf))
* Unified config crate ([#32](https://github.com/Realsnack/snacks-awesome-shortener/issues/32)) ([a525db1](https://github.com/Realsnack/snacks-awesome-shortener/commit/a525db14a9168799279521d981cf5e82ad4f9670))


### Bug Fixes

* Add inactive_threshold setting to clear dead nats consumer groups ([8ab0a4d](https://github.com/Realsnack/snacks-awesome-shortener/commit/8ab0a4dcea77b15d1e9d190b20d390cc09614127))
* Fix clippy warning ([aa02e8a](https://github.com/Realsnack/snacks-awesome-shortener/commit/aa02e8a7f8a5599581f85467765b562cb6d181a1))
* Incorrect proto serialization ([a7f589a](https://github.com/Realsnack/snacks-awesome-shortener/commit/a7f589a1d328fce8a439ac43247c109ec656c974))
* Remove unused impl From ([98c7286](https://github.com/Realsnack/snacks-awesome-shortener/commit/98c7286cf0a371765dea943fa817aa129d9f2f36))
* return 404 response for non-existent short ([#37](https://github.com/Realsnack/snacks-awesome-shortener/issues/37)) ([a16d6cc](https://github.com/Realsnack/snacks-awesome-shortener/commit/a16d6cce55ba6a6d4b450a2c9165a55b275f5701))
