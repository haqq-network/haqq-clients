# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2](https://github.com/haqq-network/haqq-clients/compare/v0.2.1...v0.2.2) - 2024-02-19

### Other
- cargo update

## [0.2.1](https://github.com/haqq-network/haqq-clients/compare/v0.2.0...v0.2.1) - 2024-02-19

### Added
- better with_height
- get_head and with_head
- req_with_height
- grpc cargo feature for clients
- don't depend on protoc binary in $PATH
- Denom helpers
- get_denom returns amount
- typescript client
- to_u256 for String and &str
- helpers
- CoinExt and get_denom
- cargo features to control proto deps
- haqq, cosmos, evmos: proto, grpc and rest

### Other
- fix release-pls?
- remove haqq-build override
- don't process haqq-build
- release
- upgrade haqq-node
- release
- cargo test
- remove codecov due to https://github.com/xd009642/tarpaulin/issues/517
- codecov
- badges fix
- fix badge link
- badges
- git-release only for haqq-grpc
- release
- only update changelog for haqq-grpc
- fix config file 🤦
- Merge pull request [#5](https://github.com/haqq-network/haqq-clients/pull/5) from haqq-network/dependabot/github_actions/actions/checkout-4
- remove release-plz from devenv
- fix test
- only publish haqq-grpc
- cargo metadata
- change crate name
- fix branch
- use release-plz
- release set crate-name
- release action
- clippy warnings in test
- fix clippy warnings
- install devenv via action
- pair to conversion_pair
- don't reimport tonic
- Revert "ci: magic nix cache"
- magic nix cache
- remove obsolete cosmos-sdk submodule
- don't fail if tmp dir doesn't exists
- use haqq-network/nix-action
- adopt to new haqq-node protos
- remove REST endpoints, openapi-generator-cli can't handle IBC swagger
- remove cosmos nft module as it's not used by haqq network
- state that ts lib is for grpc-web
- fix markdown bold
- rename repo
- REST is abbrev
- bold
- fix todo format
- readme
- prelude for helpers
- Default pattern
- reimport tonic
- Channel alias
- cachix repo
- remove cachix
- use cachix
- update nixpkgs and some cleanup
- move haqq-rest to dev-dependencies
- devenv profile install accept flake config
- cachix accept flake config from magic-nix-cache
- add protobuf dep
- don't build server
- checkout submodules without pat
- checkout submodules
- fix build on linux
- fix ci script
- fix build on linux
- checks

## [0.2.0](https://github.com/haqq-network/haqq-clients/compare/haqq-grpc-v0.1.2...haqq-grpc-v0.2.0) - 2024-02-18

### Other
- upgrade haqq-node

## [0.1.2](https://github.com/haqq-network/haqq-clients/compare/haqq-grpc-v0.1.1...haqq-grpc-v0.1.2) - 2023-11-24

### Added
- better with_height
- get_head and with_head
- req_with_height
- grpc cargo feature for clients

### Other
- cargo test
- remove codecov due to https://github.com/xd009642/tarpaulin/issues/517
- codecov
- badges fix
- fix badge link
- badges
- git-release only for haqq-grpc

## [0.1.1](https://github.com/haqq-network/haqq-clients/compare/haqq-grpc-v0.1.0...haqq-grpc-v0.1.1) - 2023-11-23

### Other
- only update changelog for haqq-grpc
- fix config file 🤦
- Merge pull request [#5](https://github.com/haqq-network/haqq-clients/pull/5) from haqq-network/dependabot/github_actions/actions/checkout-4
- remove release-plz from devenv
- fix test
- only publish haqq-grpc
