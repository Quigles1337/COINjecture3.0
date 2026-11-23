//! The block layout (Protocol Spec §6.3, ruling G0-C3f).
//!
//! A block is: the header, a `u32` solution length, the solution bytes, a `u32`
//! transaction count, the transactions. The solution length must equal `2 * m` for the
//! header's `size_param`. The whole must not exceed `BLOCK_MAX_BYTES`.
//!
//! The solution is opaque bytes here; its coefficient coding lives with the class. No
//! tuple is compiled in as active: the codec takes the size parameter that stands for
//! the active tuple as an argument (ruling G0-C8e), and uses it only for the P-8 limit
//! `SOL_MAX_BYTES`. It does not compare the header's size parameter with it. That
//! comparison is block rule B3, which belongs to consensus.
//!
//! The order of checks is fixed in `loop/reports/C10-p007-builder.md` §1.5, as clause
//! 2.2 of the P-007 resume ruling asks.

use crate::{
    class::SizeParam,
    codec::{DecodeError, Reader, Writer, len_u64},
    header::Header,
    tx::Transaction,
};

/// `BLOCK_MAX_BYTES` (P-8, ruling G0-C6c). **PROVISIONAL.**
///
/// No evidence stands behind this number. It is a conservative round figure so that
/// decoders have a bound, and Al re-rules it with P-1.
pub const BLOCK_MAX_BYTES: usize = 1_048_576;

/// Width of the two counts in a block.
const COUNT_LEN: usize = 4;

/// A block as its layout carries it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    /// The header.
    pub header: Header,
    /// The solution bytes, opaque here.
    pub solution: Vec<u8>,
    /// The transactions, in order.
    pub transactions: Vec<Transaction>,
}

/// Why a block value has no encoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EncodeError {
    /// The whole encoding would exceed its limit.
    WholeExceedsLimit {
        /// Length the encoding would have.
        length: u64,
        /// The limit it must not exceed.
        limit: u64,
    },
    /// The solution is longer than `SOL_MAX_BYTES` for the active size parameter.
    SolutionExceedsLimit {
        /// Length of the solution.
        length: u64,
        /// The limit it must not exceed.
        limit: u64,
    },
    /// The solution length is not the exact length the header's size parameter gives.
    SolutionLengthMismatch {
        /// The exact length the layout gives.
        expected: u64,
        /// The length found.
        actual: u64,
    },
}

impl Block {
    /// Encodes the block, or reports why this value has no encoding.
    ///
    /// `active` is the size parameter that stands for the active tuple. A value that
    /// encodes is exactly a value that [`Self::decode`] returns for the same `active`.
    ///
    /// # Errors
    ///
    /// Returns [`EncodeError`] when the whole would exceed [`BLOCK_MAX_BYTES`], when
    /// the solution is longer than `SOL_MAX_BYTES`, or when its length is not the exact
    /// length the header's size parameter gives.
    pub fn encode(&self, active: &SizeParam) -> Result<Vec<u8>, EncodeError> {
        self.encode_with_limit(active, BLOCK_MAX_BYTES)
    }

    fn encode_with_limit(
        &self,
        active: &SizeParam,
        block_max_bytes: usize,
    ) -> Result<Vec<u8>, EncodeError> {
        let solution_len = len_u64(self.solution.len());
        let transactions_len =
            len_u64(self.transactions.len()).saturating_mul(len_u64(Transaction::ENCODED_LEN));
        let whole = len_u64(self.header.encoded_len())
            .saturating_add(len_u64(COUNT_LEN))
            .saturating_add(solution_len)
            .saturating_add(len_u64(COUNT_LEN))
            .saturating_add(transactions_len);
        let too_long = EncodeError::WholeExceedsLimit {
            length: whole,
            limit: len_u64(block_max_bytes),
        };
        if whole > len_u64(block_max_bytes) {
            return Err(too_long);
        }
        if solution_len > active.solution_len() {
            return Err(EncodeError::SolutionExceedsLimit {
                length: solution_len,
                limit: active.solution_len(),
            });
        }
        let expected = self.header.size_param.solution_len();
        if solution_len != expected {
            return Err(EncodeError::SolutionLengthMismatch {
                expected,
                actual: solution_len,
            });
        }
        // Both counts are bounded by `whole`, which is within the limit, so on every
        // supported limit they fit a `u32`. The conversions are still checked.
        let solution_count = u32::try_from(self.solution.len()).map_err(|_| too_long)?;
        let transaction_count = u32::try_from(self.transactions.len()).map_err(|_| too_long)?;
        let capacity = usize::try_from(whole).map_err(|_| too_long)?;

        let mut writer = Writer::with_capacity(capacity)
            .bytes(&self.header.encode())
            .u32(solution_count)
            .bytes(&self.solution)
            .u32(transaction_count);
        for transaction in &self.transactions {
            writer = writer.bytes(&transaction.encode());
        }
        Ok(writer.finish())
    }

