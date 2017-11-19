# circle-code

Reinterpretation of Facebook Messenger profile code. 

![Facebook example](https://i.stack.imgur.com/uJo9L.png)

## Usage

### Run

`cargo run url_string avatar_url logo_url color`

ex : 

`cargo run http://github.com/qdequele https://avatars0.githubusercontent.com/u/6064892?s=460&v=4 https://avatars2.githubusercontent.com/u/33703450?s=200&v=4 #0084ff`


### Build

```
cargo build --release
cp target/release/ccode .
```

ex : 

`./ccode http://github.com/qdequele https://avatars0.githubusercontent.com/u/6064892?s=460&v=4 https://avatars2.githubusercontent.com/u/33703450?s=200&v=4 #0084ff`

## Specification

### Image composition

Image size : 400px

Number of circle : 4

Number of point by circle : (farest to nearest from the center)

	- 1 : 36
	- 2 : 36
	- 3 : 24
	- 4 : 36

Circle margin : 16px

Circle ray : (farest to nearest from the center)

	- 1 : 180px
	- 2 : 164px
	- 3 : 148px
	- 4 : 132px

Stroke width : 6px

Anchors : 4

	- North
	- East
	- South
	- West

Anchor external border width : 4 px
Anchor internal border width : 1 px

Avatar image ray: 116 px

### Data 

Start reading point : East on circle 1

String encoding : ISO_8859_1 (Latin-1)

String hash : md5

String max length : 255 chars

## Dependances

[lifthrasiir/rust-encoding](https://github.com/lifthrasiir/rust-encoding)
[stainless-steel/md5](https://github.com/stainless-steel/md5)
[iron/iron](https://github.com/iron/iron)
[iron/body-parser](https://github.com/iron/body-parser)
