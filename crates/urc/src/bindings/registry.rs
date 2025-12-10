///Module containing a contract's types and functions.
/**

```solidity
library BLS {
	struct G1Point { bytes32 x_a; bytes32 x_b; bytes32 y_a; bytes32 y_b; }
	struct G2Point { bytes32 x_c0_a; bytes32 x_c0_b; bytes32 x_c1_a; bytes32 x_c1_b; bytes32 y_c0_a; bytes32 y_c0_b; bytes32 y_c1_a; bytes32 y_c1_b; }
}
```*/
#[allow(
	non_camel_case_types,
	non_snake_case,
	clippy::pub_underscore_fields,
	clippy::style,
	clippy::empty_structs_with_brackets
)]
pub mod BLS {
	use super::*;
	use alloy::sol_types as alloy_sol_types;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct G1Point { bytes32 x_a; bytes32 x_b; bytes32 y_a; bytes32 y_b; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct G1Point {
		#[allow(missing_docs)]
		pub x_a: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub x_b: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub y_a: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub y_b: alloy::sol_types::private::FixedBytes<32>,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
			fn from(value: G1Point) -> Self {
				(value.x_a, value.x_b, value.y_a, value.y_b)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { x_a: tuple.0, x_b: tuple.1, y_a: tuple.2, y_b: tuple.3 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for G1Point {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_b),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_b),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for G1Point {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
				<Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolStruct for G1Point {
			const NAME: &'static str = "G1Point";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed("G1Point(bytes32 x_a,bytes32 x_b,bytes32 y_a,bytes32 y_b)")
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				alloy_sol_types::private::Vec::new()
			}
			#[inline]
			fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
				<Self as alloy_sol_types::SolStruct>::eip712_root_type()
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.x_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.x_b,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_b,
					)
					.0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for G1Point {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.x_a,
					) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.x_b,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_a,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_b,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_a, out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_b, out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_a, out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_b, out,
				);
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				let mut out = alloy_sol_types::private::Vec::new();
				<Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
				alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct G2Point { bytes32 x_c0_a; bytes32 x_c0_b; bytes32 x_c1_a; bytes32 x_c1_b; bytes32 y_c0_a; bytes32 y_c0_b; bytes32 y_c1_a; bytes32 y_c1_b; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct G2Point {
		#[allow(missing_docs)]
		pub x_c0_a: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub x_c0_b: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub x_c1_a: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub x_c1_b: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub y_c0_a: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub y_c0_b: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub y_c1_a: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub y_c1_b: alloy::sol_types::private::FixedBytes<32>,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<G2Point> for UnderlyingRustTuple<'_> {
			fn from(value: G2Point) -> Self {
				(
					value.x_c0_a,
					value.x_c0_b,
					value.x_c1_a,
					value.x_c1_b,
					value.y_c0_a,
					value.y_c0_b,
					value.y_c1_a,
					value.y_c1_b,
				)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for G2Point {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self {
					x_c0_a: tuple.0,
					x_c0_b: tuple.1,
					x_c1_a: tuple.2,
					x_c1_b: tuple.3,
					y_c0_a: tuple.4,
					y_c0_b: tuple.5,
					y_c1_a: tuple.6,
					y_c1_b: tuple.7,
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for G2Point {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for G2Point {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_c0_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_c0_b),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_c1_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.x_c1_b),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_c0_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_c0_b),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_c1_a),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.y_c1_b),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for G2Point {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
				<Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolStruct for G2Point {
			const NAME: &'static str = "G2Point";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"G2Point(bytes32 x_c0_a,bytes32 x_c0_b,bytes32 x_c1_a,bytes32 x_c1_b,bytes32 y_c0_a,bytes32 y_c0_b,bytes32 y_c1_a,bytes32 y_c1_b)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				alloy_sol_types::private::Vec::new()
			}
			#[inline]
			fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
				<Self as alloy_sol_types::SolStruct>::eip712_root_type()
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.x_c0_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.x_c0_b,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.x_c1_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.x_c1_b,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_c0_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_c0_b,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_c1_a,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.y_c1_b,
					)
					.0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for G2Point {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.x_c0_a,
					) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.x_c0_b,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.x_c1_a,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.x_c1_b,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_c0_a,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_c0_b,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_c1_a,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.y_c1_b,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_c0_a,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_c0_b,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_c1_a,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.x_c1_b,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_c0_a,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_c0_b,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_c1_a,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.y_c1_b,
					out,
				);
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				let mut out = alloy_sol_types::private::Vec::new();
				<Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
				alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
			}
		}
	};
	use alloy::contract as alloy_contract;
	/**Creates a new wrapper around an on-chain [`BLS`](self) contract instance.

	See the [wrapper's documentation](`BLSInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		address: alloy_sol_types::private::Address,
		__provider: P,
	) -> BLSInstance<P, N> {
		BLSInstance::<P, N>::new(address, __provider)
	}
	/**A [`BLS`](self) instance.

	Contains type-safe methods for interacting with an on-chain instance of the
	[`BLS`](self) contract located at a given `address`, using a given
	provider `P`.

	If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
	documentation on how to provide it), the `deploy` and `deploy_builder` methods can
	be used to deploy a new instance of the contract.

	See the [module-level documentation](self) for all the available methods.*/
	#[derive(Clone)]
	pub struct BLSInstance<P, N = alloy_contract::private::Ethereum> {
		address: alloy_sol_types::private::Address,
		provider: P,
		_network: ::core::marker::PhantomData<N>,
	}
	#[automatically_derived]
	impl<P, N> ::core::fmt::Debug for BLSInstance<P, N> {
		#[inline]
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			f.debug_tuple("BLSInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> BLSInstance<P, N> {
		/**Creates a new wrapper around an on-chain [`BLS`](self) contract instance.

		See the [wrapper's documentation](`BLSInstance`) for more details.*/
		#[inline]
		pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
			Self { address, provider: __provider, _network: ::core::marker::PhantomData }
		}
		/// Returns a reference to the address.
		#[inline]
		pub const fn address(&self) -> &alloy_sol_types::private::Address {
			&self.address
		}
		/// Sets the address.
		#[inline]
		pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
			self.address = address;
		}
		/// Sets the address and returns `self`.
		pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
			self.set_address(address);
			self
		}
		/// Returns a reference to the provider.
		#[inline]
		pub const fn provider(&self) -> &P {
			&self.provider
		}
	}
	impl<P: ::core::clone::Clone, N> BLSInstance<&P, N> {
		/// Clones the provider and returns a new instance with the cloned provider.
		#[inline]
		pub fn with_cloned_provider(self) -> BLSInstance<P, N> {
			BLSInstance {
				address: self.address,
				provider: ::core::clone::Clone::clone(&self.provider),
				_network: ::core::marker::PhantomData,
			}
		}
	}
	/// Function calls.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> BLSInstance<P, N> {
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> BLSInstance<P, N> {
		/// Creates a new event filter using this contract instance's provider and address.
		///
		/// Note that the type can be any event, not just those defined in this contract.
		/// Prefer using the other methods for building type-safe event filters.
		pub fn event_filter<E: alloy_sol_types::SolEvent>(&self) -> alloy_contract::Event<&P, E, N> {
			alloy_contract::Event::new_sol(&self.provider, &self.address)
		}
	}
}
///Module containing a contract's types and functions.
/**

```solidity
library IRegistry {
	type SlashingType is uint8;
	struct Config { uint80 minCollateralWei; uint32 fraudProofWindow; uint32 unregistrationDelay; uint32 slashWindow; uint32 optInDelay; bytes32 signingDomain; bytes32 chainId; }
	struct OperatorData { address owner; uint80 collateralWei; uint16 numKeys; uint48 registeredAt; uint48 unregisteredAt; uint48 slashedAt; bool deleted; bool equivocated; }
	struct RegistrationProof { bytes32 registrationRoot; SignedRegistration registration; bytes32[] merkleProof; bytes32 signingId; }
	struct SignedRegistration { BLS.G1Point pubkey; BLS.G2Point signature; uint64 nonce; }
	struct SlasherCommitment { address committer; uint48 optedInAt; uint48 optedOutAt; bool slashed; }
}
```*/
#[allow(
	non_camel_case_types,
	non_snake_case,
	clippy::pub_underscore_fields,
	clippy::style,
	clippy::empty_structs_with_brackets
)]
pub mod IRegistry {
	use super::*;
	use alloy::sol_types as alloy_sol_types;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SlashingType(u8);
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<SlashingType> for u8 {
			#[inline]
			fn stv_to_tokens(&self) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_> {
				alloy_sol_types::private::SolTypeValue::<alloy::sol_types::sol_data::Uint<8>>::stv_to_tokens(self)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				<alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				<alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(self)
			}
		}
		impl SlashingType {
			/// The Solidity type name.
			pub const NAME: &'static str = stringify!(@ name);
			/// Convert from the underlying value type.
			#[inline]
			pub const fn from_underlying(value: u8) -> Self {
				Self(value)
			}
			/// Return the underlying value.
			#[inline]
			pub const fn into_underlying(self) -> u8 {
				self.0
			}
			/// Return the single encoding of this value, delegating to the
			/// underlying type.
			#[inline]
			pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
				<Self as alloy_sol_types::SolType>::abi_encode(&self.0)
			}
			/// Return the packed encoding of this value, delegating to the
			/// underlying type.
			#[inline]
			pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
				<Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
			}
		}
		#[automatically_derived]
		impl From<u8> for SlashingType {
			fn from(value: u8) -> Self {
				Self::from_underlying(value)
			}
		}
		#[automatically_derived]
		impl From<SlashingType> for u8 {
			fn from(value: SlashingType) -> Self {
				value.into_underlying()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for SlashingType {
			type RustType = u8;
			type Token<'a> = <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = Self::NAME;
			const ENCODED_SIZE: Option<usize> =
				<alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				Self::type_check(token).is_ok()
			}
			#[inline]
			fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
				<alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				<alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for SlashingType {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				<alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				<alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				<alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(rust)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct Config { uint80 minCollateralWei; uint32 fraudProofWindow; uint32 unregistrationDelay; uint32 slashWindow; uint32 optInDelay; bytes32 signingDomain; bytes32 chainId; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct Config {
		#[allow(missing_docs)]
		pub minCollateralWei: alloy::sol_types::private::primitives::aliases::U80,
		#[allow(missing_docs)]
		pub fraudProofWindow: u32,
		#[allow(missing_docs)]
		pub unregistrationDelay: u32,
		#[allow(missing_docs)]
		pub slashWindow: u32,
		#[allow(missing_docs)]
		pub optInDelay: u32,
		#[allow(missing_docs)]
		pub signingDomain: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub chainId: alloy::sol_types::private::FixedBytes<32>,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			alloy::sol_types::sol_data::Uint<80>,
			alloy::sol_types::sol_data::Uint<32>,
			alloy::sol_types::sol_data::Uint<32>,
			alloy::sol_types::sol_data::Uint<32>,
			alloy::sol_types::sol_data::Uint<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::FixedBytes<32>,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			alloy::sol_types::private::primitives::aliases::U80,
			u32,
			u32,
			u32,
			u32,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::FixedBytes<32>,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<Config> for UnderlyingRustTuple<'_> {
			fn from(value: Config) -> Self {
				(
					value.minCollateralWei,
					value.fraudProofWindow,
					value.unregistrationDelay,
					value.slashWindow,
					value.optInDelay,
					value.signingDomain,
					value.chainId,
				)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for Config {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self {
					minCollateralWei: tuple.0,
					fraudProofWindow: tuple.1,
					unregistrationDelay: tuple.2,
					slashWindow: tuple.3,
					optInDelay: tuple.4,
					signingDomain: tuple.5,
					chainId: tuple.6,
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for Config {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for Config {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<alloy::sol_types::sol_data::Uint<80> as alloy_sol_types::SolType>::tokenize(
						&self.minCollateralWei,
					),
					<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
						&self.fraudProofWindow,
					),
					<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
						&self.unregistrationDelay,
					),
					<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(&self.slashWindow),
					<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(&self.optInDelay),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.signingDomain,
					),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.chainId),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for Config {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
				<Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolStruct for Config {
			const NAME: &'static str = "Config";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"Config(uint80 minCollateralWei,uint32 fraudProofWindow,uint32 unregistrationDelay,uint32 slashWindow,uint32 optInDelay,bytes32 signingDomain,bytes32 chainId)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				alloy_sol_types::private::Vec::new()
			}
			#[inline]
			fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
				<Self as alloy_sol_types::SolStruct>::eip712_root_type()
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<alloy::sol_types::sol_data::Uint<80> as alloy_sol_types::SolType>::eip712_data_word(
						&self.minCollateralWei,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.fraudProofWindow,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.unregistrationDelay,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.slashWindow,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.optInDelay,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.signingDomain,
					)
					.0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.chainId,
					)
					.0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for Config {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <alloy::sol_types::sol_data::Uint<80> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.minCollateralWei,
					) + <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.fraudProofWindow,
				) + <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.unregistrationDelay,
				) + <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.slashWindow,
				) + <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.optInDelay,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.signingDomain,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.chainId,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::Uint<80> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.minCollateralWei,
					out,
				);
				<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.fraudProofWindow,
					out,
				);
				<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.unregistrationDelay,
					out,
				);
				<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.slashWindow,
					out,
				);
				<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.optInDelay,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.signingDomain,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.chainId,
					out,
				);
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				let mut out = alloy_sol_types::private::Vec::new();
				<Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
				alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct OperatorData { address owner; uint80 collateralWei; uint16 numKeys; uint48 registeredAt; uint48 unregisteredAt; uint48 slashedAt; bool deleted; bool equivocated; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct OperatorData {
		#[allow(missing_docs)]
		pub owner: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub collateralWei: alloy::sol_types::private::primitives::aliases::U80,
		#[allow(missing_docs)]
		pub numKeys: u16,
		#[allow(missing_docs)]
		pub registeredAt: alloy::sol_types::private::primitives::aliases::U48,
		#[allow(missing_docs)]
		pub unregisteredAt: alloy::sol_types::private::primitives::aliases::U48,
		#[allow(missing_docs)]
		pub slashedAt: alloy::sol_types::private::primitives::aliases::U48,
		#[allow(missing_docs)]
		pub deleted: bool,
		#[allow(missing_docs)]
		pub equivocated: bool,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			alloy::sol_types::sol_data::Address,
			alloy::sol_types::sol_data::Uint<80>,
			alloy::sol_types::sol_data::Uint<16>,
			alloy::sol_types::sol_data::Uint<48>,
			alloy::sol_types::sol_data::Uint<48>,
			alloy::sol_types::sol_data::Uint<48>,
			alloy::sol_types::sol_data::Bool,
			alloy::sol_types::sol_data::Bool,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			alloy::sol_types::private::Address,
			alloy::sol_types::private::primitives::aliases::U80,
			u16,
			alloy::sol_types::private::primitives::aliases::U48,
			alloy::sol_types::private::primitives::aliases::U48,
			alloy::sol_types::private::primitives::aliases::U48,
			bool,
			bool,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<OperatorData> for UnderlyingRustTuple<'_> {
			fn from(value: OperatorData) -> Self {
				(
					value.owner,
					value.collateralWei,
					value.numKeys,
					value.registeredAt,
					value.unregisteredAt,
					value.slashedAt,
					value.deleted,
					value.equivocated,
				)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorData {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self {
					owner: tuple.0,
					collateralWei: tuple.1,
					numKeys: tuple.2,
					registeredAt: tuple.3,
					unregisteredAt: tuple.4,
					slashedAt: tuple.5,
					deleted: tuple.6,
					equivocated: tuple.7,
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for OperatorData {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for OperatorData {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.owner),
					<alloy::sol_types::sol_data::Uint<80> as alloy_sol_types::SolType>::tokenize(&self.collateralWei),
					<alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(&self.numKeys),
					<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::SolType>::tokenize(&self.registeredAt),
					<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::SolType>::tokenize(&self.unregisteredAt),
					<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::SolType>::tokenize(&self.slashedAt),
					<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(&self.deleted),
					<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(&self.equivocated),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for OperatorData {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
				<Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolStruct for OperatorData {
			const NAME: &'static str = "OperatorData";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"OperatorData(address owner,uint80 collateralWei,uint16 numKeys,uint48 registeredAt,uint48 unregisteredAt,uint48 slashedAt,bool deleted,bool equivocated)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				alloy_sol_types::private::Vec::new()
			}
			#[inline]
			fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
				<Self as alloy_sol_types::SolStruct>::eip712_root_type()
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(&self.owner).0,
					<alloy::sol_types::sol_data::Uint<80> as alloy_sol_types::SolType>::eip712_data_word(
						&self.collateralWei,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::eip712_data_word(&self.numKeys)
						.0,
					<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::SolType>::eip712_data_word(
						&self.registeredAt,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::SolType>::eip712_data_word(
						&self.unregisteredAt,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::SolType>::eip712_data_word(
						&self.slashedAt,
					)
					.0,
					<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(&self.deleted).0,
					<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(&self.equivocated)
						.0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for OperatorData {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.owner,
					) + <alloy::sol_types::sol_data::Uint<80> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.collateralWei,
				) + <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.numKeys,
				) + <alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.registeredAt,
				) + <alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.unregisteredAt,
				) + <alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.slashedAt,
				) + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.deleted,
				) + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.equivocated,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.owner,
					out,
				);
				<alloy::sol_types::sol_data::Uint<80> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.collateralWei,
					out,
				);
				<alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.numKeys,
					out,
				);
				<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.registeredAt,
					out,
				);
				<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.unregisteredAt,
					out,
				);
				<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.slashedAt,
					out,
				);
				<alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.deleted,
					out,
				);
				<alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.equivocated,
					out,
				);
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				let mut out = alloy_sol_types::private::Vec::new();
				<Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
				alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct RegistrationProof { bytes32 registrationRoot; SignedRegistration registration; bytes32[] merkleProof; bytes32 signingId; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct RegistrationProof {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub registration: <SignedRegistration as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub merkleProof: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
		#[allow(missing_docs)]
		pub signingId: alloy::sol_types::private::FixedBytes<32>,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			alloy::sol_types::sol_data::FixedBytes<32>,
			SignedRegistration,
			alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
			alloy::sol_types::sol_data::FixedBytes<32>,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			alloy::sol_types::private::FixedBytes<32>,
			<SignedRegistration as alloy::sol_types::SolType>::RustType,
			alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
			alloy::sol_types::private::FixedBytes<32>,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<RegistrationProof> for UnderlyingRustTuple<'_> {
			fn from(value: RegistrationProof) -> Self {
				(value.registrationRoot, value.registration, value.merkleProof, value.signingId)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for RegistrationProof {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { registrationRoot: tuple.0, registration: tuple.1, merkleProof: tuple.2, signingId: tuple.3 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for RegistrationProof {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for RegistrationProof {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.registrationRoot),
                    <SignedRegistration as alloy_sol_types::SolType>::tokenize(
                        &self.registration,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.merkleProof),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.signingId),
                )
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for RegistrationProof {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
				<Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolStruct for RegistrationProof {
			const NAME: &'static str = "RegistrationProof";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"RegistrationProof(bytes32 registrationRoot,SignedRegistration registration,bytes32[] merkleProof,bytes32 signingId)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				let mut components = alloy_sol_types::private::Vec::with_capacity(1);
				components.push(<SignedRegistration as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<SignedRegistration as alloy_sol_types::SolStruct>::eip712_components());
				components
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.registrationRoot,
                        )
                        .0,
                    <SignedRegistration as alloy_sol_types::SolType>::eip712_data_word(
                            &self.registration,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.merkleProof)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.signingId)
                        .0,
                ]
                    .concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for RegistrationProof {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.registrationRoot,
                    )
                    + <SignedRegistration as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.registration,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.merkleProof,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signingId,
                    )
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.registrationRoot,
					out,
				);
				<SignedRegistration as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.registration, out);
				<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.merkleProof,
                    out,
                );
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.signingId,
					out,
				);
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				let mut out = alloy_sol_types::private::Vec::new();
				<Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
				alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct SignedRegistration { BLS.G1Point pubkey; BLS.G2Point signature; uint64 nonce; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SignedRegistration {
		#[allow(missing_docs)]
		pub pubkey: <BLS::G1Point as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub signature: <BLS::G2Point as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub nonce: u64,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (BLS::G1Point, BLS::G2Point, alloy::sol_types::sol_data::Uint<64>);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			<BLS::G1Point as alloy::sol_types::SolType>::RustType,
			<BLS::G2Point as alloy::sol_types::SolType>::RustType,
			u64,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<SignedRegistration> for UnderlyingRustTuple<'_> {
			fn from(value: SignedRegistration) -> Self {
				(value.pubkey, value.signature, value.nonce)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedRegistration {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { pubkey: tuple.0, signature: tuple.1, nonce: tuple.2 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for SignedRegistration {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for SignedRegistration {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<BLS::G1Point as alloy_sol_types::SolType>::tokenize(&self.pubkey),
					<BLS::G2Point as alloy_sol_types::SolType>::tokenize(&self.signature),
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.nonce),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for SignedRegistration {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
				<Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolStruct for SignedRegistration {
			const NAME: &'static str = "SignedRegistration";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"SignedRegistration(G1Point pubkey,G2Point signature,uint64 nonce)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				let mut components = alloy_sol_types::private::Vec::with_capacity(2);
				components.push(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_components());
				components.push(<BLS::G2Point as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<BLS::G2Point as alloy_sol_types::SolStruct>::eip712_components());
				components
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<BLS::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.pubkey).0,
					<BLS::G2Point as alloy_sol_types::SolType>::eip712_data_word(&self.signature).0,
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(&self.nonce).0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for SignedRegistration {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <BLS::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.pubkey)
					+ <BLS::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.signature)
					+ <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.nonce,
					)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<BLS::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.pubkey, out);
				<BLS::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.signature, out);
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.nonce,
					out,
				);
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				let mut out = alloy_sol_types::private::Vec::new();
				<Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
				alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct SlasherCommitment { address committer; uint48 optedInAt; uint48 optedOutAt; bool slashed; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SlasherCommitment {
		#[allow(missing_docs)]
		pub committer: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub optedInAt: alloy::sol_types::private::primitives::aliases::U48,
		#[allow(missing_docs)]
		pub optedOutAt: alloy::sol_types::private::primitives::aliases::U48,
		#[allow(missing_docs)]
		pub slashed: bool,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			alloy::sol_types::sol_data::Address,
			alloy::sol_types::sol_data::Uint<48>,
			alloy::sol_types::sol_data::Uint<48>,
			alloy::sol_types::sol_data::Bool,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			alloy::sol_types::private::Address,
			alloy::sol_types::private::primitives::aliases::U48,
			alloy::sol_types::private::primitives::aliases::U48,
			bool,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<SlasherCommitment> for UnderlyingRustTuple<'_> {
			fn from(value: SlasherCommitment) -> Self {
				(value.committer, value.optedInAt, value.optedOutAt, value.slashed)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for SlasherCommitment {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { committer: tuple.0, optedInAt: tuple.1, optedOutAt: tuple.2, slashed: tuple.3 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for SlasherCommitment {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for SlasherCommitment {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.committer),
					<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::SolType>::tokenize(&self.optedInAt),
					<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::SolType>::tokenize(&self.optedOutAt),
					<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(&self.slashed),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for SlasherCommitment {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
				<Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolStruct for SlasherCommitment {
			const NAME: &'static str = "SlasherCommitment";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"SlasherCommitment(address committer,uint48 optedInAt,uint48 optedOutAt,bool slashed)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				alloy_sol_types::private::Vec::new()
			}
			#[inline]
			fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
				<Self as alloy_sol_types::SolStruct>::eip712_root_type()
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
						&self.committer,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::SolType>::eip712_data_word(
						&self.optedInAt,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::SolType>::eip712_data_word(
						&self.optedOutAt,
					)
					.0,
					<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(&self.slashed).0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for SlasherCommitment {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.committer,
					) + <alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.optedInAt,
				) + <alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.optedOutAt,
				) + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.slashed,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.committer,
					out,
				);
				<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.optedInAt,
					out,
				);
				<alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.optedOutAt,
					out,
				);
				<alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.slashed,
					out,
				);
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				let mut out = alloy_sol_types::private::Vec::new();
				<Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
				alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
			}
		}
	};
	use alloy::contract as alloy_contract;
	/**Creates a new wrapper around an on-chain [`IRegistry`](self) contract instance.

	See the [wrapper's documentation](`IRegistryInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		address: alloy_sol_types::private::Address,
		__provider: P,
	) -> IRegistryInstance<P, N> {
		IRegistryInstance::<P, N>::new(address, __provider)
	}
	/**A [`IRegistry`](self) instance.

	Contains type-safe methods for interacting with an on-chain instance of the
	[`IRegistry`](self) contract located at a given `address`, using a given
	provider `P`.

	If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
	documentation on how to provide it), the `deploy` and `deploy_builder` methods can
	be used to deploy a new instance of the contract.

	See the [module-level documentation](self) for all the available methods.*/
	#[derive(Clone)]
	pub struct IRegistryInstance<P, N = alloy_contract::private::Ethereum> {
		address: alloy_sol_types::private::Address,
		provider: P,
		_network: ::core::marker::PhantomData<N>,
	}
	#[automatically_derived]
	impl<P, N> ::core::fmt::Debug for IRegistryInstance<P, N> {
		#[inline]
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			f.debug_tuple("IRegistryInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> IRegistryInstance<P, N> {
		/**Creates a new wrapper around an on-chain [`IRegistry`](self) contract instance.

		See the [wrapper's documentation](`IRegistryInstance`) for more details.*/
		#[inline]
		pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
			Self { address, provider: __provider, _network: ::core::marker::PhantomData }
		}
		/// Returns a reference to the address.
		#[inline]
		pub const fn address(&self) -> &alloy_sol_types::private::Address {
			&self.address
		}
		/// Sets the address.
		#[inline]
		pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
			self.address = address;
		}
		/// Sets the address and returns `self`.
		pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
			self.set_address(address);
			self
		}
		/// Returns a reference to the provider.
		#[inline]
		pub const fn provider(&self) -> &P {
			&self.provider
		}
	}
	impl<P: ::core::clone::Clone, N> IRegistryInstance<&P, N> {
		/// Clones the provider and returns a new instance with the cloned provider.
		#[inline]
		pub fn with_cloned_provider(self) -> IRegistryInstance<P, N> {
			IRegistryInstance {
				address: self.address,
				provider: ::core::clone::Clone::clone(&self.provider),
				_network: ::core::marker::PhantomData,
			}
		}
	}
	/// Function calls.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> IRegistryInstance<P, N> {
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> IRegistryInstance<P, N> {
		/// Creates a new event filter using this contract instance's provider and address.
		///
		/// Note that the type can be any event, not just those defined in this contract.
		/// Prefer using the other methods for building type-safe event filters.
		pub fn event_filter<E: alloy_sol_types::SolEvent>(&self) -> alloy_contract::Event<&P, E, N> {
			alloy_contract::Event::new_sol(&self.provider, &self.address)
		}
	}
}
///Module containing a contract's types and functions.
/**

```solidity
library ISlasher {
	struct Commitment { uint64 commitmentType; bytes payload; bytes32 requestHash; address slasher; }
	struct Delegation { BLS.G1Point proposer; BLS.G1Point delegate; address committer; uint64 slot; bytes metadata; }
	struct SignedCommitment { Commitment commitment; uint64 nonce; bytes32 signingId; bytes signature; }
	struct SignedDelegation { Delegation delegation; uint64 nonce; bytes32 signingId; BLS.G2Point signature; }
}
```*/
#[allow(
	non_camel_case_types,
	non_snake_case,
	clippy::pub_underscore_fields,
	clippy::style,
	clippy::empty_structs_with_brackets
)]
pub mod ISlasher {
	use super::*;
	use alloy::sol_types as alloy_sol_types;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct Commitment { uint64 commitmentType; bytes payload; bytes32 requestHash; address slasher; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct Commitment {
		#[allow(missing_docs)]
		pub commitmentType: u64,
		#[allow(missing_docs)]
		pub payload: alloy::sol_types::private::Bytes,
		#[allow(missing_docs)]
		pub requestHash: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub slasher: alloy::sol_types::private::Address,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			alloy::sol_types::sol_data::Uint<64>,
			alloy::sol_types::sol_data::Bytes,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::Address,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			u64,
			alloy::sol_types::private::Bytes,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::Address,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<Commitment> for UnderlyingRustTuple<'_> {
			fn from(value: Commitment) -> Self {
				(value.commitmentType, value.payload, value.requestHash, value.slasher)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for Commitment {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { commitmentType: tuple.0, payload: tuple.1, requestHash: tuple.2, slasher: tuple.3 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for Commitment {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for Commitment {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.commitmentType),
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(&self.payload),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.requestHash,
					),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.slasher),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for Commitment {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
				<Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolStruct for Commitment {
			const NAME: &'static str = "Commitment";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"Commitment(uint64 commitmentType,bytes payload,bytes32 requestHash,address slasher)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				alloy_sol_types::private::Vec::new()
			}
			#[inline]
			fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
				<Self as alloy_sol_types::SolStruct>::eip712_root_type()
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(
						&self.commitmentType,
					)
					.0,
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(&self.payload).0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.requestHash,
					)
					.0,
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(&self.slasher)
						.0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for Commitment {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.commitmentType,
					) + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.payload,
				) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.requestHash,
				) + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.slasher,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.commitmentType,
					out,
				);
				<alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.payload,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.requestHash,
					out,
				);
				<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.slasher,
					out,
				);
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				let mut out = alloy_sol_types::private::Vec::new();
				<Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
				alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct Delegation { BLS.G1Point proposer; BLS.G1Point delegate; address committer; uint64 slot; bytes metadata; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct Delegation {
		#[allow(missing_docs)]
		pub proposer: <BLS::G1Point as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub delegate: <BLS::G1Point as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub committer: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub slot: u64,
		#[allow(missing_docs)]
		pub metadata: alloy::sol_types::private::Bytes,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			BLS::G1Point,
			BLS::G1Point,
			alloy::sol_types::sol_data::Address,
			alloy::sol_types::sol_data::Uint<64>,
			alloy::sol_types::sol_data::Bytes,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			<BLS::G1Point as alloy::sol_types::SolType>::RustType,
			<BLS::G1Point as alloy::sol_types::SolType>::RustType,
			alloy::sol_types::private::Address,
			u64,
			alloy::sol_types::private::Bytes,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<Delegation> for UnderlyingRustTuple<'_> {
			fn from(value: Delegation) -> Self {
				(value.proposer, value.delegate, value.committer, value.slot, value.metadata)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for Delegation {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { proposer: tuple.0, delegate: tuple.1, committer: tuple.2, slot: tuple.3, metadata: tuple.4 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for Delegation {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for Delegation {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<BLS::G1Point as alloy_sol_types::SolType>::tokenize(&self.proposer),
					<BLS::G1Point as alloy_sol_types::SolType>::tokenize(&self.delegate),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.committer),
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.slot),
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(&self.metadata),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for Delegation {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
				<Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolStruct for Delegation {
			const NAME: &'static str = "Delegation";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"Delegation(G1Point proposer,G1Point delegate,address committer,uint64 slot,bytes metadata)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				let mut components = alloy_sol_types::private::Vec::with_capacity(2);
				components.push(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_components());
				components.push(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<BLS::G1Point as alloy_sol_types::SolStruct>::eip712_components());
				components
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<BLS::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.proposer).0,
					<BLS::G1Point as alloy_sol_types::SolType>::eip712_data_word(&self.delegate).0,
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
						&self.committer,
					)
					.0,
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(&self.slot).0,
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(&self.metadata).0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for Delegation {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <BLS::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.proposer)
					+ <BLS::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.delegate)
					+ <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.committer,
					) + <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.slot,
				) + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.metadata,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<BLS::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.proposer, out);
				<BLS::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.delegate, out);
				<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.committer,
					out,
				);
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.slot, out,
				);
				<alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.metadata,
					out,
				);
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				let mut out = alloy_sol_types::private::Vec::new();
				<Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
				alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct SignedCommitment { Commitment commitment; uint64 nonce; bytes32 signingId; bytes signature; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SignedCommitment {
		#[allow(missing_docs)]
		pub commitment: <Commitment as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub nonce: u64,
		#[allow(missing_docs)]
		pub signingId: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub signature: alloy::sol_types::private::Bytes,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			Commitment,
			alloy::sol_types::sol_data::Uint<64>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			alloy::sol_types::sol_data::Bytes,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			<Commitment as alloy::sol_types::SolType>::RustType,
			u64,
			alloy::sol_types::private::FixedBytes<32>,
			alloy::sol_types::private::Bytes,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<SignedCommitment> for UnderlyingRustTuple<'_> {
			fn from(value: SignedCommitment) -> Self {
				(value.commitment, value.nonce, value.signingId, value.signature)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedCommitment {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { commitment: tuple.0, nonce: tuple.1, signingId: tuple.2, signature: tuple.3 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for SignedCommitment {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for SignedCommitment {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<Commitment as alloy_sol_types::SolType>::tokenize(&self.commitment),
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.nonce),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.signingId),
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(&self.signature),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for SignedCommitment {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
				<Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolStruct for SignedCommitment {
			const NAME: &'static str = "SignedCommitment";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"SignedCommitment(Commitment commitment,uint64 nonce,bytes32 signingId,bytes signature)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				let mut components = alloy_sol_types::private::Vec::with_capacity(1);
				components.push(<Commitment as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<Commitment as alloy_sol_types::SolStruct>::eip712_components());
				components
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<Commitment as alloy_sol_types::SolType>::eip712_data_word(&self.commitment).0,
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(&self.nonce).0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.signingId,
					)
					.0,
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(&self.signature)
						.0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for SignedCommitment {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <Commitment as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.commitment)
					+ <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.nonce,
					) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.signingId,
				) + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.signature,
				)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<Commitment as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.commitment, out);
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.nonce,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.signingId,
					out,
				);
				<alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.signature,
					out,
				);
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				let mut out = alloy_sol_types::private::Vec::new();
				<Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
				alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**```solidity
	struct SignedDelegation { Delegation delegation; uint64 nonce; bytes32 signingId; BLS.G2Point signature; }
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SignedDelegation {
		#[allow(missing_docs)]
		pub delegation: <Delegation as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub nonce: u64,
		#[allow(missing_docs)]
		pub signingId: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub signature: <BLS::G2Point as alloy::sol_types::SolType>::RustType,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = (
			Delegation,
			alloy::sol_types::sol_data::Uint<64>,
			alloy::sol_types::sol_data::FixedBytes<32>,
			BLS::G2Point,
		);
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = (
			<Delegation as alloy::sol_types::SolType>::RustType,
			u64,
			alloy::sol_types::private::FixedBytes<32>,
			<BLS::G2Point as alloy::sol_types::SolType>::RustType,
		);
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<SignedDelegation> for UnderlyingRustTuple<'_> {
			fn from(value: SignedDelegation) -> Self {
				(value.delegation, value.nonce, value.signingId, value.signature)
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedDelegation {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self { delegation: tuple.0, nonce: tuple.1, signingId: tuple.2, signature: tuple.3 }
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolValue for SignedDelegation {
			type SolType = Self;
		}
		#[automatically_derived]
		impl alloy_sol_types::private::SolTypeValue<Self> for SignedDelegation {
			#[inline]
			fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
				(
					<Delegation as alloy_sol_types::SolType>::tokenize(&self.delegation),
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(&self.nonce),
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(&self.signingId),
					<BLS::G2Point as alloy_sol_types::SolType>::tokenize(&self.signature),
				)
			}
			#[inline]
			fn stv_abi_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
			}
			#[inline]
			fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
				<Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
			}
			#[inline]
			fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
			}
			#[inline]
			fn stv_abi_packed_encoded_size(&self) -> usize {
				if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
					return size;
				}
				let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolType for SignedDelegation {
			type RustType = Self;
			type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
			const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
			const PACKED_ENCODED_SIZE: Option<usize> =
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
			#[inline]
			fn valid_token(token: &Self::Token<'_>) -> bool {
				<UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
			}
			#[inline]
			fn detokenize(token: Self::Token<'_>) -> Self::RustType {
				let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
				<Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolStruct for SignedDelegation {
			const NAME: &'static str = "SignedDelegation";
			#[inline]
			fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
				alloy_sol_types::private::Cow::Borrowed(
					"SignedDelegation(Delegation delegation,uint64 nonce,bytes32 signingId,G2Point signature)",
				)
			}
			#[inline]
			fn eip712_components() -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>> {
				let mut components = alloy_sol_types::private::Vec::with_capacity(2);
				components.push(<Delegation as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<Delegation as alloy_sol_types::SolStruct>::eip712_components());
				components.push(<BLS::G2Point as alloy_sol_types::SolStruct>::eip712_root_type());
				components.extend(<BLS::G2Point as alloy_sol_types::SolStruct>::eip712_components());
				components
			}
			#[inline]
			fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
				[
					<Delegation as alloy_sol_types::SolType>::eip712_data_word(&self.delegation).0,
					<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::eip712_data_word(&self.nonce).0,
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::eip712_data_word(
						&self.signingId,
					)
					.0,
					<BLS::G2Point as alloy_sol_types::SolType>::eip712_data_word(&self.signature).0,
				]
				.concat()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::EventTopic for SignedDelegation {
			#[inline]
			fn topic_preimage_length(rust: &Self::RustType) -> usize {
				0usize
					+ <Delegation as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.delegation)
					+ <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::topic_preimage_length(
						&rust.nonce,
					) + <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::topic_preimage_length(
					&rust.signingId,
				) + <BLS::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.signature)
			}
			#[inline]
			fn encode_topic_preimage(rust: &Self::RustType, out: &mut alloy_sol_types::private::Vec<u8>) {
				out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
				<Delegation as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.delegation, out);
				<alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.nonce,
					out,
				);
				<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic_preimage(
					&rust.signingId,
					out,
				);
				<BLS::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.signature, out);
			}
			#[inline]
			fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
				let mut out = alloy_sol_types::private::Vec::new();
				<Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
				alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
			}
		}
	};
	use alloy::contract as alloy_contract;
	/**Creates a new wrapper around an on-chain [`ISlasher`](self) contract instance.

	See the [wrapper's documentation](`ISlasherInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		address: alloy_sol_types::private::Address,
		__provider: P,
	) -> ISlasherInstance<P, N> {
		ISlasherInstance::<P, N>::new(address, __provider)
	}
	/**A [`ISlasher`](self) instance.

	Contains type-safe methods for interacting with an on-chain instance of the
	[`ISlasher`](self) contract located at a given `address`, using a given
	provider `P`.

	If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
	documentation on how to provide it), the `deploy` and `deploy_builder` methods can
	be used to deploy a new instance of the contract.

	See the [module-level documentation](self) for all the available methods.*/
	#[derive(Clone)]
	pub struct ISlasherInstance<P, N = alloy_contract::private::Ethereum> {
		address: alloy_sol_types::private::Address,
		provider: P,
		_network: ::core::marker::PhantomData<N>,
	}
	#[automatically_derived]
	impl<P, N> ::core::fmt::Debug for ISlasherInstance<P, N> {
		#[inline]
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			f.debug_tuple("ISlasherInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> ISlasherInstance<P, N> {
		/**Creates a new wrapper around an on-chain [`ISlasher`](self) contract instance.

		See the [wrapper's documentation](`ISlasherInstance`) for more details.*/
		#[inline]
		pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
			Self { address, provider: __provider, _network: ::core::marker::PhantomData }
		}
		/// Returns a reference to the address.
		#[inline]
		pub const fn address(&self) -> &alloy_sol_types::private::Address {
			&self.address
		}
		/// Sets the address.
		#[inline]
		pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
			self.address = address;
		}
		/// Sets the address and returns `self`.
		pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
			self.set_address(address);
			self
		}
		/// Returns a reference to the provider.
		#[inline]
		pub const fn provider(&self) -> &P {
			&self.provider
		}
	}
	impl<P: ::core::clone::Clone, N> ISlasherInstance<&P, N> {
		/// Clones the provider and returns a new instance with the cloned provider.
		#[inline]
		pub fn with_cloned_provider(self) -> ISlasherInstance<P, N> {
			ISlasherInstance {
				address: self.address,
				provider: ::core::clone::Clone::clone(&self.provider),
				_network: ::core::marker::PhantomData,
			}
		}
	}
	/// Function calls.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> ISlasherInstance<P, N> {
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> ISlasherInstance<P, N> {
		/// Creates a new event filter using this contract instance's provider and address.
		///
		/// Note that the type can be any event, not just those defined in this contract.
		/// Prefer using the other methods for building type-safe event filters.
		pub fn event_filter<E: alloy_sol_types::SolEvent>(&self) -> alloy_contract::Event<&P, E, N> {
			alloy_contract::Event::new_sol(&self.provider, &self.address)
		}
	}
}
/**

Generated by the following Solidity interface...
```solidity
library BLS {
	struct G1Point {
		bytes32 x_a;
		bytes32 x_b;
		bytes32 y_a;
		bytes32 y_b;
	}
	struct G2Point {
		bytes32 x_c0_a;
		bytes32 x_c0_b;
		bytes32 x_c1_a;
		bytes32 x_c1_b;
		bytes32 y_c0_a;
		bytes32 y_c0_b;
		bytes32 y_c1_a;
		bytes32 y_c1_b;
	}
}

library IRegistry {
	type SlashingType is uint8;
	struct Config {
		uint80 minCollateralWei;
		uint32 fraudProofWindow;
		uint32 unregistrationDelay;
		uint32 slashWindow;
		uint32 optInDelay;
		bytes32 signingDomain;
		bytes32 chainId;
	}
	struct OperatorData {
		address owner;
		uint80 collateralWei;
		uint16 numKeys;
		uint48 registeredAt;
		uint48 unregisteredAt;
		uint48 slashedAt;
		bool deleted;
		bool equivocated;
	}
	struct RegistrationProof {
		bytes32 registrationRoot;
		SignedRegistration registration;
		bytes32[] merkleProof;
		bytes32 signingId;
	}
	struct SignedRegistration {
		BLS.G1Point pubkey;
		BLS.G2Point signature;
		uint64 nonce;
	}
	struct SlasherCommitment {
		address committer;
		uint48 optedInAt;
		uint48 optedOutAt;
		bool slashed;
	}
}

library ISlasher {
	struct Commitment {
		uint64 commitmentType;
		bytes payload;
		bytes32 requestHash;
		address slasher;
	}
	struct Delegation {
		BLS.G1Point proposer;
		BLS.G1Point delegate;
		address committer;
		uint64 slot;
		bytes metadata;
	}
	struct SignedCommitment {
		Commitment commitment;
		uint64 nonce;
		bytes32 signingId;
		bytes signature;
	}
	struct SignedDelegation {
		Delegation delegation;
		uint64 nonce;
		bytes32 signingId;
		BLS.G2Point signature;
	}
}

interface Registry {
	error AlreadyOptedIn();
	error AlreadyUnregistered();
	error CollateralBelowMinimum();
	error CollateralOverflow();
	error DelegationSignatureInvalid();
	error DelegationsAreSame();
	error DifferentSlots();
	error EthTransferFailed();
	error FraudProofChallengeInvalid();
	error FraudProofMerklePathInvalid();
	error FraudProofWindowExpired();
	error FraudProofWindowNotMet();
	error InsufficientCollateral();
	error InvalidDelegation();
	error InvalidOwnerAddress();
	error InvalidProof();
	error InvalidRegistrationRoot();
	error NoCollateral();
	error NoCollateralSlashed();
	error NoCollateralToClaim();
	error NotOptedIn();
	error NotRegisteredKey();
	error NotSlashed();
	error NotUnregistered();
	error OperatorAlreadyEquivocated();
	error OperatorAlreadyRegistered();
	error OperatorAlreadyUnregistered();
	error OperatorDeleted();
	error OptInDelayNotMet();
	error SlashAmountExceedsCollateral();
	error SlashWindowExpired();
	error SlashWindowNotMet();
	error SlashingAlreadyOccurred();
	error TimestampTooOld();
	error UnauthorizedCommitment();
	error UnregistrationDelayNotMet();
	error WrongOperator();

	event CollateralAdded(bytes32 indexed registrationRoot, uint256 collateralWei);
	event CollateralClaimed(bytes32 indexed registrationRoot, uint256 collateralWei);
	event OperatorOptedIn(bytes32 indexed registrationRoot, address indexed slasher, address indexed committer);
	event OperatorOptedOut(bytes32 indexed registrationRoot, address indexed slasher);
	event OperatorRegistered(bytes32 indexed registrationRoot, uint256 collateralWei, address owner);
	event OperatorSlashed(IRegistry.SlashingType slashingType, bytes32 indexed registrationRoot, address owner, address challenger, address indexed slasher, uint256 slashAmountWei);
	event OperatorUnregistered(bytes32 indexed registrationRoot);

	constructor(IRegistry.Config _config);

	function addCollateral(bytes32 registrationRoot) external payable;
	function claimCollateral(bytes32 registrationRoot) external;
	function claimSlashedCollateral(bytes32 registrationRoot) external;
	function getConfig() external view returns (IRegistry.Config memory);
	function getHistoricalCollateral(bytes32 registrationRoot, uint256 timestamp) external view returns (uint256 collateralWei);
	function getOperatorData(bytes32 registrationRoot) external view returns (IRegistry.OperatorData memory operatorData);
	function getRegistrationProof(IRegistry.SignedRegistration[] memory regs, address owner, uint256 leafIndex, bytes32 signingId) external pure returns (IRegistry.RegistrationProof memory proof);
	function getSlasherCommitment(bytes32 registrationRoot, address slasher) external view returns (IRegistry.SlasherCommitment memory);
	function getVerifiedOperatorData(IRegistry.RegistrationProof memory proof) external view returns (IRegistry.OperatorData memory);
	function isOptedIntoSlasher(bytes32 registrationRoot, address slasher) external view returns (bool);
	function isSlashed(bytes32 registrationRoot) external view returns (bool slashed);
	function isSlashed(bytes32 registrationRoot, address slasher) external view returns (bool slashed);
	function optInToSlasher(bytes32 registrationRoot, address slasher, address committer) external;
	function optOutOfSlasher(bytes32 registrationRoot, address slasher) external;
	function register(IRegistry.SignedRegistration[] memory registrations, address owner, bytes32 signingId) external payable returns (bytes32 registrationRoot);
	function slashCommitment(bytes32 registrationRoot, ISlasher.SignedCommitment memory commitment, bytes memory evidence) external returns (uint256 slashAmountWei);
	function slashCommitment(IRegistry.RegistrationProof memory proof, ISlasher.SignedDelegation memory delegation, ISlasher.SignedCommitment memory commitment, bytes memory evidence) external returns (uint256 slashAmountWei);
	function slashEquivocation(IRegistry.RegistrationProof memory proof, ISlasher.SignedDelegation memory delegationOne, ISlasher.SignedDelegation memory delegationTwo) external returns (uint256 slashAmountWei);
	function slashRegistration(IRegistry.RegistrationProof memory proof) external returns (uint256 slashedCollateralWei);
	function slashingEvidenceAlreadyUsed(bytes32 slashingDigest) external view returns (bool);
	function unregister(bytes32 registrationRoot) external;
	function verifyMerkleProof(IRegistry.RegistrationProof memory proof) external view;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
	"type": "constructor",
	"inputs": [
	  {
		"name": "_config",
		"type": "tuple",
		"internalType": "struct IRegistry.Config",
		"components": [
		  {
			"name": "minCollateralWei",
			"type": "uint80",
			"internalType": "uint80"
		  },
		  {
			"name": "fraudProofWindow",
			"type": "uint32",
			"internalType": "uint32"
		  },
		  {
			"name": "unregistrationDelay",
			"type": "uint32",
			"internalType": "uint32"
		  },
		  {
			"name": "slashWindow",
			"type": "uint32",
			"internalType": "uint32"
		  },
		  {
			"name": "optInDelay",
			"type": "uint32",
			"internalType": "uint32"
		  },
		  {
			"name": "signingDomain",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "chainId",
			"type": "bytes32",
			"internalType": "bytes32"
		  }
		]
	  }
	],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "addCollateral",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [],
	"stateMutability": "payable"
  },
  {
	"type": "function",
	"name": "claimCollateral",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "claimSlashedCollateral",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "getConfig",
	"inputs": [],
	"outputs": [
	  {
		"name": "",
		"type": "tuple",
		"internalType": "struct IRegistry.Config",
		"components": [
		  {
			"name": "minCollateralWei",
			"type": "uint80",
			"internalType": "uint80"
		  },
		  {
			"name": "fraudProofWindow",
			"type": "uint32",
			"internalType": "uint32"
		  },
		  {
			"name": "unregistrationDelay",
			"type": "uint32",
			"internalType": "uint32"
		  },
		  {
			"name": "slashWindow",
			"type": "uint32",
			"internalType": "uint32"
		  },
		  {
			"name": "optInDelay",
			"type": "uint32",
			"internalType": "uint32"
		  },
		  {
			"name": "signingDomain",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "chainId",
			"type": "bytes32",
			"internalType": "bytes32"
		  }
		]
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "getHistoricalCollateral",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  },
	  {
		"name": "timestamp",
		"type": "uint256",
		"internalType": "uint256"
	  }
	],
	"outputs": [
	  {
		"name": "collateralWei",
		"type": "uint256",
		"internalType": "uint256"
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "getOperatorData",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [
	  {
		"name": "operatorData",
		"type": "tuple",
		"internalType": "struct IRegistry.OperatorData",
		"components": [
		  {
			"name": "owner",
			"type": "address",
			"internalType": "address"
		  },
		  {
			"name": "collateralWei",
			"type": "uint80",
			"internalType": "uint80"
		  },
		  {
			"name": "numKeys",
			"type": "uint16",
			"internalType": "uint16"
		  },
		  {
			"name": "registeredAt",
			"type": "uint48",
			"internalType": "uint48"
		  },
		  {
			"name": "unregisteredAt",
			"type": "uint48",
			"internalType": "uint48"
		  },
		  {
			"name": "slashedAt",
			"type": "uint48",
			"internalType": "uint48"
		  },
		  {
			"name": "deleted",
			"type": "bool",
			"internalType": "bool"
		  },
		  {
			"name": "equivocated",
			"type": "bool",
			"internalType": "bool"
		  }
		]
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "getRegistrationProof",
	"inputs": [
	  {
		"name": "regs",
		"type": "tuple[]",
		"internalType": "struct IRegistry.SignedRegistration[]",
		"components": [
		  {
			"name": "pubkey",
			"type": "tuple",
			"internalType": "struct BLS.G1Point",
			"components": [
			  {
				"name": "x_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  }
			]
		  },
		  {
			"name": "signature",
			"type": "tuple",
			"internalType": "struct BLS.G2Point",
			"components": [
			  {
				"name": "x_c0_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c0_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c1_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c1_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c0_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c0_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c1_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c1_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  }
			]
		  },
		  {
			"name": "nonce",
			"type": "uint64",
			"internalType": "uint64"
		  }
		]
	  },
	  {
		"name": "owner",
		"type": "address",
		"internalType": "address"
	  },
	  {
		"name": "leafIndex",
		"type": "uint256",
		"internalType": "uint256"
	  },
	  {
		"name": "signingId",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [
	  {
		"name": "proof",
		"type": "tuple",
		"internalType": "struct IRegistry.RegistrationProof",
		"components": [
		  {
			"name": "registrationRoot",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "registration",
			"type": "tuple",
			"internalType": "struct IRegistry.SignedRegistration",
			"components": [
			  {
				"name": "pubkey",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "signature",
				"type": "tuple",
				"internalType": "struct BLS.G2Point",
				"components": [
				  {
					"name": "x_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "nonce",
				"type": "uint64",
				"internalType": "uint64"
			  }
			]
		  },
		  {
			"name": "merkleProof",
			"type": "bytes32[]",
			"internalType": "bytes32[]"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  }
		]
	  }
	],
	"stateMutability": "pure"
  },
  {
	"type": "function",
	"name": "getSlasherCommitment",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  },
	  {
		"name": "slasher",
		"type": "address",
		"internalType": "address"
	  }
	],
	"outputs": [
	  {
		"name": "",
		"type": "tuple",
		"internalType": "struct IRegistry.SlasherCommitment",
		"components": [
		  {
			"name": "committer",
			"type": "address",
			"internalType": "address"
		  },
		  {
			"name": "optedInAt",
			"type": "uint48",
			"internalType": "uint48"
		  },
		  {
			"name": "optedOutAt",
			"type": "uint48",
			"internalType": "uint48"
		  },
		  {
			"name": "slashed",
			"type": "bool",
			"internalType": "bool"
		  }
		]
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "getVerifiedOperatorData",
	"inputs": [
	  {
		"name": "proof",
		"type": "tuple",
		"internalType": "struct IRegistry.RegistrationProof",
		"components": [
		  {
			"name": "registrationRoot",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "registration",
			"type": "tuple",
			"internalType": "struct IRegistry.SignedRegistration",
			"components": [
			  {
				"name": "pubkey",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "signature",
				"type": "tuple",
				"internalType": "struct BLS.G2Point",
				"components": [
				  {
					"name": "x_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "nonce",
				"type": "uint64",
				"internalType": "uint64"
			  }
			]
		  },
		  {
			"name": "merkleProof",
			"type": "bytes32[]",
			"internalType": "bytes32[]"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  }
		]
	  }
	],
	"outputs": [
	  {
		"name": "",
		"type": "tuple",
		"internalType": "struct IRegistry.OperatorData",
		"components": [
		  {
			"name": "owner",
			"type": "address",
			"internalType": "address"
		  },
		  {
			"name": "collateralWei",
			"type": "uint80",
			"internalType": "uint80"
		  },
		  {
			"name": "numKeys",
			"type": "uint16",
			"internalType": "uint16"
		  },
		  {
			"name": "registeredAt",
			"type": "uint48",
			"internalType": "uint48"
		  },
		  {
			"name": "unregisteredAt",
			"type": "uint48",
			"internalType": "uint48"
		  },
		  {
			"name": "slashedAt",
			"type": "uint48",
			"internalType": "uint48"
		  },
		  {
			"name": "deleted",
			"type": "bool",
			"internalType": "bool"
		  },
		  {
			"name": "equivocated",
			"type": "bool",
			"internalType": "bool"
		  }
		]
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "isOptedIntoSlasher",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  },
	  {
		"name": "slasher",
		"type": "address",
		"internalType": "address"
	  }
	],
	"outputs": [
	  {
		"name": "",
		"type": "bool",
		"internalType": "bool"
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "isSlashed",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [
	  {
		"name": "slashed",
		"type": "bool",
		"internalType": "bool"
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "isSlashed",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  },
	  {
		"name": "slasher",
		"type": "address",
		"internalType": "address"
	  }
	],
	"outputs": [
	  {
		"name": "slashed",
		"type": "bool",
		"internalType": "bool"
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "optInToSlasher",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  },
	  {
		"name": "slasher",
		"type": "address",
		"internalType": "address"
	  },
	  {
		"name": "committer",
		"type": "address",
		"internalType": "address"
	  }
	],
	"outputs": [],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "optOutOfSlasher",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  },
	  {
		"name": "slasher",
		"type": "address",
		"internalType": "address"
	  }
	],
	"outputs": [],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "register",
	"inputs": [
	  {
		"name": "registrations",
		"type": "tuple[]",
		"internalType": "struct IRegistry.SignedRegistration[]",
		"components": [
		  {
			"name": "pubkey",
			"type": "tuple",
			"internalType": "struct BLS.G1Point",
			"components": [
			  {
				"name": "x_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  }
			]
		  },
		  {
			"name": "signature",
			"type": "tuple",
			"internalType": "struct BLS.G2Point",
			"components": [
			  {
				"name": "x_c0_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c0_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c1_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c1_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c0_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c0_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c1_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c1_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  }
			]
		  },
		  {
			"name": "nonce",
			"type": "uint64",
			"internalType": "uint64"
		  }
		]
	  },
	  {
		"name": "owner",
		"type": "address",
		"internalType": "address"
	  },
	  {
		"name": "signingId",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"stateMutability": "payable"
  },
  {
	"type": "function",
	"name": "slashCommitment",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  },
	  {
		"name": "commitment",
		"type": "tuple",
		"internalType": "struct ISlasher.SignedCommitment",
		"components": [
		  {
			"name": "commitment",
			"type": "tuple",
			"internalType": "struct ISlasher.Commitment",
			"components": [
			  {
				"name": "commitmentType",
				"type": "uint64",
				"internalType": "uint64"
			  },
			  {
				"name": "payload",
				"type": "bytes",
				"internalType": "bytes"
			  },
			  {
				"name": "requestHash",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "slasher",
				"type": "address",
				"internalType": "address"
			  }
			]
		  },
		  {
			"name": "nonce",
			"type": "uint64",
			"internalType": "uint64"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "signature",
			"type": "bytes",
			"internalType": "bytes"
		  }
		]
	  },
	  {
		"name": "evidence",
		"type": "bytes",
		"internalType": "bytes"
	  }
	],
	"outputs": [
	  {
		"name": "slashAmountWei",
		"type": "uint256",
		"internalType": "uint256"
	  }
	],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "slashCommitment",
	"inputs": [
	  {
		"name": "proof",
		"type": "tuple",
		"internalType": "struct IRegistry.RegistrationProof",
		"components": [
		  {
			"name": "registrationRoot",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "registration",
			"type": "tuple",
			"internalType": "struct IRegistry.SignedRegistration",
			"components": [
			  {
				"name": "pubkey",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "signature",
				"type": "tuple",
				"internalType": "struct BLS.G2Point",
				"components": [
				  {
					"name": "x_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "nonce",
				"type": "uint64",
				"internalType": "uint64"
			  }
			]
		  },
		  {
			"name": "merkleProof",
			"type": "bytes32[]",
			"internalType": "bytes32[]"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  }
		]
	  },
	  {
		"name": "delegation",
		"type": "tuple",
		"internalType": "struct ISlasher.SignedDelegation",
		"components": [
		  {
			"name": "delegation",
			"type": "tuple",
			"internalType": "struct ISlasher.Delegation",
			"components": [
			  {
				"name": "proposer",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "delegate",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "committer",
				"type": "address",
				"internalType": "address"
			  },
			  {
				"name": "slot",
				"type": "uint64",
				"internalType": "uint64"
			  },
			  {
				"name": "metadata",
				"type": "bytes",
				"internalType": "bytes"
			  }
			]
		  },
		  {
			"name": "nonce",
			"type": "uint64",
			"internalType": "uint64"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "signature",
			"type": "tuple",
			"internalType": "struct BLS.G2Point",
			"components": [
			  {
				"name": "x_c0_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c0_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c1_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c1_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c0_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c0_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c1_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c1_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  }
			]
		  }
		]
	  },
	  {
		"name": "commitment",
		"type": "tuple",
		"internalType": "struct ISlasher.SignedCommitment",
		"components": [
		  {
			"name": "commitment",
			"type": "tuple",
			"internalType": "struct ISlasher.Commitment",
			"components": [
			  {
				"name": "commitmentType",
				"type": "uint64",
				"internalType": "uint64"
			  },
			  {
				"name": "payload",
				"type": "bytes",
				"internalType": "bytes"
			  },
			  {
				"name": "requestHash",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "slasher",
				"type": "address",
				"internalType": "address"
			  }
			]
		  },
		  {
			"name": "nonce",
			"type": "uint64",
			"internalType": "uint64"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "signature",
			"type": "bytes",
			"internalType": "bytes"
		  }
		]
	  },
	  {
		"name": "evidence",
		"type": "bytes",
		"internalType": "bytes"
	  }
	],
	"outputs": [
	  {
		"name": "slashAmountWei",
		"type": "uint256",
		"internalType": "uint256"
	  }
	],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "slashEquivocation",
	"inputs": [
	  {
		"name": "proof",
		"type": "tuple",
		"internalType": "struct IRegistry.RegistrationProof",
		"components": [
		  {
			"name": "registrationRoot",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "registration",
			"type": "tuple",
			"internalType": "struct IRegistry.SignedRegistration",
			"components": [
			  {
				"name": "pubkey",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "signature",
				"type": "tuple",
				"internalType": "struct BLS.G2Point",
				"components": [
				  {
					"name": "x_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "nonce",
				"type": "uint64",
				"internalType": "uint64"
			  }
			]
		  },
		  {
			"name": "merkleProof",
			"type": "bytes32[]",
			"internalType": "bytes32[]"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  }
		]
	  },
	  {
		"name": "delegationOne",
		"type": "tuple",
		"internalType": "struct ISlasher.SignedDelegation",
		"components": [
		  {
			"name": "delegation",
			"type": "tuple",
			"internalType": "struct ISlasher.Delegation",
			"components": [
			  {
				"name": "proposer",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "delegate",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "committer",
				"type": "address",
				"internalType": "address"
			  },
			  {
				"name": "slot",
				"type": "uint64",
				"internalType": "uint64"
			  },
			  {
				"name": "metadata",
				"type": "bytes",
				"internalType": "bytes"
			  }
			]
		  },
		  {
			"name": "nonce",
			"type": "uint64",
			"internalType": "uint64"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "signature",
			"type": "tuple",
			"internalType": "struct BLS.G2Point",
			"components": [
			  {
				"name": "x_c0_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c0_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c1_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c1_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c0_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c0_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c1_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c1_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  }
			]
		  }
		]
	  },
	  {
		"name": "delegationTwo",
		"type": "tuple",
		"internalType": "struct ISlasher.SignedDelegation",
		"components": [
		  {
			"name": "delegation",
			"type": "tuple",
			"internalType": "struct ISlasher.Delegation",
			"components": [
			  {
				"name": "proposer",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "delegate",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "committer",
				"type": "address",
				"internalType": "address"
			  },
			  {
				"name": "slot",
				"type": "uint64",
				"internalType": "uint64"
			  },
			  {
				"name": "metadata",
				"type": "bytes",
				"internalType": "bytes"
			  }
			]
		  },
		  {
			"name": "nonce",
			"type": "uint64",
			"internalType": "uint64"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "signature",
			"type": "tuple",
			"internalType": "struct BLS.G2Point",
			"components": [
			  {
				"name": "x_c0_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c0_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c1_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "x_c1_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c0_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c0_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c1_a",
				"type": "bytes32",
				"internalType": "bytes32"
			  },
			  {
				"name": "y_c1_b",
				"type": "bytes32",
				"internalType": "bytes32"
			  }
			]
		  }
		]
	  }
	],
	"outputs": [
	  {
		"name": "slashAmountWei",
		"type": "uint256",
		"internalType": "uint256"
	  }
	],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "slashRegistration",
	"inputs": [
	  {
		"name": "proof",
		"type": "tuple",
		"internalType": "struct IRegistry.RegistrationProof",
		"components": [
		  {
			"name": "registrationRoot",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "registration",
			"type": "tuple",
			"internalType": "struct IRegistry.SignedRegistration",
			"components": [
			  {
				"name": "pubkey",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "signature",
				"type": "tuple",
				"internalType": "struct BLS.G2Point",
				"components": [
				  {
					"name": "x_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "nonce",
				"type": "uint64",
				"internalType": "uint64"
			  }
			]
		  },
		  {
			"name": "merkleProof",
			"type": "bytes32[]",
			"internalType": "bytes32[]"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  }
		]
	  }
	],
	"outputs": [
	  {
		"name": "slashedCollateralWei",
		"type": "uint256",
		"internalType": "uint256"
	  }
	],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "slashingEvidenceAlreadyUsed",
	"inputs": [
	  {
		"name": "slashingDigest",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [
	  {
		"name": "",
		"type": "bool",
		"internalType": "bool"
	  }
	],
	"stateMutability": "view"
  },
  {
	"type": "function",
	"name": "unregister",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"internalType": "bytes32"
	  }
	],
	"outputs": [],
	"stateMutability": "nonpayable"
  },
  {
	"type": "function",
	"name": "verifyMerkleProof",
	"inputs": [
	  {
		"name": "proof",
		"type": "tuple",
		"internalType": "struct IRegistry.RegistrationProof",
		"components": [
		  {
			"name": "registrationRoot",
			"type": "bytes32",
			"internalType": "bytes32"
		  },
		  {
			"name": "registration",
			"type": "tuple",
			"internalType": "struct IRegistry.SignedRegistration",
			"components": [
			  {
				"name": "pubkey",
				"type": "tuple",
				"internalType": "struct BLS.G1Point",
				"components": [
				  {
					"name": "x_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "signature",
				"type": "tuple",
				"internalType": "struct BLS.G2Point",
				"components": [
				  {
					"name": "x_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "x_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c0_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_a",
					"type": "bytes32",
					"internalType": "bytes32"
				  },
				  {
					"name": "y_c1_b",
					"type": "bytes32",
					"internalType": "bytes32"
				  }
				]
			  },
			  {
				"name": "nonce",
				"type": "uint64",
				"internalType": "uint64"
			  }
			]
		  },
		  {
			"name": "merkleProof",
			"type": "bytes32[]",
			"internalType": "bytes32[]"
		  },
		  {
			"name": "signingId",
			"type": "bytes32",
			"internalType": "bytes32"
		  }
		]
	  }
	],
	"outputs": [],
	"stateMutability": "view"
  },
  {
	"type": "event",
	"name": "CollateralAdded",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"indexed": true,
		"internalType": "bytes32"
	  },
	  {
		"name": "collateralWei",
		"type": "uint256",
		"indexed": false,
		"internalType": "uint256"
	  }
	],
	"anonymous": false
  },
  {
	"type": "event",
	"name": "CollateralClaimed",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"indexed": true,
		"internalType": "bytes32"
	  },
	  {
		"name": "collateralWei",
		"type": "uint256",
		"indexed": false,
		"internalType": "uint256"
	  }
	],
	"anonymous": false
  },
  {
	"type": "event",
	"name": "OperatorOptedIn",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"indexed": true,
		"internalType": "bytes32"
	  },
	  {
		"name": "slasher",
		"type": "address",
		"indexed": true,
		"internalType": "address"
	  },
	  {
		"name": "committer",
		"type": "address",
		"indexed": true,
		"internalType": "address"
	  }
	],
	"anonymous": false
  },
  {
	"type": "event",
	"name": "OperatorOptedOut",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"indexed": true,
		"internalType": "bytes32"
	  },
	  {
		"name": "slasher",
		"type": "address",
		"indexed": true,
		"internalType": "address"
	  }
	],
	"anonymous": false
  },
  {
	"type": "event",
	"name": "OperatorRegistered",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"indexed": true,
		"internalType": "bytes32"
	  },
	  {
		"name": "collateralWei",
		"type": "uint256",
		"indexed": false,
		"internalType": "uint256"
	  },
	  {
		"name": "owner",
		"type": "address",
		"indexed": false,
		"internalType": "address"
	  }
	],
	"anonymous": false
  },
  {
	"type": "event",
	"name": "OperatorSlashed",
	"inputs": [
	  {
		"name": "slashingType",
		"type": "uint8",
		"indexed": false,
		"internalType": "enum IRegistry.SlashingType"
	  },
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"indexed": true,
		"internalType": "bytes32"
	  },
	  {
		"name": "owner",
		"type": "address",
		"indexed": false,
		"internalType": "address"
	  },
	  {
		"name": "challenger",
		"type": "address",
		"indexed": false,
		"internalType": "address"
	  },
	  {
		"name": "slasher",
		"type": "address",
		"indexed": true,
		"internalType": "address"
	  },
	  {
		"name": "slashAmountWei",
		"type": "uint256",
		"indexed": false,
		"internalType": "uint256"
	  }
	],
	"anonymous": false
  },
  {
	"type": "event",
	"name": "OperatorUnregistered",
	"inputs": [
	  {
		"name": "registrationRoot",
		"type": "bytes32",
		"indexed": true,
		"internalType": "bytes32"
	  }
	],
	"anonymous": false
  },
  {
	"type": "error",
	"name": "AlreadyOptedIn",
	"inputs": []
  },
  {
	"type": "error",
	"name": "AlreadyUnregistered",
	"inputs": []
  },
  {
	"type": "error",
	"name": "CollateralBelowMinimum",
	"inputs": []
  },
  {
	"type": "error",
	"name": "CollateralOverflow",
	"inputs": []
  },
  {
	"type": "error",
	"name": "DelegationSignatureInvalid",
	"inputs": []
  },
  {
	"type": "error",
	"name": "DelegationsAreSame",
	"inputs": []
  },
  {
	"type": "error",
	"name": "DifferentSlots",
	"inputs": []
  },
  {
	"type": "error",
	"name": "EthTransferFailed",
	"inputs": []
  },
  {
	"type": "error",
	"name": "FraudProofChallengeInvalid",
	"inputs": []
  },
  {
	"type": "error",
	"name": "FraudProofMerklePathInvalid",
	"inputs": []
  },
  {
	"type": "error",
	"name": "FraudProofWindowExpired",
	"inputs": []
  },
  {
	"type": "error",
	"name": "FraudProofWindowNotMet",
	"inputs": []
  },
  {
	"type": "error",
	"name": "InsufficientCollateral",
	"inputs": []
  },
  {
	"type": "error",
	"name": "InvalidDelegation",
	"inputs": []
  },
  {
	"type": "error",
	"name": "InvalidOwnerAddress",
	"inputs": []
  },
  {
	"type": "error",
	"name": "InvalidProof",
	"inputs": []
  },
  {
	"type": "error",
	"name": "InvalidRegistrationRoot",
	"inputs": []
  },
  {
	"type": "error",
	"name": "NoCollateral",
	"inputs": []
  },
  {
	"type": "error",
	"name": "NoCollateralSlashed",
	"inputs": []
  },
  {
	"type": "error",
	"name": "NoCollateralToClaim",
	"inputs": []
  },
  {
	"type": "error",
	"name": "NotOptedIn",
	"inputs": []
  },
  {
	"type": "error",
	"name": "NotRegisteredKey",
	"inputs": []
  },
  {
	"type": "error",
	"name": "NotSlashed",
	"inputs": []
  },
  {
	"type": "error",
	"name": "NotUnregistered",
	"inputs": []
  },
  {
	"type": "error",
	"name": "OperatorAlreadyEquivocated",
	"inputs": []
  },
  {
	"type": "error",
	"name": "OperatorAlreadyRegistered",
	"inputs": []
  },
  {
	"type": "error",
	"name": "OperatorAlreadyUnregistered",
	"inputs": []
  },
  {
	"type": "error",
	"name": "OperatorDeleted",
	"inputs": []
  },
  {
	"type": "error",
	"name": "OptInDelayNotMet",
	"inputs": []
  },
  {
	"type": "error",
	"name": "SlashAmountExceedsCollateral",
	"inputs": []
  },
  {
	"type": "error",
	"name": "SlashWindowExpired",
	"inputs": []
  },
  {
	"type": "error",
	"name": "SlashWindowNotMet",
	"inputs": []
  },
  {
	"type": "error",
	"name": "SlashingAlreadyOccurred",
	"inputs": []
  },
  {
	"type": "error",
	"name": "TimestampTooOld",
	"inputs": []
  },
  {
	"type": "error",
	"name": "UnauthorizedCommitment",
	"inputs": []
  },
  {
	"type": "error",
	"name": "UnregistrationDelayNotMet",
	"inputs": []
  },
  {
	"type": "error",
	"name": "WrongOperator",
	"inputs": []
  }
]
```*/
#[allow(
	non_camel_case_types,
	non_snake_case,
	clippy::pub_underscore_fields,
	clippy::style,
	clippy::empty_structs_with_brackets
)]
pub mod Registry {
	use super::*;
	use alloy::sol_types as alloy_sol_types;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `AlreadyOptedIn()` and selector `0xdcdeaba3`.
	```solidity
	error AlreadyOptedIn();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct AlreadyOptedIn;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<AlreadyOptedIn> for UnderlyingRustTuple<'_> {
			fn from(value: AlreadyOptedIn) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyOptedIn {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for AlreadyOptedIn {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "AlreadyOptedIn()";
			const SELECTOR: [u8; 4] = [220u8, 222u8, 171u8, 163u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `AlreadyUnregistered()` and selector `0x58566f5f`.
	```solidity
	error AlreadyUnregistered();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct AlreadyUnregistered;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<AlreadyUnregistered> for UnderlyingRustTuple<'_> {
			fn from(value: AlreadyUnregistered) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyUnregistered {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for AlreadyUnregistered {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "AlreadyUnregistered()";
			const SELECTOR: [u8; 4] = [88u8, 86u8, 111u8, 95u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `CollateralBelowMinimum()` and selector `0x735608ca`.
	```solidity
	error CollateralBelowMinimum();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct CollateralBelowMinimum;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<CollateralBelowMinimum> for UnderlyingRustTuple<'_> {
			fn from(value: CollateralBelowMinimum) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for CollateralBelowMinimum {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for CollateralBelowMinimum {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "CollateralBelowMinimum()";
			const SELECTOR: [u8; 4] = [115u8, 86u8, 8u8, 202u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `CollateralOverflow()` and selector `0x450d49cd`.
	```solidity
	error CollateralOverflow();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct CollateralOverflow;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<CollateralOverflow> for UnderlyingRustTuple<'_> {
			fn from(value: CollateralOverflow) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for CollateralOverflow {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for CollateralOverflow {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "CollateralOverflow()";
			const SELECTOR: [u8; 4] = [69u8, 13u8, 73u8, 205u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `DelegationSignatureInvalid()` and selector `0x7cbfed2d`.
	```solidity
	error DelegationSignatureInvalid();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct DelegationSignatureInvalid;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<DelegationSignatureInvalid> for UnderlyingRustTuple<'_> {
			fn from(value: DelegationSignatureInvalid) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelegationSignatureInvalid {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for DelegationSignatureInvalid {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "DelegationSignatureInvalid()";
			const SELECTOR: [u8; 4] = [124u8, 191u8, 237u8, 45u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `DelegationsAreSame()` and selector `0x58a155f0`.
	```solidity
	error DelegationsAreSame();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct DelegationsAreSame;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<DelegationsAreSame> for UnderlyingRustTuple<'_> {
			fn from(value: DelegationsAreSame) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelegationsAreSame {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for DelegationsAreSame {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "DelegationsAreSame()";
			const SELECTOR: [u8; 4] = [88u8, 161u8, 85u8, 240u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `DifferentSlots()` and selector `0x23d509e6`.
	```solidity
	error DifferentSlots();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct DifferentSlots;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<DifferentSlots> for UnderlyingRustTuple<'_> {
			fn from(value: DifferentSlots) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for DifferentSlots {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for DifferentSlots {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "DifferentSlots()";
			const SELECTOR: [u8; 4] = [35u8, 213u8, 9u8, 230u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `EthTransferFailed()` and selector `0x6d963f88`.
	```solidity
	error EthTransferFailed();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct EthTransferFailed;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<EthTransferFailed> for UnderlyingRustTuple<'_> {
			fn from(value: EthTransferFailed) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for EthTransferFailed {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for EthTransferFailed {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "EthTransferFailed()";
			const SELECTOR: [u8; 4] = [109u8, 150u8, 63u8, 136u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `FraudProofChallengeInvalid()` and selector `0xc653e87f`.
	```solidity
	error FraudProofChallengeInvalid();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct FraudProofChallengeInvalid;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<FraudProofChallengeInvalid> for UnderlyingRustTuple<'_> {
			fn from(value: FraudProofChallengeInvalid) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for FraudProofChallengeInvalid {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for FraudProofChallengeInvalid {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "FraudProofChallengeInvalid()";
			const SELECTOR: [u8; 4] = [198u8, 83u8, 232u8, 127u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `FraudProofMerklePathInvalid()` and selector `0x89298a80`.
	```solidity
	error FraudProofMerklePathInvalid();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct FraudProofMerklePathInvalid;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<FraudProofMerklePathInvalid> for UnderlyingRustTuple<'_> {
			fn from(value: FraudProofMerklePathInvalid) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for FraudProofMerklePathInvalid {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for FraudProofMerklePathInvalid {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "FraudProofMerklePathInvalid()";
			const SELECTOR: [u8; 4] = [137u8, 41u8, 138u8, 128u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `FraudProofWindowExpired()` and selector `0x57d143a4`.
	```solidity
	error FraudProofWindowExpired();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct FraudProofWindowExpired;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<FraudProofWindowExpired> for UnderlyingRustTuple<'_> {
			fn from(value: FraudProofWindowExpired) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for FraudProofWindowExpired {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for FraudProofWindowExpired {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "FraudProofWindowExpired()";
			const SELECTOR: [u8; 4] = [87u8, 209u8, 67u8, 164u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `FraudProofWindowNotMet()` and selector `0xd8a71421`.
	```solidity
	error FraudProofWindowNotMet();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct FraudProofWindowNotMet;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<FraudProofWindowNotMet> for UnderlyingRustTuple<'_> {
			fn from(value: FraudProofWindowNotMet) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for FraudProofWindowNotMet {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for FraudProofWindowNotMet {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "FraudProofWindowNotMet()";
			const SELECTOR: [u8; 4] = [216u8, 167u8, 20u8, 33u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `InsufficientCollateral()` and selector `0x3a23d825`.
	```solidity
	error InsufficientCollateral();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct InsufficientCollateral;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<InsufficientCollateral> for UnderlyingRustTuple<'_> {
			fn from(value: InsufficientCollateral) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientCollateral {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for InsufficientCollateral {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "InsufficientCollateral()";
			const SELECTOR: [u8; 4] = [58u8, 35u8, 216u8, 37u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `InvalidDelegation()` and selector `0xa9e649e9`.
	```solidity
	error InvalidDelegation();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct InvalidDelegation;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<InvalidDelegation> for UnderlyingRustTuple<'_> {
			fn from(value: InvalidDelegation) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidDelegation {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for InvalidDelegation {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "InvalidDelegation()";
			const SELECTOR: [u8; 4] = [169u8, 230u8, 73u8, 233u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `InvalidOwnerAddress()` and selector `0xd924e5f4`.
	```solidity
	error InvalidOwnerAddress();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct InvalidOwnerAddress;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<InvalidOwnerAddress> for UnderlyingRustTuple<'_> {
			fn from(value: InvalidOwnerAddress) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOwnerAddress {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for InvalidOwnerAddress {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "InvalidOwnerAddress()";
			const SELECTOR: [u8; 4] = [217u8, 36u8, 229u8, 244u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `InvalidProof()` and selector `0x09bde339`.
	```solidity
	error InvalidProof();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct InvalidProof;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<InvalidProof> for UnderlyingRustTuple<'_> {
			fn from(value: InvalidProof) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidProof {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for InvalidProof {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "InvalidProof()";
			const SELECTOR: [u8; 4] = [9u8, 189u8, 227u8, 57u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `InvalidRegistrationRoot()` and selector `0xb370a231`.
	```solidity
	error InvalidRegistrationRoot();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct InvalidRegistrationRoot;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<InvalidRegistrationRoot> for UnderlyingRustTuple<'_> {
			fn from(value: InvalidRegistrationRoot) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRegistrationRoot {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for InvalidRegistrationRoot {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "InvalidRegistrationRoot()";
			const SELECTOR: [u8; 4] = [179u8, 112u8, 162u8, 49u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `NoCollateral()` and selector `0x8dc8d9b3`.
	```solidity
	error NoCollateral();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct NoCollateral;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<NoCollateral> for UnderlyingRustTuple<'_> {
			fn from(value: NoCollateral) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoCollateral {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for NoCollateral {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "NoCollateral()";
			const SELECTOR: [u8; 4] = [141u8, 200u8, 217u8, 179u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `NoCollateralSlashed()` and selector `0x128bb86d`.
	```solidity
	error NoCollateralSlashed();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct NoCollateralSlashed;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<NoCollateralSlashed> for UnderlyingRustTuple<'_> {
			fn from(value: NoCollateralSlashed) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoCollateralSlashed {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for NoCollateralSlashed {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "NoCollateralSlashed()";
			const SELECTOR: [u8; 4] = [18u8, 139u8, 184u8, 109u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `NoCollateralToClaim()` and selector `0xfc9f68a9`.
	```solidity
	error NoCollateralToClaim();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct NoCollateralToClaim;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<NoCollateralToClaim> for UnderlyingRustTuple<'_> {
			fn from(value: NoCollateralToClaim) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoCollateralToClaim {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for NoCollateralToClaim {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "NoCollateralToClaim()";
			const SELECTOR: [u8; 4] = [252u8, 159u8, 104u8, 169u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `NotOptedIn()` and selector `0x69613672`.
	```solidity
	error NotOptedIn();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct NotOptedIn;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<NotOptedIn> for UnderlyingRustTuple<'_> {
			fn from(value: NotOptedIn) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotOptedIn {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for NotOptedIn {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "NotOptedIn()";
			const SELECTOR: [u8; 4] = [105u8, 97u8, 54u8, 114u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `NotRegisteredKey()` and selector `0xa43fbeb8`.
	```solidity
	error NotRegisteredKey();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct NotRegisteredKey;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<NotRegisteredKey> for UnderlyingRustTuple<'_> {
			fn from(value: NotRegisteredKey) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotRegisteredKey {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for NotRegisteredKey {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "NotRegisteredKey()";
			const SELECTOR: [u8; 4] = [164u8, 63u8, 190u8, 184u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `NotSlashed()` and selector `0xb412abe4`.
	```solidity
	error NotSlashed();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct NotSlashed;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<NotSlashed> for UnderlyingRustTuple<'_> {
			fn from(value: NotSlashed) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotSlashed {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for NotSlashed {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "NotSlashed()";
			const SELECTOR: [u8; 4] = [180u8, 18u8, 171u8, 228u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `NotUnregistered()` and selector `0x591439b0`.
	```solidity
	error NotUnregistered();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct NotUnregistered;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<NotUnregistered> for UnderlyingRustTuple<'_> {
			fn from(value: NotUnregistered) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotUnregistered {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for NotUnregistered {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "NotUnregistered()";
			const SELECTOR: [u8; 4] = [89u8, 20u8, 57u8, 176u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `OperatorAlreadyEquivocated()` and selector `0x855d0488`.
	```solidity
	error OperatorAlreadyEquivocated();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct OperatorAlreadyEquivocated;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<OperatorAlreadyEquivocated> for UnderlyingRustTuple<'_> {
			fn from(value: OperatorAlreadyEquivocated) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorAlreadyEquivocated {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for OperatorAlreadyEquivocated {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "OperatorAlreadyEquivocated()";
			const SELECTOR: [u8; 4] = [133u8, 93u8, 4u8, 136u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `OperatorAlreadyRegistered()` and selector `0x42ee68b5`.
	```solidity
	error OperatorAlreadyRegistered();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct OperatorAlreadyRegistered;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<OperatorAlreadyRegistered> for UnderlyingRustTuple<'_> {
			fn from(value: OperatorAlreadyRegistered) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorAlreadyRegistered {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for OperatorAlreadyRegistered {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "OperatorAlreadyRegistered()";
			const SELECTOR: [u8; 4] = [66u8, 238u8, 104u8, 181u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `OperatorAlreadyUnregistered()` and selector `0x4322e6e2`.
	```solidity
	error OperatorAlreadyUnregistered();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct OperatorAlreadyUnregistered;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<OperatorAlreadyUnregistered> for UnderlyingRustTuple<'_> {
			fn from(value: OperatorAlreadyUnregistered) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorAlreadyUnregistered {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for OperatorAlreadyUnregistered {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "OperatorAlreadyUnregistered()";
			const SELECTOR: [u8; 4] = [67u8, 34u8, 230u8, 226u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `OperatorDeleted()` and selector `0x0f861bc0`.
	```solidity
	error OperatorDeleted();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct OperatorDeleted;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<OperatorDeleted> for UnderlyingRustTuple<'_> {
			fn from(value: OperatorDeleted) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorDeleted {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for OperatorDeleted {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "OperatorDeleted()";
			const SELECTOR: [u8; 4] = [15u8, 134u8, 27u8, 192u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `OptInDelayNotMet()` and selector `0xe5bc446d`.
	```solidity
	error OptInDelayNotMet();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct OptInDelayNotMet;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<OptInDelayNotMet> for UnderlyingRustTuple<'_> {
			fn from(value: OptInDelayNotMet) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for OptInDelayNotMet {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for OptInDelayNotMet {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "OptInDelayNotMet()";
			const SELECTOR: [u8; 4] = [229u8, 188u8, 68u8, 109u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `SlashAmountExceedsCollateral()` and selector `0x0af1662e`.
	```solidity
	error SlashAmountExceedsCollateral();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SlashAmountExceedsCollateral;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<SlashAmountExceedsCollateral> for UnderlyingRustTuple<'_> {
			fn from(value: SlashAmountExceedsCollateral) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for SlashAmountExceedsCollateral {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for SlashAmountExceedsCollateral {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "SlashAmountExceedsCollateral()";
			const SELECTOR: [u8; 4] = [10u8, 241u8, 102u8, 46u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `SlashWindowExpired()` and selector `0xeb401d82`.
	```solidity
	error SlashWindowExpired();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SlashWindowExpired;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<SlashWindowExpired> for UnderlyingRustTuple<'_> {
			fn from(value: SlashWindowExpired) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for SlashWindowExpired {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for SlashWindowExpired {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "SlashWindowExpired()";
			const SELECTOR: [u8; 4] = [235u8, 64u8, 29u8, 130u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `SlashWindowNotMet()` and selector `0x45d4c5a2`.
	```solidity
	error SlashWindowNotMet();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SlashWindowNotMet;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<SlashWindowNotMet> for UnderlyingRustTuple<'_> {
			fn from(value: SlashWindowNotMet) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for SlashWindowNotMet {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for SlashWindowNotMet {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "SlashWindowNotMet()";
			const SELECTOR: [u8; 4] = [69u8, 212u8, 197u8, 162u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `SlashingAlreadyOccurred()` and selector `0x24701b17`.
	```solidity
	error SlashingAlreadyOccurred();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct SlashingAlreadyOccurred;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<SlashingAlreadyOccurred> for UnderlyingRustTuple<'_> {
			fn from(value: SlashingAlreadyOccurred) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for SlashingAlreadyOccurred {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for SlashingAlreadyOccurred {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "SlashingAlreadyOccurred()";
			const SELECTOR: [u8; 4] = [36u8, 112u8, 27u8, 23u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `TimestampTooOld()` and selector `0xd40fc74b`.
	```solidity
	error TimestampTooOld();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct TimestampTooOld;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<TimestampTooOld> for UnderlyingRustTuple<'_> {
			fn from(value: TimestampTooOld) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for TimestampTooOld {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for TimestampTooOld {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "TimestampTooOld()";
			const SELECTOR: [u8; 4] = [212u8, 15u8, 199u8, 75u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `UnauthorizedCommitment()` and selector `0xb4a582c3`.
	```solidity
	error UnauthorizedCommitment();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct UnauthorizedCommitment;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnauthorizedCommitment> for UnderlyingRustTuple<'_> {
			fn from(value: UnauthorizedCommitment) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnauthorizedCommitment {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for UnauthorizedCommitment {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "UnauthorizedCommitment()";
			const SELECTOR: [u8; 4] = [180u8, 165u8, 130u8, 195u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `UnregistrationDelayNotMet()` and selector `0xfb0f2444`.
	```solidity
	error UnregistrationDelayNotMet();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct UnregistrationDelayNotMet;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnregistrationDelayNotMet> for UnderlyingRustTuple<'_> {
			fn from(value: UnregistrationDelayNotMet) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnregistrationDelayNotMet {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for UnregistrationDelayNotMet {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "UnregistrationDelayNotMet()";
			const SELECTOR: [u8; 4] = [251u8, 15u8, 36u8, 68u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Custom error with signature `WrongOperator()` and selector `0x2b3b1add`.
	```solidity
	error WrongOperator();
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct WrongOperator;
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[doc(hidden)]
		#[allow(dead_code)]
		type UnderlyingSolTuple<'a> = ();
		#[doc(hidden)]
		type UnderlyingRustTuple<'a> = ();
		#[cfg(test)]
		#[allow(dead_code, unreachable_patterns)]
		fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
			match _t {
				alloy_sol_types::private::AssertTypeEq::<<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType>(
					_,
				) => {}
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<WrongOperator> for UnderlyingRustTuple<'_> {
			fn from(value: WrongOperator) -> Self {
				()
			}
		}
		#[automatically_derived]
		#[doc(hidden)]
		impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongOperator {
			fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
				Self
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolError for WrongOperator {
			type Parameters<'a> = UnderlyingSolTuple<'a>;
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "WrongOperator()";
			const SELECTOR: [u8; 4] = [43u8, 59u8, 26u8, 221u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
				<Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Self::new)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Event with signature `CollateralAdded(bytes32,uint256)` and selector `0xe33e944bf296c89ccab816113b3154f8a538589d4be56b368938edc303388d20`.
	```solidity
	event CollateralAdded(bytes32 indexed registrationRoot, uint256 collateralWei);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	#[derive(Clone)]
	pub struct CollateralAdded {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub collateralWei: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[automatically_derived]
		impl alloy_sol_types::SolEvent for CollateralAdded {
			type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::FixedBytes<32>);
			const SIGNATURE: &'static str = "CollateralAdded(bytes32,uint256)";
			const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
				227u8, 62u8, 148u8, 75u8, 242u8, 150u8, 200u8, 156u8, 202u8, 184u8, 22u8, 17u8, 59u8, 49u8, 84u8,
				248u8, 165u8, 56u8, 88u8, 157u8, 75u8, 229u8, 107u8, 54u8, 137u8, 56u8, 237u8, 195u8, 3u8, 56u8, 141u8,
				32u8,
			]);
			const ANONYMOUS: bool = false;
			#[allow(unused_variables)]
			#[inline]
			fn new(
				topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
				data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
			) -> Self {
				Self { registrationRoot: topics.1, collateralWei: data.0 }
			}
			#[inline]
			fn check_signature(
				topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
			) -> alloy_sol_types::Result<()> {
				if topics.0 != Self::SIGNATURE_HASH {
					return Err(alloy_sol_types::Error::invalid_event_signature_hash(
						Self::SIGNATURE,
						topics.0,
						Self::SIGNATURE_HASH,
					));
				}
				Ok(())
			}
			#[inline]
			fn tokenize_body(&self) -> Self::DataToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self.collateralWei),)
			}
			#[inline]
			fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
				(Self::SIGNATURE_HASH.into(), self.registrationRoot.clone())
			}
			#[inline]
			fn encode_topics_raw(
				&self,
				out: &mut [alloy_sol_types::abi::token::WordToken],
			) -> alloy_sol_types::Result<()> {
				if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
					return Err(alloy_sol_types::Error::Overrun);
				}
				out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
				out[1usize] = <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic(
					&self.registrationRoot,
				);
				Ok(())
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::private::IntoLogData for CollateralAdded {
			fn to_log_data(&self) -> alloy_sol_types::private::LogData {
				From::from(self)
			}
			fn into_log_data(self) -> alloy_sol_types::private::LogData {
				From::from(&self)
			}
		}
		#[automatically_derived]
		impl From<&CollateralAdded> for alloy_sol_types::private::LogData {
			#[inline]
			fn from(this: &CollateralAdded) -> alloy_sol_types::private::LogData {
				alloy_sol_types::SolEvent::encode_log_data(this)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Event with signature `CollateralClaimed(bytes32,uint256)` and selector `0x85eddc2a5759b2da1884eb40cdab3b3cac7da2c3c7c24851351f682e94e99c2a`.
	```solidity
	event CollateralClaimed(bytes32 indexed registrationRoot, uint256 collateralWei);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	#[derive(Clone)]
	pub struct CollateralClaimed {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub collateralWei: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[automatically_derived]
		impl alloy_sol_types::SolEvent for CollateralClaimed {
			type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::FixedBytes<32>);
			const SIGNATURE: &'static str = "CollateralClaimed(bytes32,uint256)";
			const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
				133u8, 237u8, 220u8, 42u8, 87u8, 89u8, 178u8, 218u8, 24u8, 132u8, 235u8, 64u8, 205u8, 171u8, 59u8,
				60u8, 172u8, 125u8, 162u8, 195u8, 199u8, 194u8, 72u8, 81u8, 53u8, 31u8, 104u8, 46u8, 148u8, 233u8,
				156u8, 42u8,
			]);
			const ANONYMOUS: bool = false;
			#[allow(unused_variables)]
			#[inline]
			fn new(
				topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
				data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
			) -> Self {
				Self { registrationRoot: topics.1, collateralWei: data.0 }
			}
			#[inline]
			fn check_signature(
				topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
			) -> alloy_sol_types::Result<()> {
				if topics.0 != Self::SIGNATURE_HASH {
					return Err(alloy_sol_types::Error::invalid_event_signature_hash(
						Self::SIGNATURE,
						topics.0,
						Self::SIGNATURE_HASH,
					));
				}
				Ok(())
			}
			#[inline]
			fn tokenize_body(&self) -> Self::DataToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self.collateralWei),)
			}
			#[inline]
			fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
				(Self::SIGNATURE_HASH.into(), self.registrationRoot.clone())
			}
			#[inline]
			fn encode_topics_raw(
				&self,
				out: &mut [alloy_sol_types::abi::token::WordToken],
			) -> alloy_sol_types::Result<()> {
				if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
					return Err(alloy_sol_types::Error::Overrun);
				}
				out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
				out[1usize] = <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic(
					&self.registrationRoot,
				);
				Ok(())
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::private::IntoLogData for CollateralClaimed {
			fn to_log_data(&self) -> alloy_sol_types::private::LogData {
				From::from(self)
			}
			fn into_log_data(self) -> alloy_sol_types::private::LogData {
				From::from(&self)
			}
		}
		#[automatically_derived]
		impl From<&CollateralClaimed> for alloy_sol_types::private::LogData {
			#[inline]
			fn from(this: &CollateralClaimed) -> alloy_sol_types::private::LogData {
				alloy_sol_types::SolEvent::encode_log_data(this)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Event with signature `OperatorOptedIn(bytes32,address,address)` and selector `0xb847759c24329f86c9026fadbccbbeb9241f535a709a53fe95eccc606a79d300`.
	```solidity
	event OperatorOptedIn(bytes32 indexed registrationRoot, address indexed slasher, address indexed committer);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	#[derive(Clone)]
	pub struct OperatorOptedIn {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub slasher: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub committer: alloy::sol_types::private::Address,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[automatically_derived]
		impl alloy_sol_types::SolEvent for OperatorOptedIn {
			type DataTuple<'a> = ();
			type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			type TopicList = (
				alloy_sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::Address,
				alloy::sol_types::sol_data::Address,
			);
			const SIGNATURE: &'static str = "OperatorOptedIn(bytes32,address,address)";
			const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
				184u8, 71u8, 117u8, 156u8, 36u8, 50u8, 159u8, 134u8, 201u8, 2u8, 111u8, 173u8, 188u8, 203u8, 190u8,
				185u8, 36u8, 31u8, 83u8, 90u8, 112u8, 154u8, 83u8, 254u8, 149u8, 236u8, 204u8, 96u8, 106u8, 121u8,
				211u8, 0u8,
			]);
			const ANONYMOUS: bool = false;
			#[allow(unused_variables)]
			#[inline]
			fn new(
				topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
				data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
			) -> Self {
				Self { registrationRoot: topics.1, slasher: topics.2, committer: topics.3 }
			}
			#[inline]
			fn check_signature(
				topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
			) -> alloy_sol_types::Result<()> {
				if topics.0 != Self::SIGNATURE_HASH {
					return Err(alloy_sol_types::Error::invalid_event_signature_hash(
						Self::SIGNATURE,
						topics.0,
						Self::SIGNATURE_HASH,
					));
				}
				Ok(())
			}
			#[inline]
			fn tokenize_body(&self) -> Self::DataToken<'_> {
				()
			}
			#[inline]
			fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
				(
					Self::SIGNATURE_HASH.into(),
					self.registrationRoot.clone(),
					self.slasher.clone(),
					self.committer.clone(),
				)
			}
			#[inline]
			fn encode_topics_raw(
				&self,
				out: &mut [alloy_sol_types::abi::token::WordToken],
			) -> alloy_sol_types::Result<()> {
				if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
					return Err(alloy_sol_types::Error::Overrun);
				}
				out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
				out[1usize] = <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic(
					&self.registrationRoot,
				);
				out[2usize] =
					<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(&self.slasher);
				out[3usize] =
					<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(&self.committer);
				Ok(())
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::private::IntoLogData for OperatorOptedIn {
			fn to_log_data(&self) -> alloy_sol_types::private::LogData {
				From::from(self)
			}
			fn into_log_data(self) -> alloy_sol_types::private::LogData {
				From::from(&self)
			}
		}
		#[automatically_derived]
		impl From<&OperatorOptedIn> for alloy_sol_types::private::LogData {
			#[inline]
			fn from(this: &OperatorOptedIn) -> alloy_sol_types::private::LogData {
				alloy_sol_types::SolEvent::encode_log_data(this)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Event with signature `OperatorOptedOut(bytes32,address)` and selector `0xcc541876f5cb916c709a3e40d2f1519ba6b1e89f3c98f0799545cf346c1c323c`.
	```solidity
	event OperatorOptedOut(bytes32 indexed registrationRoot, address indexed slasher);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	#[derive(Clone)]
	pub struct OperatorOptedOut {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub slasher: alloy::sol_types::private::Address,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[automatically_derived]
		impl alloy_sol_types::SolEvent for OperatorOptedOut {
			type DataTuple<'a> = ();
			type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			type TopicList = (
				alloy_sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::Address,
			);
			const SIGNATURE: &'static str = "OperatorOptedOut(bytes32,address)";
			const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
				204u8, 84u8, 24u8, 118u8, 245u8, 203u8, 145u8, 108u8, 112u8, 154u8, 62u8, 64u8, 210u8, 241u8, 81u8,
				155u8, 166u8, 177u8, 232u8, 159u8, 60u8, 152u8, 240u8, 121u8, 149u8, 69u8, 207u8, 52u8, 108u8, 28u8,
				50u8, 60u8,
			]);
			const ANONYMOUS: bool = false;
			#[allow(unused_variables)]
			#[inline]
			fn new(
				topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
				data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
			) -> Self {
				Self { registrationRoot: topics.1, slasher: topics.2 }
			}
			#[inline]
			fn check_signature(
				topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
			) -> alloy_sol_types::Result<()> {
				if topics.0 != Self::SIGNATURE_HASH {
					return Err(alloy_sol_types::Error::invalid_event_signature_hash(
						Self::SIGNATURE,
						topics.0,
						Self::SIGNATURE_HASH,
					));
				}
				Ok(())
			}
			#[inline]
			fn tokenize_body(&self) -> Self::DataToken<'_> {
				()
			}
			#[inline]
			fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
				(Self::SIGNATURE_HASH.into(), self.registrationRoot.clone(), self.slasher.clone())
			}
			#[inline]
			fn encode_topics_raw(
				&self,
				out: &mut [alloy_sol_types::abi::token::WordToken],
			) -> alloy_sol_types::Result<()> {
				if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
					return Err(alloy_sol_types::Error::Overrun);
				}
				out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
				out[1usize] = <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic(
					&self.registrationRoot,
				);
				out[2usize] =
					<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(&self.slasher);
				Ok(())
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::private::IntoLogData for OperatorOptedOut {
			fn to_log_data(&self) -> alloy_sol_types::private::LogData {
				From::from(self)
			}
			fn into_log_data(self) -> alloy_sol_types::private::LogData {
				From::from(&self)
			}
		}
		#[automatically_derived]
		impl From<&OperatorOptedOut> for alloy_sol_types::private::LogData {
			#[inline]
			fn from(this: &OperatorOptedOut) -> alloy_sol_types::private::LogData {
				alloy_sol_types::SolEvent::encode_log_data(this)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Event with signature `OperatorRegistered(bytes32,uint256,address)` and selector `0xb366de698e5a1906ad7125619b5520e01bc0c6a7e1addd0089135c4345a9e68f`.
	```solidity
	event OperatorRegistered(bytes32 indexed registrationRoot, uint256 collateralWei, address owner);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	#[derive(Clone)]
	pub struct OperatorRegistered {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub collateralWei: alloy::sol_types::private::primitives::aliases::U256,
		#[allow(missing_docs)]
		pub owner: alloy::sol_types::private::Address,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[automatically_derived]
		impl alloy_sol_types::SolEvent for OperatorRegistered {
			type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Address);
			type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::FixedBytes<32>);
			const SIGNATURE: &'static str = "OperatorRegistered(bytes32,uint256,address)";
			const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
				179u8, 102u8, 222u8, 105u8, 142u8, 90u8, 25u8, 6u8, 173u8, 113u8, 37u8, 97u8, 155u8, 85u8, 32u8, 224u8,
				27u8, 192u8, 198u8, 167u8, 225u8, 173u8, 221u8, 0u8, 137u8, 19u8, 92u8, 67u8, 69u8, 169u8, 230u8,
				143u8,
			]);
			const ANONYMOUS: bool = false;
			#[allow(unused_variables)]
			#[inline]
			fn new(
				topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
				data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
			) -> Self {
				Self { registrationRoot: topics.1, collateralWei: data.0, owner: data.1 }
			}
			#[inline]
			fn check_signature(
				topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
			) -> alloy_sol_types::Result<()> {
				if topics.0 != Self::SIGNATURE_HASH {
					return Err(alloy_sol_types::Error::invalid_event_signature_hash(
						Self::SIGNATURE,
						topics.0,
						Self::SIGNATURE_HASH,
					));
				}
				Ok(())
			}
			#[inline]
			fn tokenize_body(&self) -> Self::DataToken<'_> {
				(
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self.collateralWei),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.owner),
				)
			}
			#[inline]
			fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
				(Self::SIGNATURE_HASH.into(), self.registrationRoot.clone())
			}
			#[inline]
			fn encode_topics_raw(
				&self,
				out: &mut [alloy_sol_types::abi::token::WordToken],
			) -> alloy_sol_types::Result<()> {
				if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
					return Err(alloy_sol_types::Error::Overrun);
				}
				out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
				out[1usize] = <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic(
					&self.registrationRoot,
				);
				Ok(())
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::private::IntoLogData for OperatorRegistered {
			fn to_log_data(&self) -> alloy_sol_types::private::LogData {
				From::from(self)
			}
			fn into_log_data(self) -> alloy_sol_types::private::LogData {
				From::from(&self)
			}
		}
		#[automatically_derived]
		impl From<&OperatorRegistered> for alloy_sol_types::private::LogData {
			#[inline]
			fn from(this: &OperatorRegistered) -> alloy_sol_types::private::LogData {
				alloy_sol_types::SolEvent::encode_log_data(this)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Event with signature `OperatorSlashed(uint8,bytes32,address,address,address,uint256)` and selector `0x90ab53caef3edb1139ab395dd286a010c6e4bc7a825946901c78300c7b3aabcf`.
	```solidity
	event OperatorSlashed(IRegistry.SlashingType slashingType, bytes32 indexed registrationRoot, address owner, address challenger, address indexed slasher, uint256 slashAmountWei);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	#[derive(Clone)]
	pub struct OperatorSlashed {
		#[allow(missing_docs)]
		pub slashingType: <IRegistry::SlashingType as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub owner: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub challenger: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub slasher: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[automatically_derived]
		impl alloy_sol_types::SolEvent for OperatorSlashed {
			type DataTuple<'a> = (
				IRegistry::SlashingType,
				alloy::sol_types::sol_data::Address,
				alloy::sol_types::sol_data::Address,
				alloy::sol_types::sol_data::Uint<256>,
			);
			type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			type TopicList = (
				alloy_sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::Address,
			);
			const SIGNATURE: &'static str = "OperatorSlashed(uint8,bytes32,address,address,address,uint256)";
			const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
				144u8, 171u8, 83u8, 202u8, 239u8, 62u8, 219u8, 17u8, 57u8, 171u8, 57u8, 93u8, 210u8, 134u8, 160u8,
				16u8, 198u8, 228u8, 188u8, 122u8, 130u8, 89u8, 70u8, 144u8, 28u8, 120u8, 48u8, 12u8, 123u8, 58u8,
				171u8, 207u8,
			]);
			const ANONYMOUS: bool = false;
			#[allow(unused_variables)]
			#[inline]
			fn new(
				topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
				data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
			) -> Self {
				Self {
					slashingType: data.0,
					registrationRoot: topics.1,
					owner: data.1,
					challenger: data.2,
					slasher: topics.2,
					slashAmountWei: data.3,
				}
			}
			#[inline]
			fn check_signature(
				topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
			) -> alloy_sol_types::Result<()> {
				if topics.0 != Self::SIGNATURE_HASH {
					return Err(alloy_sol_types::Error::invalid_event_signature_hash(
						Self::SIGNATURE,
						topics.0,
						Self::SIGNATURE_HASH,
					));
				}
				Ok(())
			}
			#[inline]
			fn tokenize_body(&self) -> Self::DataToken<'_> {
				(
					<IRegistry::SlashingType as alloy_sol_types::SolType>::tokenize(&self.slashingType),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.owner),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.challenger),
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self.slashAmountWei),
				)
			}
			#[inline]
			fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
				(Self::SIGNATURE_HASH.into(), self.registrationRoot.clone(), self.slasher.clone())
			}
			#[inline]
			fn encode_topics_raw(
				&self,
				out: &mut [alloy_sol_types::abi::token::WordToken],
			) -> alloy_sol_types::Result<()> {
				if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
					return Err(alloy_sol_types::Error::Overrun);
				}
				out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
				out[1usize] = <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic(
					&self.registrationRoot,
				);
				out[2usize] =
					<alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(&self.slasher);
				Ok(())
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::private::IntoLogData for OperatorSlashed {
			fn to_log_data(&self) -> alloy_sol_types::private::LogData {
				From::from(self)
			}
			fn into_log_data(self) -> alloy_sol_types::private::LogData {
				From::from(&self)
			}
		}
		#[automatically_derived]
		impl From<&OperatorSlashed> for alloy_sol_types::private::LogData {
			#[inline]
			fn from(this: &OperatorSlashed) -> alloy_sol_types::private::LogData {
				alloy_sol_types::SolEvent::encode_log_data(this)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Event with signature `OperatorUnregistered(bytes32)` and selector `0x27d7dba9e6a8c02dcbb9b73d83573b3db86e98ee5cfe93e8882259c0b0f7fa07`.
	```solidity
	event OperatorUnregistered(bytes32 indexed registrationRoot);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	#[derive(Clone)]
	pub struct OperatorUnregistered {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		#[automatically_derived]
		impl alloy_sol_types::SolEvent for OperatorUnregistered {
			type DataTuple<'a> = ();
			type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::FixedBytes<32>);
			const SIGNATURE: &'static str = "OperatorUnregistered(bytes32)";
			const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
				39u8, 215u8, 219u8, 169u8, 230u8, 168u8, 192u8, 45u8, 203u8, 185u8, 183u8, 61u8, 131u8, 87u8, 59u8,
				61u8, 184u8, 110u8, 152u8, 238u8, 92u8, 254u8, 147u8, 232u8, 136u8, 34u8, 89u8, 192u8, 176u8, 247u8,
				250u8, 7u8,
			]);
			const ANONYMOUS: bool = false;
			#[allow(unused_variables)]
			#[inline]
			fn new(
				topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
				data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
			) -> Self {
				Self { registrationRoot: topics.1 }
			}
			#[inline]
			fn check_signature(
				topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
			) -> alloy_sol_types::Result<()> {
				if topics.0 != Self::SIGNATURE_HASH {
					return Err(alloy_sol_types::Error::invalid_event_signature_hash(
						Self::SIGNATURE,
						topics.0,
						Self::SIGNATURE_HASH,
					));
				}
				Ok(())
			}
			#[inline]
			fn tokenize_body(&self) -> Self::DataToken<'_> {
				()
			}
			#[inline]
			fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
				(Self::SIGNATURE_HASH.into(), self.registrationRoot.clone())
			}
			#[inline]
			fn encode_topics_raw(
				&self,
				out: &mut [alloy_sol_types::abi::token::WordToken],
			) -> alloy_sol_types::Result<()> {
				if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
					return Err(alloy_sol_types::Error::Overrun);
				}
				out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
				out[1usize] = <alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::EventTopic>::encode_topic(
					&self.registrationRoot,
				);
				Ok(())
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::private::IntoLogData for OperatorUnregistered {
			fn to_log_data(&self) -> alloy_sol_types::private::LogData {
				From::from(self)
			}
			fn into_log_data(self) -> alloy_sol_types::private::LogData {
				From::from(&self)
			}
		}
		#[automatically_derived]
		impl From<&OperatorUnregistered> for alloy_sol_types::private::LogData {
			#[inline]
			fn from(this: &OperatorUnregistered) -> alloy_sol_types::private::LogData {
				alloy_sol_types::SolEvent::encode_log_data(this)
			}
		}
	};
	/**Constructor`.
	```solidity
	constructor(IRegistry.Config _config);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct constructorCall {
		#[allow(missing_docs)]
		pub _config: <IRegistry::Config as alloy::sol_types::SolType>::RustType,
	}
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (IRegistry::Config,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (<IRegistry::Config as alloy::sol_types::SolType>::RustType,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
				fn from(value: constructorCall) -> Self {
					(value._config,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _config: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolConstructor for constructorCall {
			type Parameters<'a> = (IRegistry::Config,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<IRegistry::Config as alloy_sol_types::SolType>::tokenize(&self._config),)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `addCollateral(bytes32)` and selector `0xf23d4026`.
	```solidity
	function addCollateral(bytes32 registrationRoot) external payable;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct addCollateralCall {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
	}
	///Container type for the return parameters of the [`addCollateral(bytes32)`](addCollateralCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct addCollateralReturn {}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<addCollateralCall> for UnderlyingRustTuple<'_> {
				fn from(value: addCollateralCall) -> Self {
					(value.registrationRoot,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for addCollateralCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = ();
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = ();
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<addCollateralReturn> for UnderlyingRustTuple<'_> {
				fn from(value: addCollateralReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for addCollateralReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl addCollateralReturn {
			fn _tokenize(&self) -> <addCollateralCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for addCollateralCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = addCollateralReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "addCollateral(bytes32)";
			const SELECTOR: [u8; 4] = [242u8, 61u8, 64u8, 38u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
					&self.registrationRoot,
				),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				addCollateralReturn::_tokenize(ret)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(Into::into)
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Into::into)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `claimCollateral(bytes32)` and selector `0xe3fc028d`.
	```solidity
	function claimCollateral(bytes32 registrationRoot) external;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct claimCollateralCall {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
	}
	///Container type for the return parameters of the [`claimCollateral(bytes32)`](claimCollateralCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct claimCollateralReturn {}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<claimCollateralCall> for UnderlyingRustTuple<'_> {
				fn from(value: claimCollateralCall) -> Self {
					(value.registrationRoot,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimCollateralCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = ();
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = ();
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<claimCollateralReturn> for UnderlyingRustTuple<'_> {
				fn from(value: claimCollateralReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimCollateralReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl claimCollateralReturn {
			fn _tokenize(&self) -> <claimCollateralCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for claimCollateralCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = claimCollateralReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "claimCollateral(bytes32)";
			const SELECTOR: [u8; 4] = [227u8, 252u8, 2u8, 141u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
					&self.registrationRoot,
				),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				claimCollateralReturn::_tokenize(ret)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(Into::into)
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Into::into)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `claimSlashedCollateral(bytes32)` and selector `0x9f1e2c84`.
	```solidity
	function claimSlashedCollateral(bytes32 registrationRoot) external;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct claimSlashedCollateralCall {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
	}
	///Container type for the return parameters of the [`claimSlashedCollateral(bytes32)`](claimSlashedCollateralCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct claimSlashedCollateralReturn {}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<claimSlashedCollateralCall> for UnderlyingRustTuple<'_> {
				fn from(value: claimSlashedCollateralCall) -> Self {
					(value.registrationRoot,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimSlashedCollateralCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = ();
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = ();
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<claimSlashedCollateralReturn> for UnderlyingRustTuple<'_> {
				fn from(value: claimSlashedCollateralReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimSlashedCollateralReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl claimSlashedCollateralReturn {
			fn _tokenize(&self) -> <claimSlashedCollateralCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for claimSlashedCollateralCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = claimSlashedCollateralReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "claimSlashedCollateral(bytes32)";
			const SELECTOR: [u8; 4] = [159u8, 30u8, 44u8, 132u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
					&self.registrationRoot,
				),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				claimSlashedCollateralReturn::_tokenize(ret)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(Into::into)
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Into::into)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `getConfig()` and selector `0xc3f909d4`.
	```solidity
	function getConfig() external view returns (IRegistry.Config memory);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getConfigCall;
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`getConfig()`](getConfigCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getConfigReturn {
		#[allow(missing_docs)]
		pub _0: <IRegistry::Config as alloy::sol_types::SolType>::RustType,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = ();
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = ();
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getConfigCall> for UnderlyingRustTuple<'_> {
				fn from(value: getConfigCall) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getConfigCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (IRegistry::Config,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (<IRegistry::Config as alloy::sol_types::SolType>::RustType,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getConfigReturn> for UnderlyingRustTuple<'_> {
				fn from(value: getConfigReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getConfigReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for getConfigCall {
			type Parameters<'a> = ();
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = <IRegistry::Config as alloy::sol_types::SolType>::RustType;
			type ReturnTuple<'a> = (IRegistry::Config,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "getConfig()";
			const SELECTOR: [u8; 4] = [195u8, 249u8, 9u8, 212u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				()
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<IRegistry::Config as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: getConfigReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: getConfigReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `getHistoricalCollateral(bytes32,uint256)` and selector `0x24387bb4`.
	```solidity
	function getHistoricalCollateral(bytes32 registrationRoot, uint256 timestamp) external view returns (uint256 collateralWei);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getHistoricalCollateralCall {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub timestamp: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`getHistoricalCollateral(bytes32,uint256)`](getHistoricalCollateralCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getHistoricalCollateralReturn {
		#[allow(missing_docs)]
		pub collateralWei: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> =
				(alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Uint<256>);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> =
				(alloy::sol_types::private::FixedBytes<32>, alloy::sol_types::private::primitives::aliases::U256);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getHistoricalCollateralCall> for UnderlyingRustTuple<'_> {
				fn from(value: getHistoricalCollateralCall) -> Self {
					(value.registrationRoot, value.timestamp)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHistoricalCollateralCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0, timestamp: tuple.1 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getHistoricalCollateralReturn> for UnderlyingRustTuple<'_> {
				fn from(value: getHistoricalCollateralReturn) -> Self {
					(value.collateralWei,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHistoricalCollateralReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { collateralWei: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for getHistoricalCollateralCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Uint<256>);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "getHistoricalCollateral(bytes32,uint256)";
			const SELECTOR: [u8; 4] = [36u8, 56u8, 123u8, 180u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.registrationRoot,
					),
					<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(&self.timestamp),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: getHistoricalCollateralReturn = r.into();
					r.collateralWei
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: getHistoricalCollateralReturn = r.into();
					r.collateralWei
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `getOperatorData(bytes32)` and selector `0x936449ec`.
	```solidity
	function getOperatorData(bytes32 registrationRoot) external view returns (IRegistry.OperatorData memory operatorData);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getOperatorDataCall {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`getOperatorData(bytes32)`](getOperatorDataCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getOperatorDataReturn {
		#[allow(missing_docs)]
		pub operatorData: <IRegistry::OperatorData as alloy::sol_types::SolType>::RustType,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getOperatorDataCall> for UnderlyingRustTuple<'_> {
				fn from(value: getOperatorDataCall) -> Self {
					(value.registrationRoot,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorDataCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (IRegistry::OperatorData,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (<IRegistry::OperatorData as alloy::sol_types::SolType>::RustType,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getOperatorDataReturn> for UnderlyingRustTuple<'_> {
				fn from(value: getOperatorDataReturn) -> Self {
					(value.operatorData,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorDataReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { operatorData: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for getOperatorDataCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = <IRegistry::OperatorData as alloy::sol_types::SolType>::RustType;
			type ReturnTuple<'a> = (IRegistry::OperatorData,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "getOperatorData(bytes32)";
			const SELECTOR: [u8; 4] = [147u8, 100u8, 73u8, 236u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
					&self.registrationRoot,
				),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<IRegistry::OperatorData as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: getOperatorDataReturn = r.into();
					r.operatorData
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: getOperatorDataReturn = r.into();
					r.operatorData
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `getRegistrationProof(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],address,uint256,bytes32)` and selector `0x947eab92`.
	```solidity
	function getRegistrationProof(IRegistry.SignedRegistration[] memory regs, address owner, uint256 leafIndex, bytes32 signingId) external pure returns (IRegistry.RegistrationProof memory proof);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getRegistrationProofCall {
		#[allow(missing_docs)]
		pub regs:
			alloy::sol_types::private::Vec<<IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType>,
		#[allow(missing_docs)]
		pub owner: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub leafIndex: alloy::sol_types::private::primitives::aliases::U256,
		#[allow(missing_docs)]
		pub signingId: alloy::sol_types::private::FixedBytes<32>,
	}
	#[derive(serde::Serialize, serde::Deserialize)]
	///Container type for the return parameters of the [`getRegistrationProof(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],address,uint256,bytes32)`](getRegistrationProofCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getRegistrationProofReturn {
		#[allow(missing_docs)]
		pub proof: <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (
				alloy::sol_types::sol_data::Array<IRegistry::SignedRegistration>,
				alloy::sol_types::sol_data::Address,
				alloy::sol_types::sol_data::Uint<256>,
				alloy::sol_types::sol_data::FixedBytes<32>,
			);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (
				alloy::sol_types::private::Vec<<IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType>,
				alloy::sol_types::private::Address,
				alloy::sol_types::private::primitives::aliases::U256,
				alloy::sol_types::private::FixedBytes<32>,
			);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getRegistrationProofCall> for UnderlyingRustTuple<'_> {
				fn from(value: getRegistrationProofCall) -> Self {
					(value.regs, value.owner, value.leafIndex, value.signingId)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRegistrationProofCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { regs: tuple.0, owner: tuple.1, leafIndex: tuple.2, signingId: tuple.3 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (IRegistry::RegistrationProof,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (<IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getRegistrationProofReturn> for UnderlyingRustTuple<'_> {
				fn from(value: getRegistrationProofReturn) -> Self {
					(value.proof,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRegistrationProofReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { proof: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for getRegistrationProofCall {
			type Parameters<'a> = (
				alloy::sol_types::sol_data::Array<IRegistry::SignedRegistration>,
				alloy::sol_types::sol_data::Address,
				alloy::sol_types::sol_data::Uint<256>,
				alloy::sol_types::sol_data::FixedBytes<32>,
			);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType;
			type ReturnTuple<'a> = (IRegistry::RegistrationProof,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "getRegistrationProof(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],address,uint256,bytes32)";
			const SELECTOR: [u8; 4] = [148u8, 126u8, 171u8, 146u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
                    <alloy::sol_types::sol_data::Array<
                        IRegistry::SignedRegistration,
                    > as alloy_sol_types::SolType>::tokenize(&self.regs),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.leafIndex),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.signingId),
                )
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<IRegistry::RegistrationProof as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: getRegistrationProofReturn = r.into();
					r.proof
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: getRegistrationProofReturn = r.into();
					r.proof
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `getSlasherCommitment(bytes32,address)` and selector `0x5a18b192`.
	```solidity
	function getSlasherCommitment(bytes32 registrationRoot, address slasher) external view returns (IRegistry.SlasherCommitment memory);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getSlasherCommitmentCall {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub slasher: alloy::sol_types::private::Address,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`getSlasherCommitment(bytes32,address)`](getSlasherCommitmentCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getSlasherCommitmentReturn {
		#[allow(missing_docs)]
		pub _0: <IRegistry::SlasherCommitment as alloy::sol_types::SolType>::RustType,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> =
				(alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> =
				(alloy::sol_types::private::FixedBytes<32>, alloy::sol_types::private::Address);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getSlasherCommitmentCall> for UnderlyingRustTuple<'_> {
				fn from(value: getSlasherCommitmentCall) -> Self {
					(value.registrationRoot, value.slasher)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlasherCommitmentCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0, slasher: tuple.1 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (IRegistry::SlasherCommitment,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (<IRegistry::SlasherCommitment as alloy::sol_types::SolType>::RustType,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getSlasherCommitmentReturn> for UnderlyingRustTuple<'_> {
				fn from(value: getSlasherCommitmentReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlasherCommitmentReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for getSlasherCommitmentCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = <IRegistry::SlasherCommitment as alloy::sol_types::SolType>::RustType;
			type ReturnTuple<'a> = (IRegistry::SlasherCommitment,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "getSlasherCommitment(bytes32,address)";
			const SELECTOR: [u8; 4] = [90u8, 24u8, 177u8, 146u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.registrationRoot,
					),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.slasher),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<IRegistry::SlasherCommitment as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: getSlasherCommitmentReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: getSlasherCommitmentReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize)]
	/**Function with signature `getVerifiedOperatorData((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32))` and selector `0x66f1bb29`.
	```solidity
	function getVerifiedOperatorData(IRegistry.RegistrationProof memory proof) external view returns (IRegistry.OperatorData memory);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getVerifiedOperatorDataCall {
		#[allow(missing_docs)]
		pub proof: <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`getVerifiedOperatorData((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32))`](getVerifiedOperatorDataCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct getVerifiedOperatorDataReturn {
		#[allow(missing_docs)]
		pub _0: <IRegistry::OperatorData as alloy::sol_types::SolType>::RustType,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (IRegistry::RegistrationProof,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (<IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getVerifiedOperatorDataCall> for UnderlyingRustTuple<'_> {
				fn from(value: getVerifiedOperatorDataCall) -> Self {
					(value.proof,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVerifiedOperatorDataCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { proof: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (IRegistry::OperatorData,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (<IRegistry::OperatorData as alloy::sol_types::SolType>::RustType,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<getVerifiedOperatorDataReturn> for UnderlyingRustTuple<'_> {
				fn from(value: getVerifiedOperatorDataReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVerifiedOperatorDataReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for getVerifiedOperatorDataCall {
			type Parameters<'a> = (IRegistry::RegistrationProof,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = <IRegistry::OperatorData as alloy::sol_types::SolType>::RustType;
			type ReturnTuple<'a> = (IRegistry::OperatorData,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "getVerifiedOperatorData((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32))";
			const SELECTOR: [u8; 4] = [102u8, 241u8, 187u8, 41u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<IRegistry::RegistrationProof as alloy_sol_types::SolType>::tokenize(&self.proof),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<IRegistry::OperatorData as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: getVerifiedOperatorDataReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: getVerifiedOperatorDataReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `isOptedIntoSlasher(bytes32,address)` and selector `0x8486fcb8`.
	```solidity
	function isOptedIntoSlasher(bytes32 registrationRoot, address slasher) external view returns (bool);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct isOptedIntoSlasherCall {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub slasher: alloy::sol_types::private::Address,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`isOptedIntoSlasher(bytes32,address)`](isOptedIntoSlasherCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct isOptedIntoSlasherReturn {
		#[allow(missing_docs)]
		pub _0: bool,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> =
				(alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> =
				(alloy::sol_types::private::FixedBytes<32>, alloy::sol_types::private::Address);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<isOptedIntoSlasherCall> for UnderlyingRustTuple<'_> {
				fn from(value: isOptedIntoSlasherCall) -> Self {
					(value.registrationRoot, value.slasher)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOptedIntoSlasherCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0, slasher: tuple.1 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (bool,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<isOptedIntoSlasherReturn> for UnderlyingRustTuple<'_> {
				fn from(value: isOptedIntoSlasherReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOptedIntoSlasherReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for isOptedIntoSlasherCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = bool;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "isOptedIntoSlasher(bytes32,address)";
			const SELECTOR: [u8; 4] = [132u8, 134u8, 252u8, 184u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.registrationRoot,
					),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.slasher),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: isOptedIntoSlasherReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: isOptedIntoSlasherReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `isSlashed(bytes32)` and selector `0x9ee46123`.
	```solidity
	function isSlashed(bytes32 registrationRoot) external view returns (bool slashed);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct isSlashed_0Call {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`isSlashed(bytes32)`](isSlashed_0Call) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct isSlashed_0Return {
		#[allow(missing_docs)]
		pub slashed: bool,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<isSlashed_0Call> for UnderlyingRustTuple<'_> {
				fn from(value: isSlashed_0Call) -> Self {
					(value.registrationRoot,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for isSlashed_0Call {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (bool,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<isSlashed_0Return> for UnderlyingRustTuple<'_> {
				fn from(value: isSlashed_0Return) -> Self {
					(value.slashed,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for isSlashed_0Return {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { slashed: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for isSlashed_0Call {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = bool;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "isSlashed(bytes32)";
			const SELECTOR: [u8; 4] = [158u8, 228u8, 97u8, 35u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
					&self.registrationRoot,
				),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: isSlashed_0Return = r.into();
					r.slashed
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: isSlashed_0Return = r.into();
					r.slashed
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `isSlashed(bytes32,address)` and selector `0xdf2dd3ac`.
	```solidity
	function isSlashed(bytes32 registrationRoot, address slasher) external view returns (bool slashed);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct isSlashed_1Call {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub slasher: alloy::sol_types::private::Address,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`isSlashed(bytes32,address)`](isSlashed_1Call) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct isSlashed_1Return {
		#[allow(missing_docs)]
		pub slashed: bool,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> =
				(alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> =
				(alloy::sol_types::private::FixedBytes<32>, alloy::sol_types::private::Address);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<isSlashed_1Call> for UnderlyingRustTuple<'_> {
				fn from(value: isSlashed_1Call) -> Self {
					(value.registrationRoot, value.slasher)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for isSlashed_1Call {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0, slasher: tuple.1 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (bool,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<isSlashed_1Return> for UnderlyingRustTuple<'_> {
				fn from(value: isSlashed_1Return) -> Self {
					(value.slashed,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for isSlashed_1Return {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { slashed: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for isSlashed_1Call {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = bool;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "isSlashed(bytes32,address)";
			const SELECTOR: [u8; 4] = [223u8, 45u8, 211u8, 172u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.registrationRoot,
					),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.slasher),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: isSlashed_1Return = r.into();
					r.slashed
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: isSlashed_1Return = r.into();
					r.slashed
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `optInToSlasher(bytes32,address,address)` and selector `0x580f8c89`.
	```solidity
	function optInToSlasher(bytes32 registrationRoot, address slasher, address committer) external;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct optInToSlasherCall {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub slasher: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub committer: alloy::sol_types::private::Address,
	}
	///Container type for the return parameters of the [`optInToSlasher(bytes32,address,address)`](optInToSlasherCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct optInToSlasherReturn {}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (
				alloy::sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::Address,
				alloy::sol_types::sol_data::Address,
			);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (
				alloy::sol_types::private::FixedBytes<32>,
				alloy::sol_types::private::Address,
				alloy::sol_types::private::Address,
			);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<optInToSlasherCall> for UnderlyingRustTuple<'_> {
				fn from(value: optInToSlasherCall) -> Self {
					(value.registrationRoot, value.slasher, value.committer)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for optInToSlasherCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0, slasher: tuple.1, committer: tuple.2 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = ();
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = ();
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<optInToSlasherReturn> for UnderlyingRustTuple<'_> {
				fn from(value: optInToSlasherReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for optInToSlasherReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl optInToSlasherReturn {
			fn _tokenize(&self) -> <optInToSlasherCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for optInToSlasherCall {
			type Parameters<'a> = (
				alloy::sol_types::sol_data::FixedBytes<32>,
				alloy::sol_types::sol_data::Address,
				alloy::sol_types::sol_data::Address,
			);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = optInToSlasherReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "optInToSlasher(bytes32,address,address)";
			const SELECTOR: [u8; 4] = [88u8, 15u8, 140u8, 137u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.registrationRoot,
					),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.slasher),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.committer),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				optInToSlasherReturn::_tokenize(ret)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(Into::into)
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Into::into)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `optOutOfSlasher(bytes32,address)` and selector `0x7cf3c729`.
	```solidity
	function optOutOfSlasher(bytes32 registrationRoot, address slasher) external;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct optOutOfSlasherCall {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub slasher: alloy::sol_types::private::Address,
	}
	///Container type for the return parameters of the [`optOutOfSlasher(bytes32,address)`](optOutOfSlasherCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct optOutOfSlasherReturn {}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> =
				(alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> =
				(alloy::sol_types::private::FixedBytes<32>, alloy::sol_types::private::Address);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<optOutOfSlasherCall> for UnderlyingRustTuple<'_> {
				fn from(value: optOutOfSlasherCall) -> Self {
					(value.registrationRoot, value.slasher)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for optOutOfSlasherCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0, slasher: tuple.1 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = ();
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = ();
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<optOutOfSlasherReturn> for UnderlyingRustTuple<'_> {
				fn from(value: optOutOfSlasherReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for optOutOfSlasherReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl optOutOfSlasherReturn {
			fn _tokenize(&self) -> <optOutOfSlasherCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for optOutOfSlasherCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = optOutOfSlasherReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "optOutOfSlasher(bytes32,address)";
			const SELECTOR: [u8; 4] = [124u8, 243u8, 199u8, 41u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.registrationRoot,
					),
					<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(&self.slasher),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				optOutOfSlasherReturn::_tokenize(ret)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(Into::into)
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Into::into)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `register(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],address,bytes32)` and selector `0xdce43695`.
	```solidity
	function register(IRegistry.SignedRegistration[] memory registrations, address owner, bytes32 signingId) external payable returns (bytes32 registrationRoot);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct registerCall {
		#[allow(missing_docs)]
		pub registrations:
			alloy::sol_types::private::Vec<<IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType>,
		#[allow(missing_docs)]
		pub owner: alloy::sol_types::private::Address,
		#[allow(missing_docs)]
		pub signingId: alloy::sol_types::private::FixedBytes<32>,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`register(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],address,bytes32)`](registerCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct registerReturn {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (
				alloy::sol_types::sol_data::Array<IRegistry::SignedRegistration>,
				alloy::sol_types::sol_data::Address,
				alloy::sol_types::sol_data::FixedBytes<32>,
			);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (
				alloy::sol_types::private::Vec<<IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType>,
				alloy::sol_types::private::Address,
				alloy::sol_types::private::FixedBytes<32>,
			);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<registerCall> for UnderlyingRustTuple<'_> {
				fn from(value: registerCall) -> Self {
					(value.registrations, value.owner, value.signingId)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrations: tuple.0, owner: tuple.1, signingId: tuple.2 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<registerReturn> for UnderlyingRustTuple<'_> {
				fn from(value: registerReturn) -> Self {
					(value.registrationRoot,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for registerCall {
			type Parameters<'a> = (
				alloy::sol_types::sol_data::Array<IRegistry::SignedRegistration>,
				alloy::sol_types::sol_data::Address,
				alloy::sol_types::sol_data::FixedBytes<32>,
			);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::FixedBytes<32>;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "register(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64)[],address,bytes32)";
			const SELECTOR: [u8; 4] = [220u8, 228u8, 54u8, 149u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
                    <alloy::sol_types::sol_data::Array<
                        IRegistry::SignedRegistration,
                    > as alloy_sol_types::SolType>::tokenize(&self.registrations),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.signingId),
                )
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: registerReturn = r.into();
					r.registrationRoot
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: registerReturn = r.into();
					r.registrationRoot
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize)]
	/**Function with signature `slashCommitment(bytes32,((uint64,bytes,bytes32,address),uint64,bytes32,bytes),bytes)` and selector `0x20766026`.
	```solidity
	function slashCommitment(bytes32 registrationRoot, ISlasher.SignedCommitment memory commitment, bytes memory evidence) external returns (uint256 slashAmountWei);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct slashCommitment_0Call {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		#[allow(missing_docs)]
		pub commitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub evidence: alloy::sol_types::private::Bytes,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`slashCommitment(bytes32,((uint64,bytes,bytes32,address),uint64,bytes32,bytes),bytes)`](slashCommitment_0Call) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct slashCommitment_0Return {
		#[allow(missing_docs)]
		pub slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (
				alloy::sol_types::sol_data::FixedBytes<32>,
				ISlasher::SignedCommitment,
				alloy::sol_types::sol_data::Bytes,
			);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (
				alloy::sol_types::private::FixedBytes<32>,
				<ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
				alloy::sol_types::private::Bytes,
			);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<slashCommitment_0Call> for UnderlyingRustTuple<'_> {
				fn from(value: slashCommitment_0Call) -> Self {
					(value.registrationRoot, value.commitment, value.evidence)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashCommitment_0Call {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0, commitment: tuple.1, evidence: tuple.2 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<slashCommitment_0Return> for UnderlyingRustTuple<'_> {
				fn from(value: slashCommitment_0Return) -> Self {
					(value.slashAmountWei,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashCommitment_0Return {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { slashAmountWei: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for slashCommitment_0Call {
			type Parameters<'a> = (
				alloy::sol_types::sol_data::FixedBytes<32>,
				ISlasher::SignedCommitment,
				alloy::sol_types::sol_data::Bytes,
			);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str =
				"slashCommitment(bytes32,((uint64,bytes,bytes32,address),uint64,bytes32,bytes),bytes)";
			const SELECTOR: [u8; 4] = [32u8, 118u8, 96u8, 38u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
						&self.registrationRoot,
					),
					<ISlasher::SignedCommitment as alloy_sol_types::SolType>::tokenize(&self.commitment),
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(&self.evidence),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: slashCommitment_0Return = r.into();
					r.slashAmountWei
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: slashCommitment_0Return = r.into();
					r.slashAmountWei
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize)]
	/**Function with signature `slashCommitment((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)),((uint64,bytes,bytes32,address),uint64,bytes32,bytes),bytes)` and selector `0x455bc5a7`.
	```solidity
	function slashCommitment(IRegistry.RegistrationProof memory proof, ISlasher.SignedDelegation memory delegation, ISlasher.SignedCommitment memory commitment, bytes memory evidence) external returns (uint256 slashAmountWei);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct slashCommitment_1Call {
		#[allow(missing_docs)]
		pub proof: <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub delegation: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub commitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub evidence: alloy::sol_types::private::Bytes,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`slashCommitment((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)),((uint64,bytes,bytes32,address),uint64,bytes32,bytes),bytes)`](slashCommitment_1Call) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct slashCommitment_1Return {
		#[allow(missing_docs)]
		pub slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (
				IRegistry::RegistrationProof,
				ISlasher::SignedDelegation,
				ISlasher::SignedCommitment,
				alloy::sol_types::sol_data::Bytes,
			);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (
				<IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
				<ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
				<ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
				alloy::sol_types::private::Bytes,
			);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<slashCommitment_1Call> for UnderlyingRustTuple<'_> {
				fn from(value: slashCommitment_1Call) -> Self {
					(value.proof, value.delegation, value.commitment, value.evidence)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashCommitment_1Call {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { proof: tuple.0, delegation: tuple.1, commitment: tuple.2, evidence: tuple.3 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<slashCommitment_1Return> for UnderlyingRustTuple<'_> {
				fn from(value: slashCommitment_1Return) -> Self {
					(value.slashAmountWei,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashCommitment_1Return {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { slashAmountWei: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for slashCommitment_1Call {
			type Parameters<'a> = (
				IRegistry::RegistrationProof,
				ISlasher::SignedDelegation,
				ISlasher::SignedCommitment,
				alloy::sol_types::sol_data::Bytes,
			);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "slashCommitment((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)),((uint64,bytes,bytes32,address),uint64,bytes32,bytes),bytes)";
			const SELECTOR: [u8; 4] = [69u8, 91u8, 197u8, 167u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<IRegistry::RegistrationProof as alloy_sol_types::SolType>::tokenize(&self.proof),
					<ISlasher::SignedDelegation as alloy_sol_types::SolType>::tokenize(&self.delegation),
					<ISlasher::SignedCommitment as alloy_sol_types::SolType>::tokenize(&self.commitment),
					<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(&self.evidence),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: slashCommitment_1Return = r.into();
					r.slashAmountWei
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: slashCommitment_1Return = r.into();
					r.slashAmountWei
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize)]
	/**Function with signature `slashEquivocation((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))` and selector `0x499ae2f2`.
	```solidity
	function slashEquivocation(IRegistry.RegistrationProof memory proof, ISlasher.SignedDelegation memory delegationOne, ISlasher.SignedDelegation memory delegationTwo) external returns (uint256 slashAmountWei);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct slashEquivocationCall {
		#[allow(missing_docs)]
		pub proof: <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub delegationOne: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
		#[allow(missing_docs)]
		pub delegationTwo: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`slashEquivocation((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))`](slashEquivocationCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct slashEquivocationReturn {
		#[allow(missing_docs)]
		pub slashAmountWei: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> =
				(IRegistry::RegistrationProof, ISlasher::SignedDelegation, ISlasher::SignedDelegation);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (
				<IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
				<ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
				<ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
			);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<slashEquivocationCall> for UnderlyingRustTuple<'_> {
				fn from(value: slashEquivocationCall) -> Self {
					(value.proof, value.delegationOne, value.delegationTwo)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashEquivocationCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { proof: tuple.0, delegationOne: tuple.1, delegationTwo: tuple.2 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<slashEquivocationReturn> for UnderlyingRustTuple<'_> {
				fn from(value: slashEquivocationReturn) -> Self {
					(value.slashAmountWei,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashEquivocationReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { slashAmountWei: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for slashEquivocationCall {
			type Parameters<'a> =
				(IRegistry::RegistrationProof, ISlasher::SignedDelegation, ISlasher::SignedDelegation);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "slashEquivocation((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)),(((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32),address,uint64,bytes),uint64,bytes32,(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32)))";
			const SELECTOR: [u8; 4] = [73u8, 154u8, 226u8, 242u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(
					<IRegistry::RegistrationProof as alloy_sol_types::SolType>::tokenize(&self.proof),
					<ISlasher::SignedDelegation as alloy_sol_types::SolType>::tokenize(&self.delegationOne),
					<ISlasher::SignedDelegation as alloy_sol_types::SolType>::tokenize(&self.delegationTwo),
				)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: slashEquivocationReturn = r.into();
					r.slashAmountWei
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: slashEquivocationReturn = r.into();
					r.slashAmountWei
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize)]
	/**Function with signature `slashRegistration((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32))` and selector `0xfc0ce59b`.
	```solidity
	function slashRegistration(IRegistry.RegistrationProof memory proof) external returns (uint256 slashedCollateralWei);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct slashRegistrationCall {
		#[allow(missing_docs)]
		pub proof: <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`slashRegistration((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32))`](slashRegistrationCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct slashRegistrationReturn {
		#[allow(missing_docs)]
		pub slashedCollateralWei: alloy::sol_types::private::primitives::aliases::U256,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (IRegistry::RegistrationProof,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (<IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<slashRegistrationCall> for UnderlyingRustTuple<'_> {
				fn from(value: slashRegistrationCall) -> Self {
					(value.proof,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashRegistrationCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { proof: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<slashRegistrationReturn> for UnderlyingRustTuple<'_> {
				fn from(value: slashRegistrationReturn) -> Self {
					(value.slashedCollateralWei,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashRegistrationReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { slashedCollateralWei: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for slashRegistrationCall {
			type Parameters<'a> = (IRegistry::RegistrationProof,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = alloy::sol_types::private::primitives::aliases::U256;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "slashRegistration((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32))";
			const SELECTOR: [u8; 4] = [252u8, 12u8, 229u8, 155u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<IRegistry::RegistrationProof as alloy_sol_types::SolType>::tokenize(&self.proof),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: slashRegistrationReturn = r.into();
					r.slashedCollateralWei
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: slashRegistrationReturn = r.into();
					r.slashedCollateralWei
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `slashingEvidenceAlreadyUsed(bytes32)` and selector `0xc02ecd8c`.
	```solidity
	function slashingEvidenceAlreadyUsed(bytes32 slashingDigest) external view returns (bool);
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct slashingEvidenceAlreadyUsedCall {
		#[allow(missing_docs)]
		pub slashingDigest: alloy::sol_types::private::FixedBytes<32>,
	}
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	///Container type for the return parameters of the [`slashingEvidenceAlreadyUsed(bytes32)`](slashingEvidenceAlreadyUsedCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct slashingEvidenceAlreadyUsedReturn {
		#[allow(missing_docs)]
		pub _0: bool,
	}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<slashingEvidenceAlreadyUsedCall> for UnderlyingRustTuple<'_> {
				fn from(value: slashingEvidenceAlreadyUsedCall) -> Self {
					(value.slashingDigest,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashingEvidenceAlreadyUsedCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { slashingDigest: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (bool,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<slashingEvidenceAlreadyUsedReturn> for UnderlyingRustTuple<'_> {
				fn from(value: slashingEvidenceAlreadyUsedReturn) -> Self {
					(value._0,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashingEvidenceAlreadyUsedReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { _0: tuple.0 }
				}
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for slashingEvidenceAlreadyUsedCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = bool;
			type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "slashingEvidenceAlreadyUsed(bytes32)";
			const SELECTOR: [u8; 4] = [192u8, 46u8, 205u8, 140u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
					&self.slashingDigest,
				),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				(<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(|r| {
					let r: slashingEvidenceAlreadyUsedReturn = r.into();
					r._0
				})
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(|r| {
					let r: slashingEvidenceAlreadyUsedReturn = r.into();
					r._0
				})
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
	/**Function with signature `unregister(bytes32)` and selector `0x1a0919dc`.
	```solidity
	function unregister(bytes32 registrationRoot) external;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct unregisterCall {
		#[allow(missing_docs)]
		pub registrationRoot: alloy::sol_types::private::FixedBytes<32>,
	}
	///Container type for the return parameters of the [`unregister(bytes32)`](unregisterCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct unregisterReturn {}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<unregisterCall> for UnderlyingRustTuple<'_> {
				fn from(value: unregisterCall) -> Self {
					(value.registrationRoot,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for unregisterCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { registrationRoot: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = ();
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = ();
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<unregisterReturn> for UnderlyingRustTuple<'_> {
				fn from(value: unregisterReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for unregisterReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl unregisterReturn {
			fn _tokenize(&self) -> <unregisterCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for unregisterCall {
			type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = unregisterReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "unregister(bytes32)";
			const SELECTOR: [u8; 4] = [26u8, 9u8, 25u8, 220u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<alloy::sol_types::sol_data::FixedBytes<32> as alloy_sol_types::SolType>::tokenize(
					&self.registrationRoot,
				),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				unregisterReturn::_tokenize(ret)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(Into::into)
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Into::into)
			}
		}
	};
	#[derive(serde::Serialize, serde::Deserialize)]
	/**Function with signature `verifyMerkleProof((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32))` and selector `0x6f46e574`.
	```solidity
	function verifyMerkleProof(IRegistry.RegistrationProof memory proof) external view;
	```*/
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct verifyMerkleProofCall {
		#[allow(missing_docs)]
		pub proof: <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
	}
	///Container type for the return parameters of the [`verifyMerkleProof((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32))`](verifyMerkleProofCall) function.
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
	#[derive(Clone)]
	pub struct verifyMerkleProofReturn {}
	#[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
	const _: () = {
		use alloy::sol_types as alloy_sol_types;
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = (IRegistry::RegistrationProof,);
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = (<IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,);
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<verifyMerkleProofCall> for UnderlyingRustTuple<'_> {
				fn from(value: verifyMerkleProofCall) -> Self {
					(value.proof,)
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyMerkleProofCall {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self { proof: tuple.0 }
				}
			}
		}
		{
			#[doc(hidden)]
			#[allow(dead_code)]
			type UnderlyingSolTuple<'a> = ();
			#[doc(hidden)]
			type UnderlyingRustTuple<'a> = ();
			#[cfg(test)]
			#[allow(dead_code, unreachable_patterns)]
			fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
				match _t {
					alloy_sol_types::private::AssertTypeEq::<
						<UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
					>(_) => {}
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<verifyMerkleProofReturn> for UnderlyingRustTuple<'_> {
				fn from(value: verifyMerkleProofReturn) -> Self {
					()
				}
			}
			#[automatically_derived]
			#[doc(hidden)]
			impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyMerkleProofReturn {
				fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
					Self {}
				}
			}
		}
		impl verifyMerkleProofReturn {
			fn _tokenize(&self) -> <verifyMerkleProofCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
				()
			}
		}
		#[automatically_derived]
		impl alloy_sol_types::SolCall for verifyMerkleProofCall {
			type Parameters<'a> = (IRegistry::RegistrationProof,);
			type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
			type Return = verifyMerkleProofReturn;
			type ReturnTuple<'a> = ();
			type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
			const SIGNATURE: &'static str = "verifyMerkleProof((bytes32,((bytes32,bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32,bytes32),uint64),bytes32[],bytes32))";
			const SELECTOR: [u8; 4] = [111u8, 70u8, 229u8, 116u8];
			#[inline]
			fn new<'a>(tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType) -> Self {
				tuple.into()
			}
			#[inline]
			fn tokenize(&self) -> Self::Token<'_> {
				(<IRegistry::RegistrationProof as alloy_sol_types::SolType>::tokenize(&self.proof),)
			}
			#[inline]
			fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
				verifyMerkleProofReturn::_tokenize(ret)
			}
			#[inline]
			fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(Into::into)
			}
			#[inline]
			fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
				<Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(data).map(Into::into)
			}
		}
	};
	///Container for all the [`Registry`](self) function calls.
	#[derive(Clone, serde::Serialize, serde::Deserialize)]
	pub enum RegistryCalls {
		#[allow(missing_docs)]
		addCollateral(addCollateralCall),
		#[allow(missing_docs)]
		claimCollateral(claimCollateralCall),
		#[allow(missing_docs)]
		claimSlashedCollateral(claimSlashedCollateralCall),
		#[allow(missing_docs)]
		getConfig(getConfigCall),
		#[allow(missing_docs)]
		getHistoricalCollateral(getHistoricalCollateralCall),
		#[allow(missing_docs)]
		getOperatorData(getOperatorDataCall),
		#[allow(missing_docs)]
		getRegistrationProof(getRegistrationProofCall),
		#[allow(missing_docs)]
		getSlasherCommitment(getSlasherCommitmentCall),
		#[allow(missing_docs)]
		getVerifiedOperatorData(getVerifiedOperatorDataCall),
		#[allow(missing_docs)]
		isOptedIntoSlasher(isOptedIntoSlasherCall),
		#[allow(missing_docs)]
		isSlashed_0(isSlashed_0Call),
		#[allow(missing_docs)]
		isSlashed_1(isSlashed_1Call),
		#[allow(missing_docs)]
		optInToSlasher(optInToSlasherCall),
		#[allow(missing_docs)]
		optOutOfSlasher(optOutOfSlasherCall),
		#[allow(missing_docs)]
		register(registerCall),
		#[allow(missing_docs)]
		slashCommitment_0(slashCommitment_0Call),
		#[allow(missing_docs)]
		slashCommitment_1(slashCommitment_1Call),
		#[allow(missing_docs)]
		slashEquivocation(slashEquivocationCall),
		#[allow(missing_docs)]
		slashRegistration(slashRegistrationCall),
		#[allow(missing_docs)]
		slashingEvidenceAlreadyUsed(slashingEvidenceAlreadyUsedCall),
		#[allow(missing_docs)]
		unregister(unregisterCall),
		#[allow(missing_docs)]
		verifyMerkleProof(verifyMerkleProofCall),
	}
	impl RegistryCalls {
		/// All the selectors of this enum.
		///
		/// Note that the selectors might not be in the same order as the variants.
		/// No guarantees are made about the order of the selectors.
		///
		/// Prefer using `SolInterface` methods instead.
		pub const SELECTORS: &'static [[u8; 4usize]] = &[
			[26u8, 9u8, 25u8, 220u8],
			[32u8, 118u8, 96u8, 38u8],
			[36u8, 56u8, 123u8, 180u8],
			[69u8, 91u8, 197u8, 167u8],
			[73u8, 154u8, 226u8, 242u8],
			[88u8, 15u8, 140u8, 137u8],
			[90u8, 24u8, 177u8, 146u8],
			[102u8, 241u8, 187u8, 41u8],
			[111u8, 70u8, 229u8, 116u8],
			[124u8, 243u8, 199u8, 41u8],
			[132u8, 134u8, 252u8, 184u8],
			[147u8, 100u8, 73u8, 236u8],
			[148u8, 126u8, 171u8, 146u8],
			[158u8, 228u8, 97u8, 35u8],
			[159u8, 30u8, 44u8, 132u8],
			[192u8, 46u8, 205u8, 140u8],
			[195u8, 249u8, 9u8, 212u8],
			[220u8, 228u8, 54u8, 149u8],
			[223u8, 45u8, 211u8, 172u8],
			[227u8, 252u8, 2u8, 141u8],
			[242u8, 61u8, 64u8, 38u8],
			[252u8, 12u8, 229u8, 155u8],
		];
		/// The names of the variants in the same order as `SELECTORS`.
		pub const VARIANT_NAMES: &'static [&'static str] = &[
			::core::stringify!(unregister),
			::core::stringify!(slashCommitment_0),
			::core::stringify!(getHistoricalCollateral),
			::core::stringify!(slashCommitment_1),
			::core::stringify!(slashEquivocation),
			::core::stringify!(optInToSlasher),
			::core::stringify!(getSlasherCommitment),
			::core::stringify!(getVerifiedOperatorData),
			::core::stringify!(verifyMerkleProof),
			::core::stringify!(optOutOfSlasher),
			::core::stringify!(isOptedIntoSlasher),
			::core::stringify!(getOperatorData),
			::core::stringify!(getRegistrationProof),
			::core::stringify!(isSlashed_0),
			::core::stringify!(claimSlashedCollateral),
			::core::stringify!(slashingEvidenceAlreadyUsed),
			::core::stringify!(getConfig),
			::core::stringify!(register),
			::core::stringify!(isSlashed_1),
			::core::stringify!(claimCollateral),
			::core::stringify!(addCollateral),
			::core::stringify!(slashRegistration),
		];
		/// The signatures in the same order as `SELECTORS`.
		pub const SIGNATURES: &'static [&'static str] = &[
			<unregisterCall as alloy_sol_types::SolCall>::SIGNATURE,
			<slashCommitment_0Call as alloy_sol_types::SolCall>::SIGNATURE,
			<getHistoricalCollateralCall as alloy_sol_types::SolCall>::SIGNATURE,
			<slashCommitment_1Call as alloy_sol_types::SolCall>::SIGNATURE,
			<slashEquivocationCall as alloy_sol_types::SolCall>::SIGNATURE,
			<optInToSlasherCall as alloy_sol_types::SolCall>::SIGNATURE,
			<getSlasherCommitmentCall as alloy_sol_types::SolCall>::SIGNATURE,
			<getVerifiedOperatorDataCall as alloy_sol_types::SolCall>::SIGNATURE,
			<verifyMerkleProofCall as alloy_sol_types::SolCall>::SIGNATURE,
			<optOutOfSlasherCall as alloy_sol_types::SolCall>::SIGNATURE,
			<isOptedIntoSlasherCall as alloy_sol_types::SolCall>::SIGNATURE,
			<getOperatorDataCall as alloy_sol_types::SolCall>::SIGNATURE,
			<getRegistrationProofCall as alloy_sol_types::SolCall>::SIGNATURE,
			<isSlashed_0Call as alloy_sol_types::SolCall>::SIGNATURE,
			<claimSlashedCollateralCall as alloy_sol_types::SolCall>::SIGNATURE,
			<slashingEvidenceAlreadyUsedCall as alloy_sol_types::SolCall>::SIGNATURE,
			<getConfigCall as alloy_sol_types::SolCall>::SIGNATURE,
			<registerCall as alloy_sol_types::SolCall>::SIGNATURE,
			<isSlashed_1Call as alloy_sol_types::SolCall>::SIGNATURE,
			<claimCollateralCall as alloy_sol_types::SolCall>::SIGNATURE,
			<addCollateralCall as alloy_sol_types::SolCall>::SIGNATURE,
			<slashRegistrationCall as alloy_sol_types::SolCall>::SIGNATURE,
		];
		/// Returns the signature for the given selector, if known.
		#[inline]
		pub fn signature_by_selector(selector: [u8; 4usize]) -> ::core::option::Option<&'static str> {
			match Self::SELECTORS.binary_search(&selector) {
				::core::result::Result::Ok(idx) => ::core::option::Option::Some(Self::SIGNATURES[idx]),
				::core::result::Result::Err(_) => ::core::option::Option::None,
			}
		}
		/// Returns the enum variant name for the given selector, if known.
		#[inline]
		pub fn name_by_selector(selector: [u8; 4usize]) -> ::core::option::Option<&'static str> {
			let sig = Self::signature_by_selector(selector)?;
			sig.split_once('(').map(|(name, _)| name)
		}
	}
	#[automatically_derived]
	impl alloy_sol_types::SolInterface for RegistryCalls {
		const NAME: &'static str = "RegistryCalls";
		const MIN_DATA_LENGTH: usize = 0usize;
		const COUNT: usize = 22usize;
		#[inline]
		fn selector(&self) -> [u8; 4] {
			match self {
				Self::addCollateral(_) => <addCollateralCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::claimCollateral(_) => <claimCollateralCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::claimSlashedCollateral(_) => <claimSlashedCollateralCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::getConfig(_) => <getConfigCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::getHistoricalCollateral(_) => <getHistoricalCollateralCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::getOperatorData(_) => <getOperatorDataCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::getRegistrationProof(_) => <getRegistrationProofCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::getSlasherCommitment(_) => <getSlasherCommitmentCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::getVerifiedOperatorData(_) => <getVerifiedOperatorDataCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::isOptedIntoSlasher(_) => <isOptedIntoSlasherCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::isSlashed_0(_) => <isSlashed_0Call as alloy_sol_types::SolCall>::SELECTOR,
				Self::isSlashed_1(_) => <isSlashed_1Call as alloy_sol_types::SolCall>::SELECTOR,
				Self::optInToSlasher(_) => <optInToSlasherCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::optOutOfSlasher(_) => <optOutOfSlasherCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::register(_) => <registerCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::slashCommitment_0(_) => <slashCommitment_0Call as alloy_sol_types::SolCall>::SELECTOR,
				Self::slashCommitment_1(_) => <slashCommitment_1Call as alloy_sol_types::SolCall>::SELECTOR,
				Self::slashEquivocation(_) => <slashEquivocationCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::slashRegistration(_) => <slashRegistrationCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::slashingEvidenceAlreadyUsed(_) => {
					<slashingEvidenceAlreadyUsedCall as alloy_sol_types::SolCall>::SELECTOR
				}
				Self::unregister(_) => <unregisterCall as alloy_sol_types::SolCall>::SELECTOR,
				Self::verifyMerkleProof(_) => <verifyMerkleProofCall as alloy_sol_types::SolCall>::SELECTOR,
			}
		}
		#[inline]
		fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
			Self::SELECTORS.get(i).copied()
		}
		#[inline]
		fn valid_selector(selector: [u8; 4]) -> bool {
			Self::SELECTORS.binary_search(&selector).is_ok()
		}
		#[inline]
		#[allow(non_snake_case)]
		fn abi_decode_raw(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
			static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<RegistryCalls>] = &[
				{
					fn unregister(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<unregisterCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::unregister)
					}
					unregister
				},
				{
					fn slashCommitment_0(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<slashCommitment_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::slashCommitment_0)
					}
					slashCommitment_0
				},
				{
					fn getHistoricalCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getHistoricalCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::getHistoricalCollateral)
					}
					getHistoricalCollateral
				},
				{
					fn slashCommitment_1(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<slashCommitment_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::slashCommitment_1)
					}
					slashCommitment_1
				},
				{
					fn slashEquivocation(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<slashEquivocationCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::slashEquivocation)
					}
					slashEquivocation
				},
				{
					fn optInToSlasher(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<optInToSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::optInToSlasher)
					}
					optInToSlasher
				},
				{
					fn getSlasherCommitment(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getSlasherCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::getSlasherCommitment)
					}
					getSlasherCommitment
				},
				{
					fn getVerifiedOperatorData(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getVerifiedOperatorDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::getVerifiedOperatorData)
					}
					getVerifiedOperatorData
				},
				{
					fn verifyMerkleProof(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<verifyMerkleProofCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::verifyMerkleProof)
					}
					verifyMerkleProof
				},
				{
					fn optOutOfSlasher(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<optOutOfSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::optOutOfSlasher)
					}
					optOutOfSlasher
				},
				{
					fn isOptedIntoSlasher(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<isOptedIntoSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::isOptedIntoSlasher)
					}
					isOptedIntoSlasher
				},
				{
					fn getOperatorData(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getOperatorDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::getOperatorData)
					}
					getOperatorData
				},
				{
					fn getRegistrationProof(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getRegistrationProofCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::getRegistrationProof)
					}
					getRegistrationProof
				},
				{
					fn isSlashed_0(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<isSlashed_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::isSlashed_0)
					}
					isSlashed_0
				},
				{
					fn claimSlashedCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<claimSlashedCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::claimSlashedCollateral)
					}
					claimSlashedCollateral
				},
				{
					fn slashingEvidenceAlreadyUsed(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<slashingEvidenceAlreadyUsedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::slashingEvidenceAlreadyUsed)
					}
					slashingEvidenceAlreadyUsed
				},
				{
					fn getConfig(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(data).map(RegistryCalls::getConfig)
					}
					getConfig
				},
				{
					fn register(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<registerCall as alloy_sol_types::SolCall>::abi_decode_raw(data).map(RegistryCalls::register)
					}
					register
				},
				{
					fn isSlashed_1(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<isSlashed_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::isSlashed_1)
					}
					isSlashed_1
				},
				{
					fn claimCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<claimCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::claimCollateral)
					}
					claimCollateral
				},
				{
					fn addCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<addCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::addCollateral)
					}
					addCollateral
				},
				{
					fn slashRegistration(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<slashRegistrationCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
							.map(RegistryCalls::slashRegistration)
					}
					slashRegistration
				},
			];
			let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
				return Err(alloy_sol_types::Error::unknown_selector(
					<Self as alloy_sol_types::SolInterface>::NAME,
					selector,
				));
			};
			DECODE_SHIMS[idx](data)
		}
		#[inline]
		#[allow(non_snake_case)]
		fn abi_decode_raw_validate(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
			static DECODE_VALIDATE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<RegistryCalls>] = &[
				{
					fn unregister(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<unregisterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::unregister)
					}
					unregister
				},
				{
					fn slashCommitment_0(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<slashCommitment_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::slashCommitment_0)
					}
					slashCommitment_0
				},
				{
					fn getHistoricalCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getHistoricalCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::getHistoricalCollateral)
					}
					getHistoricalCollateral
				},
				{
					fn slashCommitment_1(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<slashCommitment_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::slashCommitment_1)
					}
					slashCommitment_1
				},
				{
					fn slashEquivocation(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<slashEquivocationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::slashEquivocation)
					}
					slashEquivocation
				},
				{
					fn optInToSlasher(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<optInToSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::optInToSlasher)
					}
					optInToSlasher
				},
				{
					fn getSlasherCommitment(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getSlasherCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::getSlasherCommitment)
					}
					getSlasherCommitment
				},
				{
					fn getVerifiedOperatorData(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getVerifiedOperatorDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::getVerifiedOperatorData)
					}
					getVerifiedOperatorData
				},
				{
					fn verifyMerkleProof(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<verifyMerkleProofCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::verifyMerkleProof)
					}
					verifyMerkleProof
				},
				{
					fn optOutOfSlasher(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<optOutOfSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::optOutOfSlasher)
					}
					optOutOfSlasher
				},
				{
					fn isOptedIntoSlasher(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<isOptedIntoSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::isOptedIntoSlasher)
					}
					isOptedIntoSlasher
				},
				{
					fn getOperatorData(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getOperatorDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::getOperatorData)
					}
					getOperatorData
				},
				{
					fn getRegistrationProof(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getRegistrationProofCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::getRegistrationProof)
					}
					getRegistrationProof
				},
				{
					fn isSlashed_0(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<isSlashed_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::isSlashed_0)
					}
					isSlashed_0
				},
				{
					fn claimSlashedCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<claimSlashedCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::claimSlashedCollateral)
					}
					claimSlashedCollateral
				},
				{
					fn slashingEvidenceAlreadyUsed(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<slashingEvidenceAlreadyUsedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::slashingEvidenceAlreadyUsed)
					}
					slashingEvidenceAlreadyUsed
				},
				{
					fn getConfig(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<getConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::getConfig)
					}
					getConfig
				},
				{
					fn register(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<registerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::register)
					}
					register
				},
				{
					fn isSlashed_1(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<isSlashed_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::isSlashed_1)
					}
					isSlashed_1
				},
				{
					fn claimCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<claimCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::claimCollateral)
					}
					claimCollateral
				},
				{
					fn addCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<addCollateralCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::addCollateral)
					}
					addCollateral
				},
				{
					fn slashRegistration(data: &[u8]) -> alloy_sol_types::Result<RegistryCalls> {
						<slashRegistrationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
							.map(RegistryCalls::slashRegistration)
					}
					slashRegistration
				},
			];
			let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
				return Err(alloy_sol_types::Error::unknown_selector(
					<Self as alloy_sol_types::SolInterface>::NAME,
					selector,
				));
			};
			DECODE_VALIDATE_SHIMS[idx](data)
		}
		#[inline]
		fn abi_encoded_size(&self) -> usize {
			match self {
				Self::addCollateral(inner) => <addCollateralCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::claimCollateral(inner) => {
					<claimCollateralCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::claimSlashedCollateral(inner) => {
					<claimSlashedCollateralCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::getConfig(inner) => <getConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::getHistoricalCollateral(inner) => {
					<getHistoricalCollateralCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::getOperatorData(inner) => {
					<getOperatorDataCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::getRegistrationProof(inner) => {
					<getRegistrationProofCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::getSlasherCommitment(inner) => {
					<getSlasherCommitmentCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::getVerifiedOperatorData(inner) => {
					<getVerifiedOperatorDataCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::isOptedIntoSlasher(inner) => {
					<isOptedIntoSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::isSlashed_0(inner) => <isSlashed_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::isSlashed_1(inner) => <isSlashed_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::optInToSlasher(inner) => {
					<optInToSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::optOutOfSlasher(inner) => {
					<optOutOfSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::register(inner) => <registerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::slashCommitment_0(inner) => {
					<slashCommitment_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::slashCommitment_1(inner) => {
					<slashCommitment_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::slashEquivocation(inner) => {
					<slashEquivocationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::slashRegistration(inner) => {
					<slashRegistrationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::slashingEvidenceAlreadyUsed(inner) => {
					<slashingEvidenceAlreadyUsedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
				Self::unregister(inner) => <unregisterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
				Self::verifyMerkleProof(inner) => {
					<verifyMerkleProofCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
				}
			}
		}
		#[inline]
		fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
			match self {
				Self::addCollateral(inner) => {
					<addCollateralCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::claimCollateral(inner) => {
					<claimCollateralCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::claimSlashedCollateral(inner) => {
					<claimSlashedCollateralCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::getConfig(inner) => <getConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::getHistoricalCollateral(inner) => {
					<getHistoricalCollateralCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::getOperatorData(inner) => {
					<getOperatorDataCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::getRegistrationProof(inner) => {
					<getRegistrationProofCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::getSlasherCommitment(inner) => {
					<getSlasherCommitmentCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::getVerifiedOperatorData(inner) => {
					<getVerifiedOperatorDataCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::isOptedIntoSlasher(inner) => {
					<isOptedIntoSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::isSlashed_0(inner) => <isSlashed_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::isSlashed_1(inner) => <isSlashed_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::optInToSlasher(inner) => {
					<optInToSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::optOutOfSlasher(inner) => {
					<optOutOfSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::register(inner) => <registerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::slashCommitment_0(inner) => {
					<slashCommitment_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::slashCommitment_1(inner) => {
					<slashCommitment_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::slashEquivocation(inner) => {
					<slashEquivocationCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::slashRegistration(inner) => {
					<slashRegistrationCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::slashingEvidenceAlreadyUsed(inner) => {
					<slashingEvidenceAlreadyUsedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
				Self::unregister(inner) => <unregisterCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out),
				Self::verifyMerkleProof(inner) => {
					<verifyMerkleProofCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
				}
			}
		}
	}
	///Container for all the [`Registry`](self) custom errors.
	#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
	pub enum RegistryErrors {
		#[allow(missing_docs)]
		AlreadyOptedIn(AlreadyOptedIn),
		#[allow(missing_docs)]
		AlreadyUnregistered(AlreadyUnregistered),
		#[allow(missing_docs)]
		CollateralBelowMinimum(CollateralBelowMinimum),
		#[allow(missing_docs)]
		CollateralOverflow(CollateralOverflow),
		#[allow(missing_docs)]
		DelegationSignatureInvalid(DelegationSignatureInvalid),
		#[allow(missing_docs)]
		DelegationsAreSame(DelegationsAreSame),
		#[allow(missing_docs)]
		DifferentSlots(DifferentSlots),
		#[allow(missing_docs)]
		EthTransferFailed(EthTransferFailed),
		#[allow(missing_docs)]
		FraudProofChallengeInvalid(FraudProofChallengeInvalid),
		#[allow(missing_docs)]
		FraudProofMerklePathInvalid(FraudProofMerklePathInvalid),
		#[allow(missing_docs)]
		FraudProofWindowExpired(FraudProofWindowExpired),
		#[allow(missing_docs)]
		FraudProofWindowNotMet(FraudProofWindowNotMet),
		#[allow(missing_docs)]
		InsufficientCollateral(InsufficientCollateral),
		#[allow(missing_docs)]
		InvalidDelegation(InvalidDelegation),
		#[allow(missing_docs)]
		InvalidOwnerAddress(InvalidOwnerAddress),
		#[allow(missing_docs)]
		InvalidProof(InvalidProof),
		#[allow(missing_docs)]
		InvalidRegistrationRoot(InvalidRegistrationRoot),
		#[allow(missing_docs)]
		NoCollateral(NoCollateral),
		#[allow(missing_docs)]
		NoCollateralSlashed(NoCollateralSlashed),
		#[allow(missing_docs)]
		NoCollateralToClaim(NoCollateralToClaim),
		#[allow(missing_docs)]
		NotOptedIn(NotOptedIn),
		#[allow(missing_docs)]
		NotRegisteredKey(NotRegisteredKey),
		#[allow(missing_docs)]
		NotSlashed(NotSlashed),
		#[allow(missing_docs)]
		NotUnregistered(NotUnregistered),
		#[allow(missing_docs)]
		OperatorAlreadyEquivocated(OperatorAlreadyEquivocated),
		#[allow(missing_docs)]
		OperatorAlreadyRegistered(OperatorAlreadyRegistered),
		#[allow(missing_docs)]
		OperatorAlreadyUnregistered(OperatorAlreadyUnregistered),
		#[allow(missing_docs)]
		OperatorDeleted(OperatorDeleted),
		#[allow(missing_docs)]
		OptInDelayNotMet(OptInDelayNotMet),
		#[allow(missing_docs)]
		SlashAmountExceedsCollateral(SlashAmountExceedsCollateral),
		#[allow(missing_docs)]
		SlashWindowExpired(SlashWindowExpired),
		#[allow(missing_docs)]
		SlashWindowNotMet(SlashWindowNotMet),
		#[allow(missing_docs)]
		SlashingAlreadyOccurred(SlashingAlreadyOccurred),
		#[allow(missing_docs)]
		TimestampTooOld(TimestampTooOld),
		#[allow(missing_docs)]
		UnauthorizedCommitment(UnauthorizedCommitment),
		#[allow(missing_docs)]
		UnregistrationDelayNotMet(UnregistrationDelayNotMet),
		#[allow(missing_docs)]
		WrongOperator(WrongOperator),
	}
	impl RegistryErrors {
		/// All the selectors of this enum.
		///
		/// Note that the selectors might not be in the same order as the variants.
		/// No guarantees are made about the order of the selectors.
		///
		/// Prefer using `SolInterface` methods instead.
		pub const SELECTORS: &'static [[u8; 4usize]] = &[
			[9u8, 189u8, 227u8, 57u8],
			[10u8, 241u8, 102u8, 46u8],
			[15u8, 134u8, 27u8, 192u8],
			[18u8, 139u8, 184u8, 109u8],
			[35u8, 213u8, 9u8, 230u8],
			[36u8, 112u8, 27u8, 23u8],
			[43u8, 59u8, 26u8, 221u8],
			[58u8, 35u8, 216u8, 37u8],
			[66u8, 238u8, 104u8, 181u8],
			[67u8, 34u8, 230u8, 226u8],
			[69u8, 13u8, 73u8, 205u8],
			[69u8, 212u8, 197u8, 162u8],
			[87u8, 209u8, 67u8, 164u8],
			[88u8, 86u8, 111u8, 95u8],
			[88u8, 161u8, 85u8, 240u8],
			[89u8, 20u8, 57u8, 176u8],
			[105u8, 97u8, 54u8, 114u8],
			[109u8, 150u8, 63u8, 136u8],
			[115u8, 86u8, 8u8, 202u8],
			[124u8, 191u8, 237u8, 45u8],
			[133u8, 93u8, 4u8, 136u8],
			[137u8, 41u8, 138u8, 128u8],
			[141u8, 200u8, 217u8, 179u8],
			[164u8, 63u8, 190u8, 184u8],
			[169u8, 230u8, 73u8, 233u8],
			[179u8, 112u8, 162u8, 49u8],
			[180u8, 18u8, 171u8, 228u8],
			[180u8, 165u8, 130u8, 195u8],
			[198u8, 83u8, 232u8, 127u8],
			[212u8, 15u8, 199u8, 75u8],
			[216u8, 167u8, 20u8, 33u8],
			[217u8, 36u8, 229u8, 244u8],
			[220u8, 222u8, 171u8, 163u8],
			[229u8, 188u8, 68u8, 109u8],
			[235u8, 64u8, 29u8, 130u8],
			[251u8, 15u8, 36u8, 68u8],
			[252u8, 159u8, 104u8, 169u8],
		];
		/// The names of the variants in the same order as `SELECTORS`.
		pub const VARIANT_NAMES: &'static [&'static str] = &[
			::core::stringify!(InvalidProof),
			::core::stringify!(SlashAmountExceedsCollateral),
			::core::stringify!(OperatorDeleted),
			::core::stringify!(NoCollateralSlashed),
			::core::stringify!(DifferentSlots),
			::core::stringify!(SlashingAlreadyOccurred),
			::core::stringify!(WrongOperator),
			::core::stringify!(InsufficientCollateral),
			::core::stringify!(OperatorAlreadyRegistered),
			::core::stringify!(OperatorAlreadyUnregistered),
			::core::stringify!(CollateralOverflow),
			::core::stringify!(SlashWindowNotMet),
			::core::stringify!(FraudProofWindowExpired),
			::core::stringify!(AlreadyUnregistered),
			::core::stringify!(DelegationsAreSame),
			::core::stringify!(NotUnregistered),
			::core::stringify!(NotOptedIn),
			::core::stringify!(EthTransferFailed),
			::core::stringify!(CollateralBelowMinimum),
			::core::stringify!(DelegationSignatureInvalid),
			::core::stringify!(OperatorAlreadyEquivocated),
			::core::stringify!(FraudProofMerklePathInvalid),
			::core::stringify!(NoCollateral),
			::core::stringify!(NotRegisteredKey),
			::core::stringify!(InvalidDelegation),
			::core::stringify!(InvalidRegistrationRoot),
			::core::stringify!(NotSlashed),
			::core::stringify!(UnauthorizedCommitment),
			::core::stringify!(FraudProofChallengeInvalid),
			::core::stringify!(TimestampTooOld),
			::core::stringify!(FraudProofWindowNotMet),
			::core::stringify!(InvalidOwnerAddress),
			::core::stringify!(AlreadyOptedIn),
			::core::stringify!(OptInDelayNotMet),
			::core::stringify!(SlashWindowExpired),
			::core::stringify!(UnregistrationDelayNotMet),
			::core::stringify!(NoCollateralToClaim),
		];
		/// The signatures in the same order as `SELECTORS`.
		pub const SIGNATURES: &'static [&'static str] = &[
			<InvalidProof as alloy_sol_types::SolError>::SIGNATURE,
			<SlashAmountExceedsCollateral as alloy_sol_types::SolError>::SIGNATURE,
			<OperatorDeleted as alloy_sol_types::SolError>::SIGNATURE,
			<NoCollateralSlashed as alloy_sol_types::SolError>::SIGNATURE,
			<DifferentSlots as alloy_sol_types::SolError>::SIGNATURE,
			<SlashingAlreadyOccurred as alloy_sol_types::SolError>::SIGNATURE,
			<WrongOperator as alloy_sol_types::SolError>::SIGNATURE,
			<InsufficientCollateral as alloy_sol_types::SolError>::SIGNATURE,
			<OperatorAlreadyRegistered as alloy_sol_types::SolError>::SIGNATURE,
			<OperatorAlreadyUnregistered as alloy_sol_types::SolError>::SIGNATURE,
			<CollateralOverflow as alloy_sol_types::SolError>::SIGNATURE,
			<SlashWindowNotMet as alloy_sol_types::SolError>::SIGNATURE,
			<FraudProofWindowExpired as alloy_sol_types::SolError>::SIGNATURE,
			<AlreadyUnregistered as alloy_sol_types::SolError>::SIGNATURE,
			<DelegationsAreSame as alloy_sol_types::SolError>::SIGNATURE,
			<NotUnregistered as alloy_sol_types::SolError>::SIGNATURE,
			<NotOptedIn as alloy_sol_types::SolError>::SIGNATURE,
			<EthTransferFailed as alloy_sol_types::SolError>::SIGNATURE,
			<CollateralBelowMinimum as alloy_sol_types::SolError>::SIGNATURE,
			<DelegationSignatureInvalid as alloy_sol_types::SolError>::SIGNATURE,
			<OperatorAlreadyEquivocated as alloy_sol_types::SolError>::SIGNATURE,
			<FraudProofMerklePathInvalid as alloy_sol_types::SolError>::SIGNATURE,
			<NoCollateral as alloy_sol_types::SolError>::SIGNATURE,
			<NotRegisteredKey as alloy_sol_types::SolError>::SIGNATURE,
			<InvalidDelegation as alloy_sol_types::SolError>::SIGNATURE,
			<InvalidRegistrationRoot as alloy_sol_types::SolError>::SIGNATURE,
			<NotSlashed as alloy_sol_types::SolError>::SIGNATURE,
			<UnauthorizedCommitment as alloy_sol_types::SolError>::SIGNATURE,
			<FraudProofChallengeInvalid as alloy_sol_types::SolError>::SIGNATURE,
			<TimestampTooOld as alloy_sol_types::SolError>::SIGNATURE,
			<FraudProofWindowNotMet as alloy_sol_types::SolError>::SIGNATURE,
			<InvalidOwnerAddress as alloy_sol_types::SolError>::SIGNATURE,
			<AlreadyOptedIn as alloy_sol_types::SolError>::SIGNATURE,
			<OptInDelayNotMet as alloy_sol_types::SolError>::SIGNATURE,
			<SlashWindowExpired as alloy_sol_types::SolError>::SIGNATURE,
			<UnregistrationDelayNotMet as alloy_sol_types::SolError>::SIGNATURE,
			<NoCollateralToClaim as alloy_sol_types::SolError>::SIGNATURE,
		];
		/// Returns the signature for the given selector, if known.
		#[inline]
		pub fn signature_by_selector(selector: [u8; 4usize]) -> ::core::option::Option<&'static str> {
			match Self::SELECTORS.binary_search(&selector) {
				::core::result::Result::Ok(idx) => ::core::option::Option::Some(Self::SIGNATURES[idx]),
				::core::result::Result::Err(_) => ::core::option::Option::None,
			}
		}
		/// Returns the enum variant name for the given selector, if known.
		#[inline]
		pub fn name_by_selector(selector: [u8; 4usize]) -> ::core::option::Option<&'static str> {
			let sig = Self::signature_by_selector(selector)?;
			sig.split_once('(').map(|(name, _)| name)
		}
	}
	#[automatically_derived]
	impl alloy_sol_types::SolInterface for RegistryErrors {
		const NAME: &'static str = "RegistryErrors";
		const MIN_DATA_LENGTH: usize = 0usize;
		const COUNT: usize = 37usize;
		#[inline]
		fn selector(&self) -> [u8; 4] {
			match self {
				Self::AlreadyOptedIn(_) => <AlreadyOptedIn as alloy_sol_types::SolError>::SELECTOR,
				Self::AlreadyUnregistered(_) => <AlreadyUnregistered as alloy_sol_types::SolError>::SELECTOR,
				Self::CollateralBelowMinimum(_) => <CollateralBelowMinimum as alloy_sol_types::SolError>::SELECTOR,
				Self::CollateralOverflow(_) => <CollateralOverflow as alloy_sol_types::SolError>::SELECTOR,
				Self::DelegationSignatureInvalid(_) => {
					<DelegationSignatureInvalid as alloy_sol_types::SolError>::SELECTOR
				}
				Self::DelegationsAreSame(_) => <DelegationsAreSame as alloy_sol_types::SolError>::SELECTOR,
				Self::DifferentSlots(_) => <DifferentSlots as alloy_sol_types::SolError>::SELECTOR,
				Self::EthTransferFailed(_) => <EthTransferFailed as alloy_sol_types::SolError>::SELECTOR,
				Self::FraudProofChallengeInvalid(_) => {
					<FraudProofChallengeInvalid as alloy_sol_types::SolError>::SELECTOR
				}
				Self::FraudProofMerklePathInvalid(_) => {
					<FraudProofMerklePathInvalid as alloy_sol_types::SolError>::SELECTOR
				}
				Self::FraudProofWindowExpired(_) => <FraudProofWindowExpired as alloy_sol_types::SolError>::SELECTOR,
				Self::FraudProofWindowNotMet(_) => <FraudProofWindowNotMet as alloy_sol_types::SolError>::SELECTOR,
				Self::InsufficientCollateral(_) => <InsufficientCollateral as alloy_sol_types::SolError>::SELECTOR,
				Self::InvalidDelegation(_) => <InvalidDelegation as alloy_sol_types::SolError>::SELECTOR,
				Self::InvalidOwnerAddress(_) => <InvalidOwnerAddress as alloy_sol_types::SolError>::SELECTOR,
				Self::InvalidProof(_) => <InvalidProof as alloy_sol_types::SolError>::SELECTOR,
				Self::InvalidRegistrationRoot(_) => <InvalidRegistrationRoot as alloy_sol_types::SolError>::SELECTOR,
				Self::NoCollateral(_) => <NoCollateral as alloy_sol_types::SolError>::SELECTOR,
				Self::NoCollateralSlashed(_) => <NoCollateralSlashed as alloy_sol_types::SolError>::SELECTOR,
				Self::NoCollateralToClaim(_) => <NoCollateralToClaim as alloy_sol_types::SolError>::SELECTOR,
				Self::NotOptedIn(_) => <NotOptedIn as alloy_sol_types::SolError>::SELECTOR,
				Self::NotRegisteredKey(_) => <NotRegisteredKey as alloy_sol_types::SolError>::SELECTOR,
				Self::NotSlashed(_) => <NotSlashed as alloy_sol_types::SolError>::SELECTOR,
				Self::NotUnregistered(_) => <NotUnregistered as alloy_sol_types::SolError>::SELECTOR,
				Self::OperatorAlreadyEquivocated(_) => {
					<OperatorAlreadyEquivocated as alloy_sol_types::SolError>::SELECTOR
				}
				Self::OperatorAlreadyRegistered(_) => {
					<OperatorAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
				}
				Self::OperatorAlreadyUnregistered(_) => {
					<OperatorAlreadyUnregistered as alloy_sol_types::SolError>::SELECTOR
				}
				Self::OperatorDeleted(_) => <OperatorDeleted as alloy_sol_types::SolError>::SELECTOR,
				Self::OptInDelayNotMet(_) => <OptInDelayNotMet as alloy_sol_types::SolError>::SELECTOR,
				Self::SlashAmountExceedsCollateral(_) => {
					<SlashAmountExceedsCollateral as alloy_sol_types::SolError>::SELECTOR
				}
				Self::SlashWindowExpired(_) => <SlashWindowExpired as alloy_sol_types::SolError>::SELECTOR,
				Self::SlashWindowNotMet(_) => <SlashWindowNotMet as alloy_sol_types::SolError>::SELECTOR,
				Self::SlashingAlreadyOccurred(_) => <SlashingAlreadyOccurred as alloy_sol_types::SolError>::SELECTOR,
				Self::TimestampTooOld(_) => <TimestampTooOld as alloy_sol_types::SolError>::SELECTOR,
				Self::UnauthorizedCommitment(_) => <UnauthorizedCommitment as alloy_sol_types::SolError>::SELECTOR,
				Self::UnregistrationDelayNotMet(_) => {
					<UnregistrationDelayNotMet as alloy_sol_types::SolError>::SELECTOR
				}
				Self::WrongOperator(_) => <WrongOperator as alloy_sol_types::SolError>::SELECTOR,
			}
		}
		#[inline]
		fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
			Self::SELECTORS.get(i).copied()
		}
		#[inline]
		fn valid_selector(selector: [u8; 4]) -> bool {
			Self::SELECTORS.binary_search(&selector).is_ok()
		}
		#[inline]
		#[allow(non_snake_case)]
		fn abi_decode_raw(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
			static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<RegistryErrors>] = &[
				{
					fn InvalidProof(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<InvalidProof as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::InvalidProof)
					}
					InvalidProof
				},
				{
					fn SlashAmountExceedsCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<SlashAmountExceedsCollateral as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::SlashAmountExceedsCollateral)
					}
					SlashAmountExceedsCollateral
				},
				{
					fn OperatorDeleted(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<OperatorDeleted as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::OperatorDeleted)
					}
					OperatorDeleted
				},
				{
					fn NoCollateralSlashed(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NoCollateralSlashed as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::NoCollateralSlashed)
					}
					NoCollateralSlashed
				},
				{
					fn DifferentSlots(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<DifferentSlots as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::DifferentSlots)
					}
					DifferentSlots
				},
				{
					fn SlashingAlreadyOccurred(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<SlashingAlreadyOccurred as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::SlashingAlreadyOccurred)
					}
					SlashingAlreadyOccurred
				},
				{
					fn WrongOperator(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<WrongOperator as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::WrongOperator)
					}
					WrongOperator
				},
				{
					fn InsufficientCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<InsufficientCollateral as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::InsufficientCollateral)
					}
					InsufficientCollateral
				},
				{
					fn OperatorAlreadyRegistered(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::OperatorAlreadyRegistered)
					}
					OperatorAlreadyRegistered
				},
				{
					fn OperatorAlreadyUnregistered(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<OperatorAlreadyUnregistered as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::OperatorAlreadyUnregistered)
					}
					OperatorAlreadyUnregistered
				},
				{
					fn CollateralOverflow(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<CollateralOverflow as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::CollateralOverflow)
					}
					CollateralOverflow
				},
				{
					fn SlashWindowNotMet(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<SlashWindowNotMet as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::SlashWindowNotMet)
					}
					SlashWindowNotMet
				},
				{
					fn FraudProofWindowExpired(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<FraudProofWindowExpired as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::FraudProofWindowExpired)
					}
					FraudProofWindowExpired
				},
				{
					fn AlreadyUnregistered(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<AlreadyUnregistered as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::AlreadyUnregistered)
					}
					AlreadyUnregistered
				},
				{
					fn DelegationsAreSame(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<DelegationsAreSame as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::DelegationsAreSame)
					}
					DelegationsAreSame
				},
				{
					fn NotUnregistered(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NotUnregistered as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::NotUnregistered)
					}
					NotUnregistered
				},
				{
					fn NotOptedIn(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NotOptedIn as alloy_sol_types::SolError>::abi_decode_raw(data).map(RegistryErrors::NotOptedIn)
					}
					NotOptedIn
				},
				{
					fn EthTransferFailed(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<EthTransferFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::EthTransferFailed)
					}
					EthTransferFailed
				},
				{
					fn CollateralBelowMinimum(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<CollateralBelowMinimum as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::CollateralBelowMinimum)
					}
					CollateralBelowMinimum
				},
				{
					fn DelegationSignatureInvalid(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<DelegationSignatureInvalid as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::DelegationSignatureInvalid)
					}
					DelegationSignatureInvalid
				},
				{
					fn OperatorAlreadyEquivocated(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<OperatorAlreadyEquivocated as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::OperatorAlreadyEquivocated)
					}
					OperatorAlreadyEquivocated
				},
				{
					fn FraudProofMerklePathInvalid(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<FraudProofMerklePathInvalid as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::FraudProofMerklePathInvalid)
					}
					FraudProofMerklePathInvalid
				},
				{
					fn NoCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NoCollateral as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::NoCollateral)
					}
					NoCollateral
				},
				{
					fn NotRegisteredKey(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NotRegisteredKey as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::NotRegisteredKey)
					}
					NotRegisteredKey
				},
				{
					fn InvalidDelegation(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<InvalidDelegation as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::InvalidDelegation)
					}
					InvalidDelegation
				},
				{
					fn InvalidRegistrationRoot(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<InvalidRegistrationRoot as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::InvalidRegistrationRoot)
					}
					InvalidRegistrationRoot
				},
				{
					fn NotSlashed(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NotSlashed as alloy_sol_types::SolError>::abi_decode_raw(data).map(RegistryErrors::NotSlashed)
					}
					NotSlashed
				},
				{
					fn UnauthorizedCommitment(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<UnauthorizedCommitment as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::UnauthorizedCommitment)
					}
					UnauthorizedCommitment
				},
				{
					fn FraudProofChallengeInvalid(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<FraudProofChallengeInvalid as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::FraudProofChallengeInvalid)
					}
					FraudProofChallengeInvalid
				},
				{
					fn TimestampTooOld(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<TimestampTooOld as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::TimestampTooOld)
					}
					TimestampTooOld
				},
				{
					fn FraudProofWindowNotMet(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<FraudProofWindowNotMet as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::FraudProofWindowNotMet)
					}
					FraudProofWindowNotMet
				},
				{
					fn InvalidOwnerAddress(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<InvalidOwnerAddress as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::InvalidOwnerAddress)
					}
					InvalidOwnerAddress
				},
				{
					fn AlreadyOptedIn(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<AlreadyOptedIn as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::AlreadyOptedIn)
					}
					AlreadyOptedIn
				},
				{
					fn OptInDelayNotMet(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<OptInDelayNotMet as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::OptInDelayNotMet)
					}
					OptInDelayNotMet
				},
				{
					fn SlashWindowExpired(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<SlashWindowExpired as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::SlashWindowExpired)
					}
					SlashWindowExpired
				},
				{
					fn UnregistrationDelayNotMet(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<UnregistrationDelayNotMet as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::UnregistrationDelayNotMet)
					}
					UnregistrationDelayNotMet
				},
				{
					fn NoCollateralToClaim(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NoCollateralToClaim as alloy_sol_types::SolError>::abi_decode_raw(data)
							.map(RegistryErrors::NoCollateralToClaim)
					}
					NoCollateralToClaim
				},
			];
			let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
				return Err(alloy_sol_types::Error::unknown_selector(
					<Self as alloy_sol_types::SolInterface>::NAME,
					selector,
				));
			};
			DECODE_SHIMS[idx](data)
		}
		#[inline]
		#[allow(non_snake_case)]
		fn abi_decode_raw_validate(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
			static DECODE_VALIDATE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<RegistryErrors>] = &[
				{
					fn InvalidProof(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<InvalidProof as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::InvalidProof)
					}
					InvalidProof
				},
				{
					fn SlashAmountExceedsCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<SlashAmountExceedsCollateral as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::SlashAmountExceedsCollateral)
					}
					SlashAmountExceedsCollateral
				},
				{
					fn OperatorDeleted(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<OperatorDeleted as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::OperatorDeleted)
					}
					OperatorDeleted
				},
				{
					fn NoCollateralSlashed(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NoCollateralSlashed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::NoCollateralSlashed)
					}
					NoCollateralSlashed
				},
				{
					fn DifferentSlots(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<DifferentSlots as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::DifferentSlots)
					}
					DifferentSlots
				},
				{
					fn SlashingAlreadyOccurred(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<SlashingAlreadyOccurred as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::SlashingAlreadyOccurred)
					}
					SlashingAlreadyOccurred
				},
				{
					fn WrongOperator(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<WrongOperator as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::WrongOperator)
					}
					WrongOperator
				},
				{
					fn InsufficientCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<InsufficientCollateral as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::InsufficientCollateral)
					}
					InsufficientCollateral
				},
				{
					fn OperatorAlreadyRegistered(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::OperatorAlreadyRegistered)
					}
					OperatorAlreadyRegistered
				},
				{
					fn OperatorAlreadyUnregistered(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<OperatorAlreadyUnregistered as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::OperatorAlreadyUnregistered)
					}
					OperatorAlreadyUnregistered
				},
				{
					fn CollateralOverflow(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<CollateralOverflow as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::CollateralOverflow)
					}
					CollateralOverflow
				},
				{
					fn SlashWindowNotMet(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<SlashWindowNotMet as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::SlashWindowNotMet)
					}
					SlashWindowNotMet
				},
				{
					fn FraudProofWindowExpired(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<FraudProofWindowExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::FraudProofWindowExpired)
					}
					FraudProofWindowExpired
				},
				{
					fn AlreadyUnregistered(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<AlreadyUnregistered as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::AlreadyUnregistered)
					}
					AlreadyUnregistered
				},
				{
					fn DelegationsAreSame(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<DelegationsAreSame as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::DelegationsAreSame)
					}
					DelegationsAreSame
				},
				{
					fn NotUnregistered(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NotUnregistered as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::NotUnregistered)
					}
					NotUnregistered
				},
				{
					fn NotOptedIn(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NotOptedIn as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::NotOptedIn)
					}
					NotOptedIn
				},
				{
					fn EthTransferFailed(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<EthTransferFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::EthTransferFailed)
					}
					EthTransferFailed
				},
				{
					fn CollateralBelowMinimum(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<CollateralBelowMinimum as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::CollateralBelowMinimum)
					}
					CollateralBelowMinimum
				},
				{
					fn DelegationSignatureInvalid(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<DelegationSignatureInvalid as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::DelegationSignatureInvalid)
					}
					DelegationSignatureInvalid
				},
				{
					fn OperatorAlreadyEquivocated(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<OperatorAlreadyEquivocated as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::OperatorAlreadyEquivocated)
					}
					OperatorAlreadyEquivocated
				},
				{
					fn FraudProofMerklePathInvalid(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<FraudProofMerklePathInvalid as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::FraudProofMerklePathInvalid)
					}
					FraudProofMerklePathInvalid
				},
				{
					fn NoCollateral(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NoCollateral as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::NoCollateral)
					}
					NoCollateral
				},
				{
					fn NotRegisteredKey(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NotRegisteredKey as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::NotRegisteredKey)
					}
					NotRegisteredKey
				},
				{
					fn InvalidDelegation(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<InvalidDelegation as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::InvalidDelegation)
					}
					InvalidDelegation
				},
				{
					fn InvalidRegistrationRoot(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<InvalidRegistrationRoot as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::InvalidRegistrationRoot)
					}
					InvalidRegistrationRoot
				},
				{
					fn NotSlashed(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NotSlashed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::NotSlashed)
					}
					NotSlashed
				},
				{
					fn UnauthorizedCommitment(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<UnauthorizedCommitment as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::UnauthorizedCommitment)
					}
					UnauthorizedCommitment
				},
				{
					fn FraudProofChallengeInvalid(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<FraudProofChallengeInvalid as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::FraudProofChallengeInvalid)
					}
					FraudProofChallengeInvalid
				},
				{
					fn TimestampTooOld(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<TimestampTooOld as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::TimestampTooOld)
					}
					TimestampTooOld
				},
				{
					fn FraudProofWindowNotMet(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<FraudProofWindowNotMet as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::FraudProofWindowNotMet)
					}
					FraudProofWindowNotMet
				},
				{
					fn InvalidOwnerAddress(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<InvalidOwnerAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::InvalidOwnerAddress)
					}
					InvalidOwnerAddress
				},
				{
					fn AlreadyOptedIn(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<AlreadyOptedIn as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::AlreadyOptedIn)
					}
					AlreadyOptedIn
				},
				{
					fn OptInDelayNotMet(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<OptInDelayNotMet as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::OptInDelayNotMet)
					}
					OptInDelayNotMet
				},
				{
					fn SlashWindowExpired(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<SlashWindowExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::SlashWindowExpired)
					}
					SlashWindowExpired
				},
				{
					fn UnregistrationDelayNotMet(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<UnregistrationDelayNotMet as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::UnregistrationDelayNotMet)
					}
					UnregistrationDelayNotMet
				},
				{
					fn NoCollateralToClaim(data: &[u8]) -> alloy_sol_types::Result<RegistryErrors> {
						<NoCollateralToClaim as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
							.map(RegistryErrors::NoCollateralToClaim)
					}
					NoCollateralToClaim
				},
			];
			let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
				return Err(alloy_sol_types::Error::unknown_selector(
					<Self as alloy_sol_types::SolInterface>::NAME,
					selector,
				));
			};
			DECODE_VALIDATE_SHIMS[idx](data)
		}
		#[inline]
		fn abi_encoded_size(&self) -> usize {
			match self {
				Self::AlreadyOptedIn(inner) => <AlreadyOptedIn as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::AlreadyUnregistered(inner) => {
					<AlreadyUnregistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::CollateralBelowMinimum(inner) => {
					<CollateralBelowMinimum as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::CollateralOverflow(inner) => {
					<CollateralOverflow as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::DelegationSignatureInvalid(inner) => {
					<DelegationSignatureInvalid as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::DelegationsAreSame(inner) => {
					<DelegationsAreSame as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::DifferentSlots(inner) => <DifferentSlots as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::EthTransferFailed(inner) => {
					<EthTransferFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::FraudProofChallengeInvalid(inner) => {
					<FraudProofChallengeInvalid as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::FraudProofMerklePathInvalid(inner) => {
					<FraudProofMerklePathInvalid as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::FraudProofWindowExpired(inner) => {
					<FraudProofWindowExpired as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::FraudProofWindowNotMet(inner) => {
					<FraudProofWindowNotMet as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::InsufficientCollateral(inner) => {
					<InsufficientCollateral as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::InvalidDelegation(inner) => {
					<InvalidDelegation as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::InvalidOwnerAddress(inner) => {
					<InvalidOwnerAddress as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::InvalidProof(inner) => <InvalidProof as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::InvalidRegistrationRoot(inner) => {
					<InvalidRegistrationRoot as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::NoCollateral(inner) => <NoCollateral as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::NoCollateralSlashed(inner) => {
					<NoCollateralSlashed as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::NoCollateralToClaim(inner) => {
					<NoCollateralToClaim as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::NotOptedIn(inner) => <NotOptedIn as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::NotRegisteredKey(inner) => {
					<NotRegisteredKey as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::NotSlashed(inner) => <NotSlashed as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::NotUnregistered(inner) => <NotUnregistered as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::OperatorAlreadyEquivocated(inner) => {
					<OperatorAlreadyEquivocated as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::OperatorAlreadyRegistered(inner) => {
					<OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::OperatorAlreadyUnregistered(inner) => {
					<OperatorAlreadyUnregistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::OperatorDeleted(inner) => <OperatorDeleted as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::OptInDelayNotMet(inner) => {
					<OptInDelayNotMet as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::SlashAmountExceedsCollateral(inner) => {
					<SlashAmountExceedsCollateral as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::SlashWindowExpired(inner) => {
					<SlashWindowExpired as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::SlashWindowNotMet(inner) => {
					<SlashWindowNotMet as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::SlashingAlreadyOccurred(inner) => {
					<SlashingAlreadyOccurred as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::TimestampTooOld(inner) => <TimestampTooOld as alloy_sol_types::SolError>::abi_encoded_size(inner),
				Self::UnauthorizedCommitment(inner) => {
					<UnauthorizedCommitment as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::UnregistrationDelayNotMet(inner) => {
					<UnregistrationDelayNotMet as alloy_sol_types::SolError>::abi_encoded_size(inner)
				}
				Self::WrongOperator(inner) => <WrongOperator as alloy_sol_types::SolError>::abi_encoded_size(inner),
			}
		}
		#[inline]
		fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
			match self {
				Self::AlreadyOptedIn(inner) => {
					<AlreadyOptedIn as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::AlreadyUnregistered(inner) => {
					<AlreadyUnregistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::CollateralBelowMinimum(inner) => {
					<CollateralBelowMinimum as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::CollateralOverflow(inner) => {
					<CollateralOverflow as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::DelegationSignatureInvalid(inner) => {
					<DelegationSignatureInvalid as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::DelegationsAreSame(inner) => {
					<DelegationsAreSame as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::DifferentSlots(inner) => {
					<DifferentSlots as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::EthTransferFailed(inner) => {
					<EthTransferFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::FraudProofChallengeInvalid(inner) => {
					<FraudProofChallengeInvalid as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::FraudProofMerklePathInvalid(inner) => {
					<FraudProofMerklePathInvalid as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::FraudProofWindowExpired(inner) => {
					<FraudProofWindowExpired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::FraudProofWindowNotMet(inner) => {
					<FraudProofWindowNotMet as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::InsufficientCollateral(inner) => {
					<InsufficientCollateral as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::InvalidDelegation(inner) => {
					<InvalidDelegation as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::InvalidOwnerAddress(inner) => {
					<InvalidOwnerAddress as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::InvalidProof(inner) => <InvalidProof as alloy_sol_types::SolError>::abi_encode_raw(inner, out),
				Self::InvalidRegistrationRoot(inner) => {
					<InvalidRegistrationRoot as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::NoCollateral(inner) => <NoCollateral as alloy_sol_types::SolError>::abi_encode_raw(inner, out),
				Self::NoCollateralSlashed(inner) => {
					<NoCollateralSlashed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::NoCollateralToClaim(inner) => {
					<NoCollateralToClaim as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::NotOptedIn(inner) => <NotOptedIn as alloy_sol_types::SolError>::abi_encode_raw(inner, out),
				Self::NotRegisteredKey(inner) => {
					<NotRegisteredKey as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::NotSlashed(inner) => <NotSlashed as alloy_sol_types::SolError>::abi_encode_raw(inner, out),
				Self::NotUnregistered(inner) => {
					<NotUnregistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::OperatorAlreadyEquivocated(inner) => {
					<OperatorAlreadyEquivocated as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::OperatorAlreadyRegistered(inner) => {
					<OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::OperatorAlreadyUnregistered(inner) => {
					<OperatorAlreadyUnregistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::OperatorDeleted(inner) => {
					<OperatorDeleted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::OptInDelayNotMet(inner) => {
					<OptInDelayNotMet as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::SlashAmountExceedsCollateral(inner) => {
					<SlashAmountExceedsCollateral as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::SlashWindowExpired(inner) => {
					<SlashWindowExpired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::SlashWindowNotMet(inner) => {
					<SlashWindowNotMet as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::SlashingAlreadyOccurred(inner) => {
					<SlashingAlreadyOccurred as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::TimestampTooOld(inner) => {
					<TimestampTooOld as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::UnauthorizedCommitment(inner) => {
					<UnauthorizedCommitment as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::UnregistrationDelayNotMet(inner) => {
					<UnregistrationDelayNotMet as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
				}
				Self::WrongOperator(inner) => <WrongOperator as alloy_sol_types::SolError>::abi_encode_raw(inner, out),
			}
		}
	}
	///Container for all the [`Registry`](self) events.
	#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
	pub enum RegistryEvents {
		#[allow(missing_docs)]
		CollateralAdded(CollateralAdded),
		#[allow(missing_docs)]
		CollateralClaimed(CollateralClaimed),
		#[allow(missing_docs)]
		OperatorOptedIn(OperatorOptedIn),
		#[allow(missing_docs)]
		OperatorOptedOut(OperatorOptedOut),
		#[allow(missing_docs)]
		OperatorRegistered(OperatorRegistered),
		#[allow(missing_docs)]
		OperatorSlashed(OperatorSlashed),
		#[allow(missing_docs)]
		OperatorUnregistered(OperatorUnregistered),
	}
	impl RegistryEvents {
		/// All the selectors of this enum.
		///
		/// Note that the selectors might not be in the same order as the variants.
		/// No guarantees are made about the order of the selectors.
		///
		/// Prefer using `SolInterface` methods instead.
		pub const SELECTORS: &'static [[u8; 32usize]] = &[
			[
				39u8, 215u8, 219u8, 169u8, 230u8, 168u8, 192u8, 45u8, 203u8, 185u8, 183u8, 61u8, 131u8, 87u8, 59u8,
				61u8, 184u8, 110u8, 152u8, 238u8, 92u8, 254u8, 147u8, 232u8, 136u8, 34u8, 89u8, 192u8, 176u8, 247u8,
				250u8, 7u8,
			],
			[
				133u8, 237u8, 220u8, 42u8, 87u8, 89u8, 178u8, 218u8, 24u8, 132u8, 235u8, 64u8, 205u8, 171u8, 59u8,
				60u8, 172u8, 125u8, 162u8, 195u8, 199u8, 194u8, 72u8, 81u8, 53u8, 31u8, 104u8, 46u8, 148u8, 233u8,
				156u8, 42u8,
			],
			[
				144u8, 171u8, 83u8, 202u8, 239u8, 62u8, 219u8, 17u8, 57u8, 171u8, 57u8, 93u8, 210u8, 134u8, 160u8,
				16u8, 198u8, 228u8, 188u8, 122u8, 130u8, 89u8, 70u8, 144u8, 28u8, 120u8, 48u8, 12u8, 123u8, 58u8,
				171u8, 207u8,
			],
			[
				179u8, 102u8, 222u8, 105u8, 142u8, 90u8, 25u8, 6u8, 173u8, 113u8, 37u8, 97u8, 155u8, 85u8, 32u8, 224u8,
				27u8, 192u8, 198u8, 167u8, 225u8, 173u8, 221u8, 0u8, 137u8, 19u8, 92u8, 67u8, 69u8, 169u8, 230u8,
				143u8,
			],
			[
				184u8, 71u8, 117u8, 156u8, 36u8, 50u8, 159u8, 134u8, 201u8, 2u8, 111u8, 173u8, 188u8, 203u8, 190u8,
				185u8, 36u8, 31u8, 83u8, 90u8, 112u8, 154u8, 83u8, 254u8, 149u8, 236u8, 204u8, 96u8, 106u8, 121u8,
				211u8, 0u8,
			],
			[
				204u8, 84u8, 24u8, 118u8, 245u8, 203u8, 145u8, 108u8, 112u8, 154u8, 62u8, 64u8, 210u8, 241u8, 81u8,
				155u8, 166u8, 177u8, 232u8, 159u8, 60u8, 152u8, 240u8, 121u8, 149u8, 69u8, 207u8, 52u8, 108u8, 28u8,
				50u8, 60u8,
			],
			[
				227u8, 62u8, 148u8, 75u8, 242u8, 150u8, 200u8, 156u8, 202u8, 184u8, 22u8, 17u8, 59u8, 49u8, 84u8,
				248u8, 165u8, 56u8, 88u8, 157u8, 75u8, 229u8, 107u8, 54u8, 137u8, 56u8, 237u8, 195u8, 3u8, 56u8, 141u8,
				32u8,
			],
		];
		/// The names of the variants in the same order as `SELECTORS`.
		pub const VARIANT_NAMES: &'static [&'static str] = &[
			::core::stringify!(OperatorUnregistered),
			::core::stringify!(CollateralClaimed),
			::core::stringify!(OperatorSlashed),
			::core::stringify!(OperatorRegistered),
			::core::stringify!(OperatorOptedIn),
			::core::stringify!(OperatorOptedOut),
			::core::stringify!(CollateralAdded),
		];
		/// The signatures in the same order as `SELECTORS`.
		pub const SIGNATURES: &'static [&'static str] = &[
			<OperatorUnregistered as alloy_sol_types::SolEvent>::SIGNATURE,
			<CollateralClaimed as alloy_sol_types::SolEvent>::SIGNATURE,
			<OperatorSlashed as alloy_sol_types::SolEvent>::SIGNATURE,
			<OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE,
			<OperatorOptedIn as alloy_sol_types::SolEvent>::SIGNATURE,
			<OperatorOptedOut as alloy_sol_types::SolEvent>::SIGNATURE,
			<CollateralAdded as alloy_sol_types::SolEvent>::SIGNATURE,
		];
		/// Returns the signature for the given selector, if known.
		#[inline]
		pub fn signature_by_selector(selector: [u8; 32usize]) -> ::core::option::Option<&'static str> {
			match Self::SELECTORS.binary_search(&selector) {
				::core::result::Result::Ok(idx) => ::core::option::Option::Some(Self::SIGNATURES[idx]),
				::core::result::Result::Err(_) => ::core::option::Option::None,
			}
		}
		/// Returns the enum variant name for the given selector, if known.
		#[inline]
		pub fn name_by_selector(selector: [u8; 32usize]) -> ::core::option::Option<&'static str> {
			let sig = Self::signature_by_selector(selector)?;
			sig.split_once('(').map(|(name, _)| name)
		}
	}
	#[automatically_derived]
	impl alloy_sol_types::SolEventInterface for RegistryEvents {
		const NAME: &'static str = "RegistryEvents";
		const COUNT: usize = 7usize;
		fn decode_raw_log(topics: &[alloy_sol_types::Word], data: &[u8]) -> alloy_sol_types::Result<Self> {
			match topics.first().copied() {
				Some(<CollateralAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
					<CollateralAdded as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
						.map(Self::CollateralAdded)
				}
				Some(<CollateralClaimed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
					<CollateralClaimed as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
						.map(Self::CollateralClaimed)
				}
				Some(<OperatorOptedIn as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
					<OperatorOptedIn as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
						.map(Self::OperatorOptedIn)
				}
				Some(<OperatorOptedOut as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
					<OperatorOptedOut as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
						.map(Self::OperatorOptedOut)
				}
				Some(<OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
					<OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
						.map(Self::OperatorRegistered)
				}
				Some(<OperatorSlashed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
					<OperatorSlashed as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
						.map(Self::OperatorSlashed)
				}
				Some(<OperatorUnregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
					<OperatorUnregistered as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
						.map(Self::OperatorUnregistered)
				}
				_ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
					name: <Self as alloy_sol_types::SolEventInterface>::NAME,
					log: alloy_sol_types::private::Box::new(alloy_sol_types::private::LogData::new_unchecked(
						topics.to_vec(),
						data.to_vec().into(),
					)),
				}),
			}
		}
	}
	#[automatically_derived]
	impl alloy_sol_types::private::IntoLogData for RegistryEvents {
		fn to_log_data(&self) -> alloy_sol_types::private::LogData {
			match self {
				Self::CollateralAdded(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
				Self::CollateralClaimed(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
				Self::OperatorOptedIn(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
				Self::OperatorOptedOut(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
				Self::OperatorRegistered(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
				Self::OperatorSlashed(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
				Self::OperatorUnregistered(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
			}
		}
		fn into_log_data(self) -> alloy_sol_types::private::LogData {
			match self {
				Self::CollateralAdded(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
				Self::CollateralClaimed(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
				Self::OperatorOptedIn(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
				Self::OperatorOptedOut(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
				Self::OperatorRegistered(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
				Self::OperatorSlashed(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
				Self::OperatorUnregistered(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
			}
		}
	}
	use alloy::contract as alloy_contract;
	/**Creates a new wrapper around an on-chain [`Registry`](self) contract instance.

	See the [wrapper's documentation](`RegistryInstance`) for more details.*/
	#[inline]
	pub const fn new<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
		address: alloy_sol_types::private::Address,
		__provider: P,
	) -> RegistryInstance<P, N> {
		RegistryInstance::<P, N>::new(address, __provider)
	}
	/**A [`Registry`](self) instance.

	Contains type-safe methods for interacting with an on-chain instance of the
	[`Registry`](self) contract located at a given `address`, using a given
	provider `P`.

	If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
	documentation on how to provide it), the `deploy` and `deploy_builder` methods can
	be used to deploy a new instance of the contract.

	See the [module-level documentation](self) for all the available methods.*/
	#[derive(Clone)]
	pub struct RegistryInstance<P, N = alloy_contract::private::Ethereum> {
		address: alloy_sol_types::private::Address,
		provider: P,
		_network: ::core::marker::PhantomData<N>,
	}
	#[automatically_derived]
	impl<P, N> ::core::fmt::Debug for RegistryInstance<P, N> {
		#[inline]
		fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
			f.debug_tuple("RegistryInstance").field(&self.address).finish()
		}
	}
	/// Instantiation and getters/setters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> RegistryInstance<P, N> {
		/**Creates a new wrapper around an on-chain [`Registry`](self) contract instance.

		See the [wrapper's documentation](`RegistryInstance`) for more details.*/
		#[inline]
		pub const fn new(address: alloy_sol_types::private::Address, __provider: P) -> Self {
			Self { address, provider: __provider, _network: ::core::marker::PhantomData }
		}
		/// Returns a reference to the address.
		#[inline]
		pub const fn address(&self) -> &alloy_sol_types::private::Address {
			&self.address
		}
		/// Sets the address.
		#[inline]
		pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
			self.address = address;
		}
		/// Sets the address and returns `self`.
		pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
			self.set_address(address);
			self
		}
		/// Returns a reference to the provider.
		#[inline]
		pub const fn provider(&self) -> &P {
			&self.provider
		}
	}
	impl<P: ::core::clone::Clone, N> RegistryInstance<&P, N> {
		/// Clones the provider and returns a new instance with the cloned provider.
		#[inline]
		pub fn with_cloned_provider(self) -> RegistryInstance<P, N> {
			RegistryInstance {
				address: self.address,
				provider: ::core::clone::Clone::clone(&self.provider),
				_network: ::core::marker::PhantomData,
			}
		}
	}
	/// Function calls.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> RegistryInstance<P, N> {
		/// Creates a new call builder using this contract instance's provider and address.
		///
		/// Note that the call can be any function call, not just those defined in this
		/// contract. Prefer using the other methods for building type-safe contract calls.
		pub fn call_builder<C: alloy_sol_types::SolCall>(&self, call: &C) -> alloy_contract::SolCallBuilder<&P, C, N> {
			alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
		}
		///Creates a new call builder for the [`addCollateral`] function.
		pub fn addCollateral(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, addCollateralCall, N> {
			self.call_builder(&addCollateralCall { registrationRoot })
		}
		///Creates a new call builder for the [`claimCollateral`] function.
		pub fn claimCollateral(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, claimCollateralCall, N> {
			self.call_builder(&claimCollateralCall { registrationRoot })
		}
		///Creates a new call builder for the [`claimSlashedCollateral`] function.
		pub fn claimSlashedCollateral(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, claimSlashedCollateralCall, N> {
			self.call_builder(&claimSlashedCollateralCall { registrationRoot })
		}
		///Creates a new call builder for the [`getConfig`] function.
		pub fn getConfig(&self) -> alloy_contract::SolCallBuilder<&P, getConfigCall, N> {
			self.call_builder(&getConfigCall)
		}
		///Creates a new call builder for the [`getHistoricalCollateral`] function.
		pub fn getHistoricalCollateral(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
			timestamp: alloy::sol_types::private::primitives::aliases::U256,
		) -> alloy_contract::SolCallBuilder<&P, getHistoricalCollateralCall, N> {
			self.call_builder(&getHistoricalCollateralCall { registrationRoot, timestamp })
		}
		///Creates a new call builder for the [`getOperatorData`] function.
		pub fn getOperatorData(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, getOperatorDataCall, N> {
			self.call_builder(&getOperatorDataCall { registrationRoot })
		}
		///Creates a new call builder for the [`getRegistrationProof`] function.
		pub fn getRegistrationProof(
			&self,
			regs: alloy::sol_types::private::Vec<
				<IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType,
			>,
			owner: alloy::sol_types::private::Address,
			leafIndex: alloy::sol_types::private::primitives::aliases::U256,
			signingId: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, getRegistrationProofCall, N> {
			self.call_builder(&getRegistrationProofCall { regs, owner, leafIndex, signingId })
		}
		///Creates a new call builder for the [`getSlasherCommitment`] function.
		pub fn getSlasherCommitment(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
			slasher: alloy::sol_types::private::Address,
		) -> alloy_contract::SolCallBuilder<&P, getSlasherCommitmentCall, N> {
			self.call_builder(&getSlasherCommitmentCall { registrationRoot, slasher })
		}
		///Creates a new call builder for the [`getVerifiedOperatorData`] function.
		pub fn getVerifiedOperatorData(
			&self,
			proof: <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
		) -> alloy_contract::SolCallBuilder<&P, getVerifiedOperatorDataCall, N> {
			self.call_builder(&getVerifiedOperatorDataCall { proof })
		}
		///Creates a new call builder for the [`isOptedIntoSlasher`] function.
		pub fn isOptedIntoSlasher(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
			slasher: alloy::sol_types::private::Address,
		) -> alloy_contract::SolCallBuilder<&P, isOptedIntoSlasherCall, N> {
			self.call_builder(&isOptedIntoSlasherCall { registrationRoot, slasher })
		}
		///Creates a new call builder for the [`isSlashed_0`] function.
		pub fn isSlashed_0(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, isSlashed_0Call, N> {
			self.call_builder(&isSlashed_0Call { registrationRoot })
		}
		///Creates a new call builder for the [`isSlashed_1`] function.
		pub fn isSlashed_1(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
			slasher: alloy::sol_types::private::Address,
		) -> alloy_contract::SolCallBuilder<&P, isSlashed_1Call, N> {
			self.call_builder(&isSlashed_1Call { registrationRoot, slasher })
		}
		///Creates a new call builder for the [`optInToSlasher`] function.
		pub fn optInToSlasher(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
			slasher: alloy::sol_types::private::Address,
			committer: alloy::sol_types::private::Address,
		) -> alloy_contract::SolCallBuilder<&P, optInToSlasherCall, N> {
			self.call_builder(&optInToSlasherCall { registrationRoot, slasher, committer })
		}
		///Creates a new call builder for the [`optOutOfSlasher`] function.
		pub fn optOutOfSlasher(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
			slasher: alloy::sol_types::private::Address,
		) -> alloy_contract::SolCallBuilder<&P, optOutOfSlasherCall, N> {
			self.call_builder(&optOutOfSlasherCall { registrationRoot, slasher })
		}
		///Creates a new call builder for the [`register`] function.
		pub fn register(
			&self,
			registrations: alloy::sol_types::private::Vec<
				<IRegistry::SignedRegistration as alloy::sol_types::SolType>::RustType,
			>,
			owner: alloy::sol_types::private::Address,
			signingId: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, registerCall, N> {
			self.call_builder(&registerCall { registrations, owner, signingId })
		}
		///Creates a new call builder for the [`slashCommitment_0`] function.
		pub fn slashCommitment_0(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
			commitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
			evidence: alloy::sol_types::private::Bytes,
		) -> alloy_contract::SolCallBuilder<&P, slashCommitment_0Call, N> {
			self.call_builder(&slashCommitment_0Call { registrationRoot, commitment, evidence })
		}
		///Creates a new call builder for the [`slashCommitment_1`] function.
		pub fn slashCommitment_1(
			&self,
			proof: <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
			delegation: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
			commitment: <ISlasher::SignedCommitment as alloy::sol_types::SolType>::RustType,
			evidence: alloy::sol_types::private::Bytes,
		) -> alloy_contract::SolCallBuilder<&P, slashCommitment_1Call, N> {
			self.call_builder(&slashCommitment_1Call { proof, delegation, commitment, evidence })
		}
		///Creates a new call builder for the [`slashEquivocation`] function.
		pub fn slashEquivocation(
			&self,
			proof: <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
			delegationOne: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
			delegationTwo: <ISlasher::SignedDelegation as alloy::sol_types::SolType>::RustType,
		) -> alloy_contract::SolCallBuilder<&P, slashEquivocationCall, N> {
			self.call_builder(&slashEquivocationCall { proof, delegationOne, delegationTwo })
		}
		///Creates a new call builder for the [`slashRegistration`] function.
		pub fn slashRegistration(
			&self,
			proof: <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
		) -> alloy_contract::SolCallBuilder<&P, slashRegistrationCall, N> {
			self.call_builder(&slashRegistrationCall { proof })
		}
		///Creates a new call builder for the [`slashingEvidenceAlreadyUsed`] function.
		pub fn slashingEvidenceAlreadyUsed(
			&self,
			slashingDigest: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, slashingEvidenceAlreadyUsedCall, N> {
			self.call_builder(&slashingEvidenceAlreadyUsedCall { slashingDigest })
		}
		///Creates a new call builder for the [`unregister`] function.
		pub fn unregister(
			&self,
			registrationRoot: alloy::sol_types::private::FixedBytes<32>,
		) -> alloy_contract::SolCallBuilder<&P, unregisterCall, N> {
			self.call_builder(&unregisterCall { registrationRoot })
		}
		///Creates a new call builder for the [`verifyMerkleProof`] function.
		pub fn verifyMerkleProof(
			&self,
			proof: <IRegistry::RegistrationProof as alloy::sol_types::SolType>::RustType,
		) -> alloy_contract::SolCallBuilder<&P, verifyMerkleProofCall, N> {
			self.call_builder(&verifyMerkleProofCall { proof })
		}
	}
	/// Event filters.
	impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network> RegistryInstance<P, N> {
		/// Creates a new event filter using this contract instance's provider and address.
		///
		/// Note that the type can be any event, not just those defined in this contract.
		/// Prefer using the other methods for building type-safe event filters.
		pub fn event_filter<E: alloy_sol_types::SolEvent>(&self) -> alloy_contract::Event<&P, E, N> {
			alloy_contract::Event::new_sol(&self.provider, &self.address)
		}
		///Creates a new event filter for the [`CollateralAdded`] event.
		pub fn CollateralAdded_filter(&self) -> alloy_contract::Event<&P, CollateralAdded, N> {
			self.event_filter::<CollateralAdded>()
		}
		///Creates a new event filter for the [`CollateralClaimed`] event.
		pub fn CollateralClaimed_filter(&self) -> alloy_contract::Event<&P, CollateralClaimed, N> {
			self.event_filter::<CollateralClaimed>()
		}
		///Creates a new event filter for the [`OperatorOptedIn`] event.
		pub fn OperatorOptedIn_filter(&self) -> alloy_contract::Event<&P, OperatorOptedIn, N> {
			self.event_filter::<OperatorOptedIn>()
		}
		///Creates a new event filter for the [`OperatorOptedOut`] event.
		pub fn OperatorOptedOut_filter(&self) -> alloy_contract::Event<&P, OperatorOptedOut, N> {
			self.event_filter::<OperatorOptedOut>()
		}
		///Creates a new event filter for the [`OperatorRegistered`] event.
		pub fn OperatorRegistered_filter(&self) -> alloy_contract::Event<&P, OperatorRegistered, N> {
			self.event_filter::<OperatorRegistered>()
		}
		///Creates a new event filter for the [`OperatorSlashed`] event.
		pub fn OperatorSlashed_filter(&self) -> alloy_contract::Event<&P, OperatorSlashed, N> {
			self.event_filter::<OperatorSlashed>()
		}
		///Creates a new event filter for the [`OperatorUnregistered`] event.
		pub fn OperatorUnregistered_filter(&self) -> alloy_contract::Event<&P, OperatorUnregistered, N> {
			self.event_filter::<OperatorUnregistered>()
		}
	}
}