    /// Decodes an input that is exactly one block.
    ///
    /// `active` is the size parameter that stands for the active tuple; it supplies
    /// `SOL_MAX_BYTES = 2 * m`. Nothing is allocated for a variable-length part until
    /// its count has been checked against its limit and against the bytes that remain.
    ///
    /// # Errors
    ///
    /// Returns, in this order of checks: [`DecodeError::WholeExceedsLimit`] before any
    /// field is read; [`DecodeError::TooShort`] or [`DecodeError::UnknownClassId`] from
    /// the header; for the solution length, [`DecodeError::CountExceedsLimit`], then
    /// [`DecodeError::SolutionLengthMismatch`], then [`DecodeError::TooShort`]; and for
    /// the transaction count, [`DecodeError::TooShort`] when it needs more bytes than
    /// remain and [`DecodeError::TrailingBytes`] when it needs fewer.
    pub fn decode(input: &[u8], active: &SizeParam) -> Result<Self, DecodeError> {
        Self::decode_with_limit(input, active, BLOCK_MAX_BYTES)
    }

    fn decode_with_limit(
        input: &[u8],
        active: &SizeParam,
        block_max_bytes: usize,
    ) -> Result<Self, DecodeError> {
        // 1. The whole, before any field is read.
        if input.len() > block_max_bytes {
            return Err(DecodeError::WholeExceedsLimit {
                length: len_u64(input.len()),
                limit: len_u64(block_max_bytes),
            });
        }
        let mut reader = Reader::new(input);

        // 2. The header, field by field.
        let header = Header::read(&mut reader)?;

        // 3. The solution length: its limit, its exact value, the bytes that remain.
        let solution_len = u64::from(reader.u32()?);
        let limit = active.solution_len();
        if solution_len > limit {
            return Err(DecodeError::CountExceedsLimit {
                count: solution_len,
                limit,
            });
        }
        let expected = header.size_param.solution_len();
        if solution_len != expected {
            return Err(DecodeError::SolutionLengthMismatch {
                expected,
                actual: solution_len,
            });
        }
        let solution = Self::read_counted_bytes(&mut reader, solution_len)?.to_vec();

        // 4. The transaction count: no limit of its own; the bytes that remain must
        //    equal 188 times the count.
        let transaction_count = u64::from(reader.u32()?);
        let needed = transaction_count.saturating_mul(len_u64(Transaction::ENCODED_LEN));
        let remaining = len_u64(reader.remaining());
        if needed > remaining {
            return Err(DecodeError::TooShort { needed, remaining });
        }
        if needed < remaining {
            return Err(DecodeError::TrailingBytes {
                extra: remaining - needed,
            });
        }

        // 5. The transactions. `needed == remaining`, so the count is bounded by the
        //    input length and the capacity below is too.
        let capacity = reader.remaining() / Transaction::ENCODED_LEN;
        let mut transactions = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            transactions.push(Transaction::read(&mut reader)?);
        }
        reader.finish()?;

        Ok(Self {
            header,
            solution,
            transactions,
        })
    }

    fn read_counted_bytes<'a>(
        reader: &mut Reader<'a>,
        count: u64,
    ) -> Result<&'a [u8], DecodeError> {
        let too_short = DecodeError::TooShort {
            needed: count,
            remaining: len_u64(reader.remaining()),
        };
        let count = usize::try_from(count).map_err(|_| too_short)?;
        reader.bytes(count)
    }
}

