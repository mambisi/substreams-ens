
const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
/// Contract's functions.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod functions {
    use super::INTERNAL_ERR;
    #[derive(Debug, Clone, PartialEq)]
    pub struct Abi {
        pub node: [u8; 32usize],
        pub content_types: substreams::scalar::BigInt,
    }
    impl Abi {
        const METHOD_ID: [u8; 4] = [34u8, 3u8, 171u8, 86u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Uint(256usize),
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                content_types: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    self.content_types.clone().to_signed_bytes_be().as_slice(),
                )),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<(substreams::scalar::BigInt, Vec<u8>), String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<(substreams::scalar::BigInt, Vec<u8>), String> {
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Uint(256usize), ethabi::ParamType::Bytes],
                data.as_ref(),
            )
            .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            values.reverse();
            Ok((
                {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
            ))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<(substreams::scalar::BigInt, Vec<u8>)> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Abi {
        const NAME: &'static str = "ABI";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<(substreams::scalar::BigInt, Vec<u8>)> for Abi {
        fn output(data: &[u8]) -> Result<(substreams::scalar::BigInt, Vec<u8>), String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Addr1 {
        pub node: [u8; 32usize],
    }
    impl Addr1 {
        const METHOD_ID: [u8; 4] = [59u8, 59u8, 87u8, 222u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::FixedBytes(32usize)],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::FixedBytes(self.node.as_ref().to_vec())]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<u8>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec())
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Addr1 {
        const NAME: &'static str = "addr1";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for Addr1 {
        fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Addr2 {
        pub node: [u8; 32usize],
        pub coin_type: substreams::scalar::BigInt,
    }
    impl Addr2 {
        const METHOD_ID: [u8; 4] = [241u8, 203u8, 126u8, 6u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Uint(256usize),
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                coin_type: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    self.coin_type.clone().to_signed_bytes_be().as_slice(),
                )),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<u8>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bytes], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_bytes()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Addr2 {
        const NAME: &'static str = "addr2";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for Addr2 {
        fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Authorisations {
        pub param0: [u8; 32usize],
        pub param1: Vec<u8>,
        pub param2: Vec<u8>,
    }
    impl Authorisations {
        const METHOD_ID: [u8; 4] = [248u8, 107u8, 200u8, 121u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                param0: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                param1: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                param2: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.param0.as_ref().to_vec()),
                ethabi::Token::Address(ethabi::Address::from_slice(&self.param1)),
                ethabi::Token::Address(ethabi::Address::from_slice(&self.param2)),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_bool()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Authorisations {
        const NAME: &'static str = "authorisations";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<bool> for Authorisations {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Contenthash {
        pub node: [u8; 32usize],
    }
    impl Contenthash {
        const METHOD_ID: [u8; 4] = [188u8, 28u8, 88u8, 209u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::FixedBytes(32usize)],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::FixedBytes(self.node.as_ref().to_vec())]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<u8>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bytes], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_bytes()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Contenthash {
        const NAME: &'static str = "contenthash";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for Contenthash {
        fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct InterfaceImplementer {
        pub node: [u8; 32usize],
        pub interface_id: [u8; 4usize],
    }
    impl InterfaceImplementer {
        const METHOD_ID: [u8; 4] = [18u8, 74u8, 49u8, 156u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::FixedBytes(4usize),
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                interface_id: {
                    let mut result = [0u8; 4];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::FixedBytes(self.interface_id.as_ref().to_vec()),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<u8>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_address()
                .expect(INTERNAL_ERR)
                .as_bytes()
                .to_vec())
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for InterfaceImplementer {
        const NAME: &'static str = "interfaceImplementer";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for InterfaceImplementer {
        fn output(data: &[u8]) -> Result<Vec<u8>, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Name {
        pub node: [u8; 32usize],
    }
    impl Name {
        const METHOD_ID: [u8; 4] = [105u8, 31u8, 52u8, 49u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::FixedBytes(32usize)],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::FixedBytes(self.node.as_ref().to_vec())]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<String, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<String, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::String], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_string()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<String> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Name {
        const NAME: &'static str = "name";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<String> for Name {
        fn output(data: &[u8]) -> Result<String, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Pubkey {
        pub node: [u8; 32usize],
    }
    impl Pubkey {
        const METHOD_ID: [u8; 4] = [200u8, 105u8, 2u8, 51u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::FixedBytes(32usize)],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::FixedBytes(self.node.as_ref().to_vec())]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<([u8; 32usize], [u8; 32usize]), String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<([u8; 32usize], [u8; 32usize]), String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::FixedBytes(32usize),
                ],
                data.as_ref(),
            )
            .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            values.reverse();
            Ok((
                {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            ))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<([u8; 32usize], [u8; 32usize])> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Pubkey {
        const NAME: &'static str = "pubkey";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<([u8; 32usize], [u8; 32usize])> for Pubkey {
        fn output(data: &[u8]) -> Result<([u8; 32usize], [u8; 32usize]), String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct SetAbi {
        pub node: [u8; 32usize],
        pub content_type: substreams::scalar::BigInt,
        pub data: Vec<u8>,
    }
    impl SetAbi {
        const METHOD_ID: [u8; 4] = [98u8, 49u8, 149u8, 176u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Bytes,
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                content_type: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                data: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    self.content_type.clone().to_signed_bytes_be().as_slice(),
                )),
                ethabi::Token::Bytes(self.data.clone()),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for SetAbi {
        const NAME: &'static str = "setABI";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct SetAddr1 {
        pub node: [u8; 32usize],
        pub coin_type: substreams::scalar::BigInt,
        pub a: Vec<u8>,
    }
    impl SetAddr1 {
        const METHOD_ID: [u8; 4] = [139u8, 149u8, 221u8, 113u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Bytes,
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                coin_type: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                a: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    self.coin_type.clone().to_signed_bytes_be().as_slice(),
                )),
                ethabi::Token::Bytes(self.a.clone()),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for SetAddr1 {
        const NAME: &'static str = "setAddr1";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct SetAddr2 {
        pub node: [u8; 32usize],
        pub a: Vec<u8>,
    }
    impl SetAddr2 {
        const METHOD_ID: [u8; 4] = [213u8, 250u8, 43u8, 0u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Address,
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                a: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::Address(ethabi::Address::from_slice(&self.a)),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for SetAddr2 {
        const NAME: &'static str = "setAddr2";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct SetAuthorisation {
        pub node: [u8; 32usize],
        pub target: Vec<u8>,
        pub is_authorised: bool,
    }
    impl SetAuthorisation {
        const METHOD_ID: [u8; 4] = [62u8, 156u8, 231u8, 148u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Bool,
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                target: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                is_authorised: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bool()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::Address(ethabi::Address::from_slice(&self.target)),
                ethabi::Token::Bool(self.is_authorised),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for SetAuthorisation {
        const NAME: &'static str = "setAuthorisation";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct SetContenthash {
        pub node: [u8; 32usize],
        pub hash: Vec<u8>,
    }
    impl SetContenthash {
        const METHOD_ID: [u8; 4] = [48u8, 78u8, 106u8, 222u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Bytes,
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                hash: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::Bytes(self.hash.clone()),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for SetContenthash {
        const NAME: &'static str = "setContenthash";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct SetInterface {
        pub node: [u8; 32usize],
        pub interface_id: [u8; 4usize],
        pub implementer: Vec<u8>,
    }
    impl SetInterface {
        const METHOD_ID: [u8; 4] = [229u8, 157u8, 137u8, 93u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::FixedBytes(4usize),
                    ethabi::ParamType::Address,
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                interface_id: {
                    let mut result = [0u8; 4];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                implementer: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::FixedBytes(self.interface_id.as_ref().to_vec()),
                ethabi::Token::Address(ethabi::Address::from_slice(&self.implementer)),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for SetInterface {
        const NAME: &'static str = "setInterface";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct SetName {
        pub node: [u8; 32usize],
        pub name: String,
    }
    impl SetName {
        const METHOD_ID: [u8; 4] = [119u8, 55u8, 34u8, 19u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::String,
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                name: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::String(self.name.clone()),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for SetName {
        const NAME: &'static str = "setName";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct SetPubkey {
        pub node: [u8; 32usize],
        pub x: [u8; 32usize],
        pub y: [u8; 32usize],
    }
    impl SetPubkey {
        const METHOD_ID: [u8; 4] = [41u8, 205u8, 98u8, 234u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::FixedBytes(32usize),
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                x: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                y: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::FixedBytes(self.x.as_ref().to_vec()),
                ethabi::Token::FixedBytes(self.y.as_ref().to_vec()),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for SetPubkey {
        const NAME: &'static str = "setPubkey";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct SetText {
        pub node: [u8; 32usize],
        pub key: String,
        pub value: String,
    }
    impl SetText {
        const METHOD_ID: [u8; 4] = [16u8, 241u8, 58u8, 140u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::String,
                    ethabi::ParamType::String,
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                key: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
                value: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::String(self.key.clone()),
                ethabi::Token::String(self.value.clone()),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for SetText {
        const NAME: &'static str = "setText";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct SupportsInterface {
        pub interface_id: [u8; 4usize],
    }
    impl SupportsInterface {
        const METHOD_ID: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::FixedBytes(4usize)],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                interface_id: {
                    let mut result = [0u8; 4];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::FixedBytes(
                self.interface_id.as_ref().to_vec(),
            )]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_bool()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for SupportsInterface {
        const NAME: &'static str = "supportsInterface";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<bool> for SupportsInterface {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Text {
        pub node: [u8; 32usize],
        pub key: String,
    }
    impl Text {
        const METHOD_ID: [u8; 4] = [89u8, 209u8, 212u8, 60u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::String,
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                key: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::FixedBytes(self.node.as_ref().to_vec()),
                ethabi::Token::String(self.key.clone()),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<String, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<String, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::String], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_string()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<String> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall {
                    to_addr: address,
                    data: self.encode(),
                }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses.get(0).expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Text {
        const NAME: &'static str = "text";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<String> for Text {
        fn output(data: &[u8]) -> Result<String, String> {
            Self::output(data)
        }
    }
}
/// Contract's events.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod events {
    use super::INTERNAL_ERR;
    #[derive(Debug, Clone, PartialEq)]
    pub struct AbiChanged {
        pub node: [u8; 32usize],
        pub content_type: substreams::scalar::BigInt,
    }
    impl AbiChanged {
        const TOPIC_ID: [u8; 32] = [
            170u8, 18u8, 27u8, 190u8, 239u8, 95u8, 50u8, 245u8, 150u8, 26u8, 42u8, 40u8, 150u8,
            110u8, 118u8, 144u8, 35u8, 145u8, 15u8, 201u8, 71u8, 144u8, 89u8, 238u8, 52u8, 149u8,
            212u8, 193u8, 166u8, 150u8, 239u8, 227u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 3usize {
                return false;
            }
            if log.data.len() != 0usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'node' from topic of type 'bytes32': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                content_type: {
                    let mut v = [0 as u8; 32];
                    ethabi::decode(
                                &[ethabi::ParamType::Uint(256usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'content_type' from topic of type 'uint256': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
    }
    impl substreams_ethereum::Event for AbiChanged {
        const NAME: &'static str = "ABIChanged";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct AddrChanged {
        pub node: [u8; 32usize],
        pub a: Vec<u8>,
    }
    impl AddrChanged {
        const TOPIC_ID: [u8; 32] = [
            82u8, 215u8, 216u8, 97u8, 240u8, 154u8, 179u8, 210u8, 98u8, 57u8, 212u8, 146u8, 232u8,
            150u8, 134u8, 41u8, 249u8, 94u8, 158u8, 49u8, 140u8, 240u8, 183u8, 59u8, 253u8, 220u8,
            68u8, 21u8, 34u8, 161u8, 95u8, 210u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 2usize {
                return false;
            }
            if log.data.len() != 32usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], log.data.as_ref())
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'node' from topic of type 'bytes32': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                a: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
    }
    impl substreams_ethereum::Event for AddrChanged {
        const NAME: &'static str = "AddrChanged";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct AddressChanged {
        pub node: [u8; 32usize],
        pub coin_type: substreams::scalar::BigInt,
        pub new_address: Vec<u8>,
    }
    impl AddressChanged {
        const TOPIC_ID: [u8; 32] = [
            101u8, 65u8, 37u8, 129u8, 22u8, 142u8, 136u8, 161u8, 230u8, 12u8, 100u8, 89u8, 215u8,
            244u8, 74u8, 232u8, 58u8, 208u8, 131u8, 46u8, 103u8, 8u8, 38u8, 192u8, 90u8, 78u8,
            36u8, 118u8, 181u8, 122u8, 247u8, 82u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 2usize {
                return false;
            }
            if log.data.len() < 96usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Uint(256usize), ethabi::ParamType::Bytes],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'node' from topic of type 'bytes32': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                coin_type: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                new_address: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
            })
        }
    }
    impl substreams_ethereum::Event for AddressChanged {
        const NAME: &'static str = "AddressChanged";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct AuthorisationChanged {
        pub node: [u8; 32usize],
        pub owner: Vec<u8>,
        pub target: Vec<u8>,
        pub is_authorised: bool,
    }
    impl AuthorisationChanged {
        const TOPIC_ID: [u8; 32] = [
            225u8, 197u8, 97u8, 10u8, 110u8, 12u8, 190u8, 16u8, 118u8, 78u8, 205u8, 24u8, 42u8,
            220u8, 239u8, 30u8, 195u8, 56u8, 220u8, 78u8, 25u8, 156u8, 153u8, 195u8, 44u8, 233u8,
            143u8, 56u8, 225u8, 39u8, 145u8, 223u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 4usize {
                return false;
            }
            if log.data.len() != 32usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], log.data.as_ref())
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'node' from topic of type 'bytes32': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                owner: ethabi::decode(&[ethabi::ParamType::Address], log.topics[2usize].as_ref())
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'owner' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                target: ethabi::decode(&[ethabi::ParamType::Address], log.topics[3usize].as_ref())
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'target' from topic of type 'address': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                is_authorised: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bool()
                    .expect(INTERNAL_ERR),
            })
        }
    }
    impl substreams_ethereum::Event for AuthorisationChanged {
        const NAME: &'static str = "AuthorisationChanged";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct ContenthashChanged {
        pub node: [u8; 32usize],
        pub hash: Vec<u8>,
    }
    impl ContenthashChanged {
        const TOPIC_ID: [u8; 32] = [
            227u8, 121u8, 193u8, 98u8, 78u8, 215u8, 231u8, 20u8, 204u8, 9u8, 55u8, 82u8, 138u8,
            50u8, 53u8, 157u8, 105u8, 213u8, 40u8, 19u8, 55u8, 118u8, 83u8, 19u8, 219u8, 164u8,
            224u8, 129u8, 183u8, 45u8, 117u8, 120u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 2usize {
                return false;
            }
            if log.data.len() < 64usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bytes], log.data.as_ref())
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'node' from topic of type 'bytes32': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                hash: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
            })
        }
    }
    impl substreams_ethereum::Event for ContenthashChanged {
        const NAME: &'static str = "ContenthashChanged";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct InterfaceChanged {
        pub node: [u8; 32usize],
        pub interface_id: [u8; 4usize],
        pub implementer: Vec<u8>,
    }
    impl InterfaceChanged {
        const TOPIC_ID: [u8; 32] = [
            124u8, 105u8, 240u8, 107u8, 234u8, 11u8, 222u8, 245u8, 101u8, 183u8, 9u8, 233u8, 58u8,
            20u8, 120u8, 54u8, 176u8, 6u8, 59u8, 162u8, 221u8, 137u8, 240u8, 45u8, 11u8, 126u8,
            141u8, 147u8, 30u8, 106u8, 109u8, 170u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 3usize {
                return false;
            }
            if log.data.len() != 32usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], log.data.as_ref())
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'node' from topic of type 'bytes32': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                interface_id: {
                    let mut result = [0u8; 4];
                    let v = ethabi::decode(
                                &[ethabi::ParamType::FixedBytes(4usize)],
                                log.topics[2usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'interface_id' from topic of type 'bytes4': {:?}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                implementer: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
    }
    impl substreams_ethereum::Event for InterfaceChanged {
        const NAME: &'static str = "InterfaceChanged";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct NameChanged {
        pub node: [u8; 32usize],
        pub name: String,
    }
    impl NameChanged {
        const TOPIC_ID: [u8; 32] = [
            183u8, 210u8, 158u8, 145u8, 16u8, 65u8, 232u8, 217u8, 184u8, 67u8, 54u8, 158u8, 137u8,
            11u8, 203u8, 114u8, 201u8, 56u8, 134u8, 146u8, 186u8, 72u8, 182u8, 90u8, 197u8, 78u8,
            114u8, 20u8, 196u8, 195u8, 72u8, 247u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 2usize {
                return false;
            }
            if log.data.len() < 64usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::String], log.data.as_ref())
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'node' from topic of type 'bytes32': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                name: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
            })
        }
    }
    impl substreams_ethereum::Event for NameChanged {
        const NAME: &'static str = "NameChanged";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct PubkeyChanged {
        pub node: [u8; 32usize],
        pub x: [u8; 32usize],
        pub y: [u8; 32usize],
    }
    impl PubkeyChanged {
        const TOPIC_ID: [u8; 32] = [
            29u8, 111u8, 94u8, 3u8, 211u8, 246u8, 62u8, 181u8, 135u8, 81u8, 152u8, 102u8, 41u8,
            165u8, 67u8, 155u8, 174u8, 229u8, 7u8, 159u8, 240u8, 79u8, 52u8, 91u8, 236u8, 182u8,
            110u8, 35u8, 235u8, 21u8, 78u8, 70u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 2usize {
                return false;
            }
            if log.data.len() != 64usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::FixedBytes(32usize),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'node' from topic of type 'bytes32': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                x: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                y: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
    }
    impl substreams_ethereum::Event for PubkeyChanged {
        const NAME: &'static str = "PubkeyChanged";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct TextChanged1 {
        pub node: [u8; 32usize],
        pub indexed_key: String,
        pub key: String,
    }
    impl TextChanged1 {
        const TOPIC_ID: [u8; 32] = [
            216u8, 201u8, 51u8, 75u8, 26u8, 156u8, 47u8, 157u8, 163u8, 66u8, 160u8, 162u8, 179u8,
            38u8, 41u8, 193u8, 162u8, 41u8, 182u8, 68u8, 93u8, 173u8, 120u8, 148u8, 127u8, 103u8,
            75u8, 68u8, 68u8, 74u8, 117u8, 80u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 3usize {
                return false;
            }
            if log.data.len() < 64usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::String], log.data.as_ref())
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'node' from topic of type 'bytes32': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                indexed_key: ethabi::decode(
                    &[ethabi::ParamType::String],
                    log.topics[2usize].as_ref(),
                )
                .map_err(|e| {
                    format!(
                        "unable to decode param 'indexed_key' from topic of type 'string': {:?}",
                        e
                    )
                })?
                .pop()
                .expect(INTERNAL_ERR)
                .into_string()
                .expect(INTERNAL_ERR),
                key: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
            })
        }
    }
    impl substreams_ethereum::Event for TextChanged1 {
        const NAME: &'static str = "TextChanged1";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct TextChanged2 {
        pub node: [u8; 32usize],
        pub indexed_key: String,
        pub key: String,
        pub value: String,
    }
    impl TextChanged2 {
        const TOPIC_ID: [u8; 32] = [
            68u8, 139u8, 192u8, 20u8, 241u8, 83u8, 103u8, 38u8, 207u8, 141u8, 84u8, 255u8, 61u8,
            100u8, 129u8, 237u8, 60u8, 188u8, 104u8, 60u8, 37u8, 145u8, 202u8, 32u8, 66u8, 116u8,
            0u8, 154u8, 250u8, 9u8, 177u8, 161u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 3usize {
                return false;
            }
            if log.data.len() < 128usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[ethabi::ParamType::String, ethabi::ParamType::String],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'node' from topic of type 'bytes32': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                indexed_key: ethabi::decode(
                    &[ethabi::ParamType::String],
                    log.topics[2usize].as_ref(),
                )
                .map_err(|e| {
                    format!(
                        "unable to decode param 'indexed_key' from topic of type 'string': {:?}",
                        e
                    )
                })?
                .pop()
                .expect(INTERNAL_ERR)
                .into_string()
                .expect(INTERNAL_ERR),
                key: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
                value: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_string()
                    .expect(INTERNAL_ERR),
            })
        }
    }
    impl substreams_ethereum::Event for TextChanged2 {
        const NAME: &'static str = "TextChanged2";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct VersionChanged {
        pub node: [u8; 32usize],
        pub new_version: substreams::scalar::BigInt,
    }
    impl VersionChanged {
        const TOPIC_ID: [u8; 32] = [
            198u8, 98u8, 28u8, 203u8, 143u8, 63u8, 90u8, 4u8, 187u8, 101u8, 2u8, 21u8, 75u8, 44u8,
            175u8, 106u8, 223u8, 89u8, 131u8, 254u8, 118u8, 223u8, 239u8, 28u8, 252u8, 156u8, 66u8,
            227u8, 87u8, 157u8, 180u8, 68u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 2usize {
                return false;
            }
            if log.data.len() != 32usize {
                return false;
            }
            return log.topics.get(0).expect("bounds already checked").as_ref() == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Uint(64usize)], log.data.as_ref())
                .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                node: {
                    let mut result = [0u8; 32];
                    let v = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.topics[1usize].as_ref(),
                    )
                    .map_err(|e| {
                        format!(
                            "unable to decode param 'node' from topic of type 'bytes32': {:?}",
                            e
                        )
                    })?
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_fixed_bytes()
                    .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                new_version: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
    }
    impl substreams_ethereum::Event for VersionChanged {
        const NAME: &'static str = "VersionChanged";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
}
