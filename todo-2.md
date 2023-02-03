# What you should do now (2)

## Clone `squink-splash-beginner`

```bash
git clone https://github.com/paritytech/squink-splash-beginner.git
cd squink-splash-beginner/
```

## `lib.rs`: change line 28 

Set your favorite numbers between 0-15.

## Build your Player Contract

Execute:

```bash
cargo contract build --release
```

Your contract will be written here:

```
target/ink/basic_player.contract
```

## Upload & Instantiate Your Player

[Open Contracts UI here](https://github.com/paritytech/contracts-ui).

<img src=".images/rococo.png" />

<img src=".images/s1.png" />

<img src=".images/s2.png" />

Upload `target/ink/basic_player.contract`:

<img src=".images/s4.png" />

<img src=".images/s5.png" />

<img src=".images/s6.png" />

## Upload & Instantiate `game-metadata.json`

<img src=".images/s7.png" />

<img src=".images/s8.png" />

<img src=".images/s10.png" />

## Register your player

<img src=".images/s14.png" />

<img src=".images/s0.png" />
