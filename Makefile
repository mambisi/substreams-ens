
ENDPOINT ?= mainnet.eth.streamingfast.io:443
GRAPH_CONFIG ?= ../graph-node-dev/config/graphman.toml

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_transfers -s 12292922 -t +10

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml

.PHONE: package
package: build
	substreams pack -o substreams.spkg substreams.yaml

.PHONE: deploy_local
deploy_local: package
	mkdir build 2> /dev/null || true
	graph build --ipfs http://localhost:5001 ens-subgraph/subgraph.yaml
	graph create ens --node http://127.0.0.1:8020
	graph deploy --node http://127.0.0.1:8020 --ipfs http://127.0.0.1:5001 --version-label v0.0.1 ens ens-subgraph/subgraph.yaml

.PHONE: undeploy_local
undeploy_local:
	graphman --config "$(GRAPH_CONFIG)" drop --force uniswap_v3