#[cfg(test)]
mod tests {
    use super::{BLOCK_MAX_BYTES, Block, EncodeError};
    use crate::{
        Amount,
        class::{SisSizeParam, SizeParam},
        codec::DecodeError,
        header::Header,
        keys::{Address, PublicKey, Signature},
        tx::Transaction,
    };

    /// NON-NORMATIVE stand-ins for the active tuple (ruling G0-C8e; P-007 resume
    /// ruling, clause 2.3). Both are tuples already measured in `bench/p003-sis`. The
    /// choice makes neither a candidate for P-4, and no library code names a tuple.
    const NON_NORMATIVE_STAND_INS: [SisSizeParam; 2] = [
        SisSizeParam {
            n: 8,
            m: 112,
            q: 67,
            beta_squared: 112,
        },
        SisSizeParam {
            n: 16,
            m: 288,
            q: 257,
            beta_squared: 288,
        },
    ];

    fn transaction(seed: u8) -> Transaction {
        Transaction {
            network_id: u32::from(seed),
            from: Address::from_bytes([seed; 32]),
            to: Address::from_bytes([seed.wrapping_add(1); 32]),
            amount: Amount::new(u64::from(seed) << 40),
            fee: Amount::new(u64::from(seed)),
            nonce: u64::from(seed) + 1,
            pubkey: PublicKey::from_bytes([seed.wrapping_add(2); 32]),
            sig: Signature::from_bytes([seed.wrapping_add(3); 64]),
        }
    }

    fn block(size: SisSizeParam, transactions: usize) -> Block {
        let solution_len = usize::try_from(size.solution_len()).expect("small fixture");
        Block {
            header: Header {
                network_id: 1,
                height: 2,
                parent_hash: [3; 32],
                beacon_out: [4; 32],
                instance_seed: [5; 32],
                size_param: SizeParam::Sis(size),
                hash_target: [6; 32],
                tx_root: [7; 32],
                state_root: [8; 32],
                sol_commit: [9; 32],
                timestamp: 10,
                miner_addr: Address::from_bytes([11; 32]),
                nonce: 12,
            },
            solution: (0..solution_len)
                .map(|index| u8::try_from(index % 256).expect("below 256"))
                .collect(),
            transactions: (0..transactions)
                .map(|index| transaction(u8::try_from(index % 200).expect("below 200")))
                .collect(),
        }
    }

    #[test]
    fn layout_is_header_count_solution_count_transactions() {
        for size in NON_NORMATIVE_STAND_INS {
            let active = SizeParam::Sis(size);
            let value = block(size, 2);
            let bytes = value.encode(&active).expect("fixture encodes");
            let solution_len = value.solution.len();
            assert_eq!(bytes.len(), 306 + 4 + solution_len + 4 + 2 * 188);
            assert_eq!(&bytes[..306], value.header.encode().as_slice());
            assert_eq!(
                bytes[306..310],
                u32::try_from(solution_len).expect("small").to_le_bytes()
            );
            assert_eq!(&bytes[310..310 + solution_len], value.solution.as_slice());
            let after = 310 + solution_len;
            assert_eq!(bytes[after..after + 4], 2_u32.to_le_bytes());
            assert_eq!(
                &bytes[after + 4..after + 4 + 188],
                &value.transactions[0].encode()
            );
            assert_eq!(&bytes[after + 4 + 188..], &value.transactions[1].encode());
        }
    }

    #[test]
    fn encode_then_decode_returns_the_value_and_decode_then_encode_the_input() {
        for size in NON_NORMATIVE_STAND_INS {
            let active = SizeParam::Sis(size);
            for transactions in [0, 1, 3] {
                let value = block(size, transactions);
                let bytes = value.encode(&active).expect("fixture encodes");
                let decoded = Block::decode(&bytes, &active).expect("fixture decodes");
                assert_eq!(decoded, value);
                assert_eq!(decoded.encode(&active), Ok(bytes));
            }
        }
    }

