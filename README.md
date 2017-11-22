# circle-code

Reinterpretation of Facebook Messenger profile code. 

![example](https://raw.githubusercontent.com/OpenCircleCode/occ-generator/master/image.png)

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

Number of point by circle :

	- P36 = 36
	- P40 = 40
	- P45 = 45
	- P60 = 60
	- P72 = 72
	- P90 = 90
	- P120 = 120

Circle margin : 16px

Circle ray : (farest to nearest from the center)

	- 1 : 180px
	- 2 : 164px
	- 3 : 148px
	- 4 : 132px

Stroke width : 7px

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

String max length : 255 chars

## Example

```
./ccode "http://github.com/qdequele" "https://avatars0.githubusercontent.com/u/6064892?s=460&v=4" "https://avatars2.githubusercontent.com/u/33703450?s=200&v=4" "#0084ff"
```

binary : 0110100001110100011101000111000000111010001011110010111101100111011010010111010001101000011101010110001000101110011000110110111101101101001011110111000101100100011001010111000101110101011001010110110001100101

## Dependances

[lifthrasiir/rust-encoding](https://github.com/lifthrasiir/rust-encoding)

[stainless-steel/md5](https://github.com/stainless-steel/md5)

[iron/iron](https://github.com/iron/iron)

[iron/body-parser](https://github.com/iron/body-parser)
