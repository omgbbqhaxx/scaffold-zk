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


## Get eth address as input & check validity.

```shell
use scaffoldzk::get_address;
pub mod scaffoldzk;
let mut eth_address = String::new();
let placeholder = "Please enter your Ethereum address:";
eth_address.push_str(&get_address_input(placeholder)); 
```


## Get string and check max length

```shell
use scaffoldzk::get_string;
pub mod scaffoldzk;
let mut getstring = String::new();
let max_length = 20;  
let placeholder = "Please enter your text:";
getstring.push_str(&get_string(placeholder, max_length)); 
println!("Your string is here:  {}", getstring);
```

## Get Integer and check max length

```shell
use scaffoldzk::get_integer;
pub mod scaffoldzk;
let max_length = 4;  
let placeholder = "Please enter your integer:";
let getint = get_integer(placeholder, max_length); 
println!("Your PIN or Number is: {}", getint); 
```


## Ask questions and get answers

```shell
use scaffoldzk::ask_question;
pub mod scaffoldzk;


let mut user_answers = "".to_string();
let question1 = "Who invented bitcoin";
let answers1 = ["Sreeram Kannan", "Vitalik Buterin", "Satoshi Nakamoto"];
user_answers.push(ask_question(question1, &answers1));

let question2 = "What is the largest ocean on Earth?";
let answers2 = ["Atlantic", "Indian", "Pacific"];
user_answers.push(ask_question(question2, &answers2));

let question3 = "What is the most aligned color";
let answers3 = ["Green", "Red", "Blue"];
user_answers.push(ask_question(question3, &answers3));
```


## Developer & Contributor commants.


```shell
sudo cargo run --release -- prove-sp1 examples/scaffold-zk

sudo cargo run --release -- prove-sp1 examples/scaffold-zk --submit-to-aligned --keystore-path ~/.foundry/keystores/key.json
```



Our aligned proof is here : https://explorer.alignedlayer.com/batches/0x5393328194acb68fed5284b2318ff886edff9da924da5b342caaf42be9c62478

Our smart contract is here : https://holesky.etherscan.io/tx/0xe774ddb285ddcddb066dced9b9c7b19031343c00813a5b835d1d60f7137fef59




When you create an ELF file.

Clone the aligned repo and run 

```shell
sudo make install_aligned_compiling
aligned get-vk-commitment --verification_key_file <path_to_input_file> --proving_system SP1
```
Our official commit.
```shell
[2024-10-18T01:18:59Z INFO  aligned] Commitment: fb51b2712031ca8b29657ecf09688eeb6c160be9ccf9b1304a97465015057f1e
```
## License

[![License](https://img.shields.io/github/license/ethereum/cpp-ethereum.svg)](LICENSE)

All contributions are made under the [GNU General Public License v3](https://www.gnu.org/licenses/gpl-3.0.en.html). See [LICENSE](LICENSE).