# Changelog

## [2.0.0](https://github.com/Realsnack/snacks-awesome-shortener/compare/api_gateway-v1.0.0...api_gateway-v2.0.0) (2026-06-05)


### ⚠ BREAKING CHANGES

* Rewrite messages into ProtoBuf from MessagePack

### Features

* **#22:** TypeString trait to get string of type name ([#26](https://github.com/Realsnack/snacks-awesome-shortener/issues/26)) ([47d47ef](https://github.com/Realsnack/snacks-awesome-shortener/commit/47d47efb49d1e586a1041c79502f7ccf94a21019))
* Added hostname to api_gateway root endpoint ([a3e485b](https://github.com/Realsnack/snacks-awesome-shortener/commit/a3e485b334a5d0fa7c016dd5f3514a4cb9b8d72d))
* Added instance id into response consumer on api_gateway ([58c2035](https://github.com/Realsnack/snacks-awesome-shortener/commit/58c20353411bd0597b4dafe6e3b594a9d78ec9ae))
* All messages converted to ProtoBuf ([495aad3](https://github.com/Realsnack/snacks-awesome-shortener/commit/495aad33fa1323a033aa201edbb8f5e302d50d46))
* Api gateway can create shorts ([ed57223](https://github.com/Realsnack/snacks-awesome-shortener/commit/ed57223b00fd4eebdf842bb0a104c202ce93d9d0))
* api_gateway PoC ([7e23fae](https://github.com/Realsnack/snacks-awesome-shortener/commit/7e23faeb4ef3ae50ad137a6f5ca073a09795e546))
* APIs to retrieve shorts ([#33](https://github.com/Realsnack/snacks-awesome-shortener/issues/33)) ([d10f6ad](https://github.com/Realsnack/snacks-awesome-shortener/commit/d10f6ade270f5a32d30c9499e7ba4f1b0f590115))
* building api_gateway routes ([91054c4](https://github.com/Realsnack/snacks-awesome-shortener/commit/91054c463b3fe31eb5afdcfe6a53bea98d9f652c))
* created api_gateway consumer task ([54f7947](https://github.com/Realsnack/snacks-awesome-shortener/commit/54f79470113ce4b3a066ba471425262e2e1460e5))
* Created basic api-gateway ([cdbd317](https://github.com/Realsnack/snacks-awesome-shortener/commit/cdbd31726a9a792437162b08f43c9be5ce7b72ab))
* Created proper rest request model ([4cf2a61](https://github.com/Realsnack/snacks-awesome-shortener/commit/4cf2a6195da8ddd2adb0a84980adde4538713877))
* refactor nats consumer for api_gateway and added AppState ([cf56793](https://github.com/Realsnack/snacks-awesome-shortener/commit/cf567933d540b452f0847ec5780255681f3bc728))
* Rewrite messages into ProtoBuf from MessagePack ([0696ea7](https://github.com/Realsnack/snacks-awesome-shortener/commit/0696ea7cbd9ee97b8104266a36b9c035238d24cf))
* Unified config crate ([#32](https://github.com/Realsnack/snacks-awesome-shortener/issues/32)) ([a525db1](https://github.com/Realsnack/snacks-awesome-shortener/commit/a525db14a9168799279521d981cf5e82ad4f9670))
* Use common logging in api_gateway ([4304032](https://github.com/Realsnack/snacks-awesome-shortener/commit/4304032a2db1477ab223e231964fad8a79a17c34))
* Use common logging in api_gateway ([3a16434](https://github.com/Realsnack/snacks-awesome-shortener/commit/3a16434996a1697266d87975601ed1980322d9ba))


### Bug Fixes

* failing build due to unused import ([330056a](https://github.com/Realsnack/snacks-awesome-shortener/commit/330056a11845950c3311f66442f417ce04e512a4))
* Fix subject for short retrieval ([49083f7](https://github.com/Realsnack/snacks-awesome-shortener/commit/49083f763f95e613842029bb36d91c0aeacecf17))
* Remove limitation on consumed messages ([453dce2](https://github.com/Realsnack/snacks-awesome-shortener/commit/453dce2a60702753c83b5ece419d57e7c9362ebe))
* return 404 response for non-existent short ([#37](https://github.com/Realsnack/snacks-awesome-shortener/issues/37)) ([a16d6cc](https://github.com/Realsnack/snacks-awesome-shortener/commit/a16d6cce55ba6a6d4b450a2c9165a55b275f5701))
