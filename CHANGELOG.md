## [0.4.2](https://github.com/meskill/mystic-light-sdk/compare/v0.4.1...v0.4.2) (2022-10-01)


### Continuous Integration

* little simplificatin for clippy calls ([d9b3dc1](https://github.com/meskill/mystic-light-sdk/commit/d9b3dc18ccea458c26f60bdfe1c0d03c4d166e8c))


### Documentation

* contribution docs fixes ([6c966fb](https://github.com/meskill/mystic-light-sdk/commit/6c966fb83d6b2d46872160dd9830f056ad2edcd2))
* fix description of proper usage of build script ([0db21cf](https://github.com/meskill/mystic-light-sdk/commit/0db21cf9824b7239d031ee8e327ca532bf4f7676))

## [0.4.1](https://github.com/meskill/mystic-light-sdk/compare/v0.4.0...v0.4.1) (2022-08-20)


### Features

* migrate to tracing library for logging ([fd3051c](https://github.com/meskill/mystic-light-sdk/commit/fd3051cbdd3875a4acbbfe27e6e40df918c3635b))

## [0.4.0](https://github.com/meskill/mystic-light-sdk/compare/v0.3.0...v0.4.0) (2022-08-06)


### Features

* methods to update device led state with partial values ([4f37a44](https://github.com/meskill/mystic-light-sdk/commit/4f37a446ac67cbf8a6675b39ae8ce20cc1849e63))


### Code Refactoring

* rethinking the way to work with async-graphql library ([dfea16e](https://github.com/meskill/mystic-light-sdk/commit/dfea16eae6b3adab2679878a14164a90e3087aaa))

## [0.3.0](https://github.com/meskill/mystic-light-sdk/compare/v0.2.4...v0.3.0) (2022-07-03)


### ⚠ BREAKING CHANGES

* some functions are now private

### Features

* add logging ([fe92118](https://github.com/meskill/mystic-light-sdk/commit/fe9211806077613fcd9dd941b826176014b2061b))
* async_graphql support ([fea4613](https://github.com/meskill/mystic-light-sdk/commit/fea4613c84eef33ef06a278c6372791d046619c8))
* feat: support for multithreading ([0b8123c](https://github.com/meskill/mystic-light-sdk/commit/0b8123cb2b18fe66cefda97ee794573ed61a9225))


### Chores

* devcontainer config cleanup ([d4db6f1](https://github.com/meskill/mystic-light-sdk/commit/d4db6f170bf90e7dd6369418478ccdd34ab70e85))


### Code Refactoring

* hide factory functions with internal options from public access ([02ebc48](https://github.com/meskill/mystic-light-sdk/commit/02ebc48f812d18d02bc4d88ba7bfd6ced68efdcc))
* mark errors as non_exhaustive ([5064098](https://github.com/meskill/mystic-light-sdk/commit/5064098556072e7fd1ade2c4603968c0855f8fd5))


### Continuous Integration

* test for different features groups ([dadfa9e](https://github.com/meskill/mystic-light-sdk/commit/dadfa9ed056b325116ff6e4f4320306fbdf4a6bc))

### [0.2.4](https://github.com/meskill/mystic-light-sdk/compare/v0.2.3...v0.2.4) (2022-05-26)


### Documentation

* autogenerate README.md ([42f8407](https://github.com/meskill/mystic-light-sdk/commit/42f8407620324209a60248683d403a9d3401ebff))

### [0.2.3](https://github.com/meskill/mystic-light-sdk/compare/v0.2.2...v0.2.3) (2022-05-25)


### Bug Fixes

* build script change detection for path ([906eada](https://github.com/meskill/mystic-light-sdk/commit/906eadab58fb4ef8a6b4a7f8e492a6c3cc845ef8))

### [0.2.2](https://github.com/meskill/mystic-light-sdk/compare/v0.2.1...v0.2.2) (2022-05-23)


### Documentation

* fix README.md ([36870f8](https://github.com/meskill/mystic-light-sdk/commit/36870f89e57f2cae329e0fbc7bebfe3c38858b7a))

### [0.2.1](https://github.com/meskill/mystic-light-sdk/compare/v0.2.0...v0.2.1) (2022-05-22)


### Documentation

* inline example to lib docs ([862c04d](https://github.com/meskill/mystic-light-sdk/commit/862c04d45dbe3632b2708203c62797c350ca949c))


### Build System

* add env usage to build script ([0137cfb](https://github.com/meskill/mystic-light-sdk/commit/0137cfb21665bc929987ca58b403532fdd5e0e89))


### Continuous Integration

* fix publish ([dc2d883](https://github.com/meskill/mystic-light-sdk/commit/dc2d883ca2f5ec965aa9ad4a92b3f17fd8b8d110))
* fixup changes to lock file after publish ([4b03592](https://github.com/meskill/mystic-light-sdk/commit/4b035924d0a434d2759d5c4fbd44fec8cb88a4e7))
* run tests with all features ([ea9bc64](https://github.com/meskill/mystic-light-sdk/commit/ea9bc64baf946c71afa63bb1295d283e7772253b))

## [0.2.0](https://github.com/meskill/mystic-light-sdk/compare/v0.1.5...v0.2.0) (2022-05-22)


### Features

* add getters for some field for device and led ([4a30763](https://github.com/meskill/mystic-light-sdk/commit/4a3076312bf8716b7bf5a781585b091f9958aaa4))
* add serde feature implementation to able to serialize/deserialize ([bdb7f09](https://github.com/meskill/mystic-light-sdk/commit/bdb7f091e5110177df3d8af8c6b99af67023e67e))


### Continuous Integration

* use locked lock file ([6cd535d](https://github.com/meskill/mystic-light-sdk/commit/6cd535d4bff1079c9d441805c1129fc6aa11db69))


### Code Refactoring

* replace CommoError generation with custom_error! macro ([b5c6762](https://github.com/meskill/mystic-light-sdk/commit/b5c6762507dd0163303d9b8ff7dd3f92ce223eed))


### Build System

* fix build for docs.rs ([b7073d2](https://github.com/meskill/mystic-light-sdk/commit/b7073d2906e5c21a8d21b0cd5689e91756f19427))
* update wsl+docker execution scripts ([ccb1c15](https://github.com/meskill/mystic-light-sdk/commit/ccb1c151fb1e8f29deaba5dd2b8d8a68c86f29ae))

### [0.1.5](https://github.com/meskill/mystic-light-sdk/compare/v0.1.4...v0.1.5) (2022-04-26)


### Documentation

* fix Cargo.toml ([f7d693e](https://github.com/meskill/mystic-light-sdk/commit/f7d693e2e1e0fa73113afca55a2536315fc44534))

### [0.1.4](https://github.com/meskill/mystic-light-sdk/compare/v0.1.3...v0.1.4) (2022-04-26)


### Documentation

* fix license ([71eb2b1](https://github.com/meskill/mystic-light-sdk/commit/71eb2b1885f46ef8b460cf7f4cceff1f4fa324a5))

### [0.1.3](https://github.com/meskill/mystic-light-sdk/compare/v0.1.2...v0.1.3) (2022-04-26)


### Bug Fixes

* publish ([9118995](https://github.com/meskill/mystic-light-sdk/commit/911899517fb16c96888bdb0f8e7e37f8911d6d68))

### [0.1.2](https://github.com/meskill/mystic-light-sdk/compare/v0.1.1...v0.1.2) (2022-04-26)


### Documentation

* update Cargo.toml ([a1f1344](https://github.com/meskill/mystic-light-sdk/commit/a1f13448d4eb71b48e226c0f61d9b67b33de507a))


### Continuous Integration

* fix publish ([43e64c9](https://github.com/meskill/mystic-light-sdk/commit/43e64c938e4cad1abfd9078919351f50323103b0))
* github workflow cleanup ([5e9d8f4](https://github.com/meskill/mystic-light-sdk/commit/5e9d8f4fa97d05ebe0a32a3b20c09a6335bae40d))
* run publish only on release ([7de2d94](https://github.com/meskill/mystic-light-sdk/commit/7de2d94f4878777f7bc3b5352a3fd7890736a408))

### [0.1.1](https://github.com/meskill/mystic-light-sdk/compare/v0.1.0...v0.1.1) (2022-04-26)


### Documentation

* fix license field ([2c57a1b](https://github.com/meskill/mystic-light-sdk/commit/2c57a1b5dfc5b0233fdf5967e024bf19544c4399))


### Continuous Integration

* add github workflow ([2976f65](https://github.com/meskill/mystic-light-sdk/commit/2976f65ff773b7db140c61d1f199d6bbc6060b1f))
