<h1 align="center">Rust Qualifier Exam</h1>

An open source learning resource, available to all.

This exam is maintained by the Polkadot Blockchain Academy, for the benefit of the entire Rust community.
The Academy accepts individuals modestly skilled in Rust, and maintains this exam to help everyone assess their proficiency being of a level we would consider for the program.

We encourage everyone to take this exam for fun, to help assess your own Rust understanding, or as part of your [application to the Polkadot Blockchain Academy](#applying-to-the-polkadot-blockchain-academy).

## Honor Code

When used as a qualifier for the Academy, this exam is intended to be taken individually without help from other programmers, AIs, or the breadth of the internet.
The first section of the exam is the honor code.
Please read it _carefully and thoughtfully_ before you review or start any work.

Those not applying to the Academy are free to relax the honor code as you deem fit, however, the **[license and use policy](#license-and-use-policies) forbids publishing solutions** for everyone.
Naturally as an exam, public solutions undermine usefulness of this public good for the whole community.
We humbly ask you respect the intended academic integrity of our community when considering how you interact with the exam in your communities.

## Time Commitment

The exam is intended to be quite a challenge for those more novice in Rust.
It is expected to take a minimum of 10-20 hours for the average applicant to complete.
Your time may be shorter or significantly longer than this.

## Organization

This exam is structured as a single Rust crate with many modules.
Each module contains some well-commented starter code, and some `todo!()` macros that you should replace with solution code.

## Tests

Each module contains a small starter test suite to help you know whether your solutions are on the right track or not.
You can run these tests with standard cargo commands.
The provided tests are mostly failing when you first begin, but they should all pass by the time you are finished.

```sh
# Run the unit tests
cargo test

# Run the doc doc tests
cargo test --doc
```

The provided tests are **not** a complete test suite.
You are encouraged to write additional tests to ensure that their code works as intended.

The Academy maintains a larger more thorough test suite privately.
If you are applying, your submission will be assessed with this more thorough test suite.
To be sure that the test suite will always work, **do not change any function or type signatures** unless explicitly permitted to do so.

The entire exam should be completed using Rust stable toolchain with the specified version in [Cargo.toml](./Cargo.toml) as the Academy test suite will explicitly use this version.

## Completing the Exam

Work through the exam by replacing each occurrence of `todo!()` with your solution to the problem.
When all the provided tests pass, plus all the additional tests you wrote are passing, your work is finished.
If you are pushing to any remote servers while solving the exam (as is required for PBA applicants) be sure that the remote servers are not publicly viewable.
You may not publish solutions to the exam, please read the [License and use policies](#license-and-use-policies) for specifics.

## Applying to the Polkadot Blockchain Academy

The Polkadot Blockchain Academy is a classroom-based educational program covering the conceptual underpinnings and the hands-on application of blockchain technology.
The curriculum covers disciplines such as economics, governance, game theory, cryptography, peer-based and distributed network systems, systems and API design, and much more.
In addition to theoretical modules, students will apply their knowledge and build blockchains and parachains using [Substrate](https://substrate.io).

To apply to the Polkadot Blockchain Academy, please visit https://dot.li/pba-github for information, location, and dates of the current or next cohort, and a form that will start the process for consideration in the next cohort.
This qualifier exam is just one part of the process and is not, by itself, an application.

### License and Use Policies

Mozilla Public License Version 2.0 - See the [LICENSE](./LICENSE) for details.

In addition to the license, you may not publish materials that reveal, directly or indirectly, in any way, solutions to this exam publicly.
Lastly, you should not share any solutions or privately to prevent peer-collusion and respect the academic integrity of this public good resource of the Academy's community.

# 🚀 Good luck! 🚀