    #[test]
    fn the_whole_is_tested_before_any_field_is_read() {
        let active = SizeParam::Sis(NON_NORMATIVE_STAND_INS[0]);
        // All zero bytes: `class_id` 0 would be the error if a field were read first.
        let input = vec![0_u8; BLOCK_MAX_BYTES + 1];
        assert_eq!(
            Block::decode(&input, &active),
            Err(DecodeError::WholeExceedsLimit {
                length: 1_048_577,
                limit: 1_048_576
            })
        );
        assert_eq!(
            Block::decode(&input[..BLOCK_MAX_BYTES], &active),
            Err(DecodeError::UnknownClassId { class_id: 0 })
        );
        assert_eq!(
            Block::decode(&input[..BLOCK_MAX_BYTES - 1], &active),
            Err(DecodeError::UnknownClassId { class_id: 0 })
        );
    }

    #[test]
    fn the_block_limit_accepts_at_the_boundary_and_rejects_one_above() {
        // A well-formed block is 314 + 2m + 188k bytes, which is always even, and with
        // the stand-in tuples no such length equals 1,048,576. The boundary is therefore
        // exercised here against the block's own length as the limit.
        for size in NON_NORMATIVE_STAND_INS {
            let active = SizeParam::Sis(size);
            let value = block(size, 4);
            let bytes = value.encode(&active).expect("fixture encodes");
            let len = bytes.len();

            assert_eq!(
                Block::decode_with_limit(&bytes, &active, len),
                Ok(value.clone())
            );
            assert_eq!(value.encode_with_limit(&active, len), Ok(bytes.clone()));

            assert_eq!(
                Block::decode_with_limit(&bytes, &active, len - 1),
                Err(DecodeError::WholeExceedsLimit {
                    length: u64::try_from(len).expect("small"),
                    limit: u64::try_from(len - 1).expect("small"),
                })
            );
            assert_eq!(
                value.encode_with_limit(&active, len - 1),
                Err(EncodeError::WholeExceedsLimit {
                    length: u64::try_from(len).expect("small"),
                    limit: u64::try_from(len - 1).expect("small"),
                })
            );

            // One byte below the block's length cannot be a block: it is one
            // transaction short by one byte.
            assert_eq!(
                Block::decode_with_limit(&bytes[..len - 1], &active, len),
                Err(DecodeError::TooShort {
                    needed: 4 * 188,
                    remaining: 4 * 188 - 1
                })
            );
        }
    }

    #[test]
    fn the_largest_block_under_the_provisional_limit_round_trips() {
        for size in NON_NORMATIVE_STAND_INS {
            let active = SizeParam::Sis(size);
            let overhead = 314 + usize::try_from(size.solution_len()).expect("small");
            let most = (BLOCK_MAX_BYTES - overhead) / 188;
            let value = block(size, most);
            let bytes = value.encode(&active).expect("largest block encodes");
            assert!(bytes.len() <= BLOCK_MAX_BYTES);
            assert!(bytes.len() + 188 > BLOCK_MAX_BYTES);
            assert_eq!(Block::decode(&bytes, &active), Ok(value));

            let one_more = block(size, most + 1);
            assert_eq!(
                one_more.encode(&active),
                Err(EncodeError::WholeExceedsLimit {
                    length: u64::try_from(overhead + (most + 1) * 188).expect("small"),
                    limit: 1_048_576
                })
            );
        }
    }

    #[test]
    fn the_solution_limit_holds_at_the_boundary_one_below_and_one_above() {
        for size in NON_NORMATIVE_STAND_INS {
            let active = SizeParam::Sis(size);
            let exact = size.solution_len();
            let value = block(size, 1);
            let bytes = value.encode(&active).expect("fixture encodes");
            assert!(Block::decode(&bytes, &active).is_ok());

            // One above the limit: the count exceeds SOL_MAX_BYTES.
            let mut above = bytes.clone();
            above[306..310]
                .copy_from_slice(&u32::try_from(exact + 1).expect("small").to_le_bytes());
            assert_eq!(
                Block::decode(&above, &active),
                Err(DecodeError::CountExceedsLimit {
                    count: exact + 1,
                    limit: exact
                })
            );

            // One below: within the limit, and not the exact length.
            let mut below = bytes.clone();
            below[306..310]
                .copy_from_slice(&u32::try_from(exact - 1).expect("small").to_le_bytes());
            assert_eq!(
                Block::decode(&below, &active),
                Err(DecodeError::SolutionLengthMismatch {
                    expected: exact,
                    actual: exact - 1
                })
            );

            let mut longer = value.clone();
            longer.solution.push(0);
            assert_eq!(
                longer.encode(&active),
                Err(EncodeError::SolutionExceedsLimit {
                    length: exact + 1,
                    limit: exact
                })
            );
            let mut shorter = value.clone();
            shorter.solution.pop();
            assert_eq!(
                shorter.encode(&active),
                Err(EncodeError::SolutionLengthMismatch {
                    expected: exact,
                    actual: exact - 1
                })
            );
        }
    }

