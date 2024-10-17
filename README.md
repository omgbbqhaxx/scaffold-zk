# 🏗 Scaffold-ZK
Welcome to the Scaffold-ZK.

🧪 An open-source, up-to-date toolkit for building decentralized zero knowledge applications (dapps) on the alignedlayer. It's designed to make it easier for developers to create and deploy ZKVM programs and build user cli-interfaces that interact with those programs.


## Contact

[![Gitter](https://img.shields.io/gitter/room/nwjs/nw.js.svg)](https://x.com/yasinaktimur/)
[![GitHub Issues](https://img.shields.io/badge/open%20issues-0-yellow.svg)](https://github.com/omgbbqhaxx/zkmine/issues)

ZKVM's | Status
---------------- | ----------
SP1 | [![TravisCI](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://travis-ci.org/cloudbank/cloudbank-github)
RISC0         | [![AppVeyor](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://ci.appveyor.com/project/cloudbank/cloudbank-github)


-  [Alignedlayer Twitter](https://x.com/alignedlayer)
-  [Alignedlayer Docs](https://docs.alignedlayer.com)

## Usagement


Add scaffoldzk.rs into your main.rs folder

You can easily get address, string, integer inputs using scaffold-zk.

Example, you can get and check easily eth address from user and check is it valid : 

```shell
use scaffoldzk::get_address_input;
pub mod scaffoldzk;
let mut eth_address = String::new();
let userinput = "Please enter your Ethereum address:";
eth_address.push_str(&get_address_input(userinput)); 
```





## Developer & Contributor commants.

```shell
sudo cargo run --release -- prove-sp1 examples/zkmine

sudo cargo run --release -- prove-sp1 examples/zkmine --submit-to-aligned --keystore-path ~/.foundry/keystores/key.json
```

When you create an ELF file.

Clone the aligned repo and run 

sudo make install_aligned_compiling

If the binary doesn’t work

```shell
aligned get-vk-commitment --verification_key_file <path_to_input_file> --proving_system SP1
```

## License

[![License](https://img.shields.io/github/license/ethereum/cpp-ethereum.svg)](LICENSE)

All contributions are made under the [GNU General Public License v3](https://www.gnu.org/licenses/gpl-3.0.en.html). See [LICENSE](LICENSE).