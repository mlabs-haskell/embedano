# Embedano: Product Development Research interview Report

Ben Hart, MLabs

Dec 27, 2022

- [Embedano: Product Development Research interview Report](#embedano-product-development-research-interview-report)
  - [Introduction](#introduction)
  - [Interview #1: Nick Ayotte](#interview-1-nick-ayotte)
  - [Interview #2: Colin Hobbins](#interview-2-colin-hobbins)
  - [Interview #3: Shawn Roller \& Kostas Bastas](#interview-3-shawn-roller--kostas-bastas)
  - [Conclusions](#conclusions)

## Introduction

In the course of ensuring we’re providing direct value to the Cardano developer community through our work on the Embedano project, we elected to include some interviews with some hand-picked community members. We selected a number of individuals working across the NFT, Defi, and wallet verticals, namely: Nick Ayotte (Equine.gg), Colin Hobbins (Lode Wallet), and Shawn Roller/Kostas Bastas (Gero Wallet).

Each interview consisted of the following topics and questions:

- A high-level overview of the project (a firmware and interface for doing automated transaction or data signing with an HSM, targeting the Internet of Things as a key use-case).
- What difficulties have you encountered with existing hardware wallets, if any?
- What dependencies are you using to interact with Trezor and Ledger wallets? Do they use web-USB or some other technology?
- Have you seen use-cases for an automated signing device with a light wallet? Is light wallet compatibility a key concern for our use-case
How relevant is CIP-21 - is it still up to date? Are there problems with the design?

By holding these interviews, our hope is that we can resolve usability issues upfront and identify the ideal set of final deliverables for the project, specifically whether or not to enable light wallet support, if we have sufficient funding to do so.

## Interview #1: Nick Ayotte

Monday, Dec 5

Key takeaways:

- Internet-of-things and secure signing devices are a key use-case that really isn’t well addressed currently
- There are a lot of tokenized solar energy projects in particular that could make use of such a device
- At the enterprise level, Amazon Signing services currently don’t support the ed25519 signing scheme - which is critical for Cardano support, so one potential application is automated signing for enterprises who may be running automated actors on-chain (for example, a centralized bridge operator or arbitrage bot, both of which have significant security risks and would benefit from being able to use a hardware wallet for signing such that an attacker could not extract a private key from the device.
- CIP-21 will be critical, and it indicates some technical hurdles when working with large transactions in particular, in general, it is best practice to enable signing some auxiliary data.

## Interview #2: Colin Hobbins

Tuesday,  Dec 6

Key takeaways:

- Obsidian (Lode Wallet Parent company), is actually a Ledger-certified developer and is recommended for custom jobs by Ledger. They are particularly interested in anything Ledger related in Rust. They have [an existing project](https://blog.obsidian.systems/alamgu-the-rust-ledger-platform/), which is designed as a client system for working directly with Ledger, their work is quite applicable as our goal is to create something open source that works with the same hardware as Ledger.
- Obsidian may be able to verify the work done by this project, they have existing UI work around ledger approval, etc.
- “Through experimentation we have determined that for low-level, embedded applications, Rust is an ideal programming language. Compared to C, Rust offers improved safety, thanks to its rich type system and memory-safety guarantees; and compared to Haskell, Rust has very little runtime overhead.”
- There are [more Obsidian libraries relating to Ledger]( https://github.com/obsidiansystems?q=ledger&type=all&language=&sort=)
- Lode wallet is not pursuing hardware wallet support at this time since it is not a differentiating factor
- Light Wallet compatibility might not be necessary for our purposes around IoT

## Interview #3: Shawn Roller & Kostas Bastas

Wednesday, Dec 21

Key takeaways:

- Hardware wallets are really painful to support,  light wallets spend a lot of time supporting Ledger in particular, it’s much more painful than Trezor.
- It may fall out of scope for the current project, but a single library that abstracts over supported hardware devices would really make life easier for Wallet developers
- There is a problem in a recent release of Ledger - perhaps Vasil support issues?
- There is a Telegram chat that our team is now invited to for wallet developers who work on Hardware wallet compatibility (it currently includes Gero and Eternl, possibly others)
- Many other hardware wallets want support in Gero, even if they don’t properly support Cardano. (Keystone was an example used).

## Conclusions

Light wallet compatibility libraries are probably out-of-scope for this initial project, as it isn’t clear why someone might need a light wallet for this application at this time. In an IoT setting, the main device would delegate to the hardware wallet for signing and then submit the transaction via any available submit-API.

There’s a serious gap for enterprise users who might need automated signing of transactions but need to be able to secure their systems against attackers and reduce the loss associated with an attack.

These will remain our primary goals for the project. It is likely that we will continue to collaborate with Obsidian systems as we work through the project as they have a substantial amount of experience in this area and seem quite enthusiastic about the project.
