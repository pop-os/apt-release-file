use std::str::FromStr;

/// The size of an icon package. IE: `48x48@2`
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct ImageSize {
    pub pixels: u16,
    pub hidpi: u16,
}

impl FromStr for ImageSize {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .find('x')
            .ok_or("not a recognizable image string")
            .and_then(|pos| {
                let x = &input[..pos];
                let y = &input[pos + 1..];
                match y.find('@') {
                    Some(pos) => {
                        let z = &y[pos + 1..];
                        let y = &y[..pos];

                        match (x.parse::<u16>(), y.parse::<u16>(), z.parse::<u16>()) {
                            (Ok(pixels), Ok(y), Ok(hidpi)) => if pixels == y {
                                Ok(ImageSize { pixels, hidpi })
                            } else {
                                Err("width and height do not match")
                            },
                            _ => Err("width, height, and/or scale did not parse as integers"),
                        }
                    }
                    None => match (x.parse::<u16>(), y.parse::<u16>()) {
                        (Ok(pixels), Ok(y)) => if pixels == y {
                            Ok(ImageSize { pixels, hidpi: 0 })
                        } else {
                            Err("width and height do not match")
                        },
                        _ => Err("width and/or height failed to parse as integers"),
                    },
                }
            })
    }
}