    #[test]
    fn the_header_size_parameter_gives_the_exact_length_and_active_only_the_limit() {
        let [smaller, larger] = NON_NORMATIVE_STAND_INS;
        // Header carries the smaller tuple, the argument is the larger one: within the
        // limit and exact for the header. The decoder accepts; block rule B3 would not.
        let value = block(smaller, 1);
        let bytes = value
            .encode(&SizeParam::Sis(larger))
            .expect("within the larger limit");
        assert_eq!(Block::decode(&bytes, &SizeParam::Sis(larger)), Ok(value));

        // Header carries the larger tuple, the argument is the smaller one: exact for
        // the header and above the limit.
        let value = block(larger, 1);
        let bytes = value
            .encode(&SizeParam::Sis(larger))
            .expect("fixture encodes");
        assert_eq!(
            Block::decode(&bytes, &SizeParam::Sis(smaller)),
            Err(DecodeError::CountExceedsLimit {
                count: larger.solution_len(),
                limit: smaller.solution_len()
            })
        );
    }

    #[test]
    fn the_transaction_count_is_judged_against_the_bytes_that_remain() {
        let size = NON_NORMATIVE_STAND_INS[0];
        let active = SizeParam::Sis(size);
        let value = block(size, 2);
        let bytes = value.encode(&active).expect("fixture encodes");
        let count_at = 310 + usize::try_from(size.solution_len()).expect("small");

        // The count needs more bytes than remain: "shorter than required".
        let mut more = bytes.clone();
        more[count_at..count_at + 4].copy_from_slice(&3_u32.to_le_bytes());
        assert_eq!(
            Block::decode(&more, &active),
            Err(DecodeError::TooShort {
                needed: 3 * 188,
                remaining: 2 * 188
            })
        );

        // The count needs fewer: "bytes remain".
        let mut fewer = bytes.clone();
        fewer[count_at..count_at + 4].copy_from_slice(&1_u32.to_le_bytes());
        assert_eq!(
            Block::decode(&fewer, &active),
            Err(DecodeError::TrailingBytes { extra: 188 })
        );

        // The largest count: no overflow, no allocation, "shorter than required".
        let mut most = bytes.clone();
        most[count_at..count_at + 4].copy_from_slice(&u32::MAX.to_le_bytes());
        assert_eq!(
            Block::decode(&most, &active),
            Err(DecodeError::TooShort {
                needed: 188 * u64::from(u32::MAX),
                remaining: 2 * 188
            })
        );

        // One stray byte after the last transaction.
        let mut stray = bytes.clone();
        stray.push(0);
        assert_eq!(
            Block::decode(&stray, &active),
            Err(DecodeError::TrailingBytes { extra: 1 })
        );
    }

    #[test]
    fn every_truncation_of_a_block_is_rejected_without_panic() {
        let size = NON_NORMATIVE_STAND_INS[0];
        let active = SizeParam::Sis(size);
        let bytes = block(size, 2).encode(&active).expect("fixture encodes");
        for len in 0..bytes.len() {
            assert!(matches!(
                Block::decode(&bytes[..len], &active),
                Err(DecodeError::TooShort { .. })
            ));
        }
    }

    #[test]
    fn an_unregistered_class_in_the_header_is_reported_before_the_counts() {
        let size = NON_NORMATIVE_STAND_INS[0];
        let active = SizeParam::Sis(size);
        let mut bytes = block(size, 1).encode(&active).expect("fixture encodes");
        bytes[108..110].copy_from_slice(&0_u16.to_le_bytes());
        bytes[306..310].copy_from_slice(&u32::MAX.to_le_bytes());
        assert_eq!(
            Block::decode(&bytes, &active),
            Err(DecodeError::UnknownClassId { class_id: 0 })
        );
    }
}
