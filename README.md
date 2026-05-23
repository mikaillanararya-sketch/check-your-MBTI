# MBTI Smart Contract App

A decentralized MBTI personality test application built using Rust and Soroban SDK on the Stellar blockchain network.

## Contract Information
| Item        | Value                                                      |
| ----------- | ---------------------------------------------------------- |
| Contract ID | `CCD6YLZVJMWSSPIOSQ7BOR2Q5UJNKG5OZZILMX4L52YJUWOXETJFIGU3` |


## Features

* Determine MBTI personality type automatically
* Store MBTI results on-chain
* Retrieve all MBTI history
* Delete MBTI results by ID
* Built with Rust and Soroban SDK

---

# How It Works

The application receives scores from four MBTI dimensions:

* Extrovert (E) vs Introvert (I)
* Sensing (S) vs Intuition (N)
* Thinking (T) vs Feeling (F)
* Judging (J) vs Perceiving (P)

The smart contract compares each pair of scores and generates a personality type such as:

* INTJ
* ENFP
* ESFJ
* ENTJ

All results are stored directly on the blockchain.

---

# Project Structure

```bash
.
├── src
│   ├── lib.rs
│   └── test.rs
├── Cargo.toml
└── README.md
```

---

# Smart Contract Functions

## `determine_mbti`

Determine user MBTI type and save it to blockchain storage.

### Parameters

| Parameter | Type   |
| --------- | ------ |
| name      | String |
| e         | u32    |
| i         | u32    |
| s         | u32    |
| n         | u32    |
| t         | u32    |
| f         | u32    |
| j         | u32    |
| p         | u32    |

### Example

```rust
determine_mbti(
    env,
    String::from_str(&env, "Mika"),
    8,
    3,
    6,
    9,
    7,
    2,
    5,
    4
);
```

### Result

```txt
ENTJ
```

---

## `get_results`

Retrieve all saved MBTI results from blockchain storage.

---

## `delete_result`

Delete MBTI result by ID.

### Parameters

| Parameter | Type |
| --------- | ---- |
| id        | u64  |

---

# Technologies Used

* Rust
* Soroban SDK
* Stellar Blockchain

---

# Installation

## Clone Repository

```bash
git clone <repository-url>
cd mbti-smart-contract
```

## Build Project

```bash
cargo build
```

## Run Tests

```bash
cargo test
```

---

# Example MBTI Result Structure

```rust
pub struct MBTIResult {
    id: u64,
    name: String,
    mbti_type: String,
    description: String,
}
```

---

# License

mikaillanararya
