# Changelog

## [2.0.0](https://github.com/Realsnack/snacks-awesome-shortener/compare/test_tool-v1.0.0...test_tool-v2.0.0) (2026-06-05)


### ⚠ BREAKING CHANGES

* Rewrite messages into ProtoBuf from MessagePack

### Features

* **#22:** TypeString trait to get string of type name ([#26](https://github.com/Realsnack/snacks-awesome-shortener/issues/26)) ([47d47ef](https://github.com/Realsnack/snacks-awesome-shortener/commit/47d47efb49d1e586a1041c79502f7ccf94a21019))
* Add retrieve short in data_persistor ([b3e3e09](https://github.com/Realsnack/snacks-awesome-shortener/commit/b3e3e09c58b9468b1e157d1dba3488ec753c40ca))
* All messages converted to ProtoBuf ([495aad3](https://github.com/Realsnack/snacks-awesome-shortener/commit/495aad33fa1323a033aa201edbb8f5e302d50d46))
* Api gateway can create shorts ([ed57223](https://github.com/Realsnack/snacks-awesome-shortener/commit/ed57223b00fd4eebdf842bb0a104c202ce93d9d0))
* api_gateway PoC ([7e23fae](https://github.com/Realsnack/snacks-awesome-shortener/commit/7e23faeb4ef3ae50ad137a6f5ca073a09795e546))
* APIs to retrieve shorts ([#33](https://github.com/Realsnack/snacks-awesome-shortener/issues/33)) ([d10f6ad](https://github.com/Realsnack/snacks-awesome-shortener/commit/d10f6ade270f5a32d30c9499e7ba4f1b0f590115))
* Created test_tool for easier testing ([125ab38](https://github.com/Realsnack/snacks-awesome-shortener/commit/125ab387dd919838fd80b93237268ab1ddb9c145))
* CreateShortCommand protobuff prototype ([7134930](https://github.com/Realsnack/snacks-awesome-shortener/commit/7134930a89f2dc81b3fa853110df86ca6db6dd5b))
* Implemented create short in short_service ([2d825a8](https://github.com/Realsnack/snacks-awesome-shortener/commit/2d825a8154ce75b5a1addadbc30a9f3e277db171))
* jetstream consume in testtool ([be8938d](https://github.com/Realsnack/snacks-awesome-shortener/commit/be8938d5fee01afaf5b7245baf4b3b426b4e391b))
* Persist messages in postgres ([9416141](https://github.com/Realsnack/snacks-awesome-shortener/commit/94161413dace17ad8fd89c8254fdd6ea57081872))
* RetrieveShortsCommand returns a ShortRetrievedEvent to api_gateway::response ([0214ab6](https://github.com/Realsnack/snacks-awesome-shortener/commit/0214ab65bfdfdcf3809170346f1970a38436f894))
* Reusable jetstream in consumer based services ([#30](https://github.com/Realsnack/snacks-awesome-shortener/issues/30)) ([e2121ca](https://github.com/Realsnack/snacks-awesome-shortener/commit/e2121cad3e8196a3e85fd2beb2f079981a5c6588)), closes [#24](https://github.com/Realsnack/snacks-awesome-shortener/issues/24)
* Rewrite messages into ProtoBuf from MessagePack ([0696ea7](https://github.com/Realsnack/snacks-awesome-shortener/commit/0696ea7cbd9ee97b8104266a36b9c035238d24cf))
* TestTool send CreateShortRequest ([979788e](https://github.com/Realsnack/snacks-awesome-shortener/commit/979788e807af4504675d5d720ce82ec511bde62e))
* Unified config crate ([#32](https://github.com/Realsnack/snacks-awesome-shortener/issues/32)) ([a525db1](https://github.com/Realsnack/snacks-awesome-shortener/commit/a525db14a9168799279521d981cf5e82ad4f9670))


### Bug Fixes

* Added flush() into test_tool ([985531a](https://github.com/Realsnack/snacks-awesome-shortener/commit/985531af093c04feabc5dad4702a1c5f1388d03d))
