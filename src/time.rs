use chrono::{DateTime, Utc};
use std::io;

pub(crate) fn get_time(value: &str) -> io::Result<DateTime<Utc>> {
    let fields = value.split_whitespace().collect::<Vec<&str>>();
    if fields.len() != 6 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "timezone is invalid: should have been six fields: {}",
                value
            ),
        ));
    }

    let mut buffer = fields[..4].join(" ");
    buffer.push(' ');

    if fields[4].split(':').next().unwrap().len() == 1 {
        buffer.push('0');
    }

    buffer.push_str(&fields[4]);
    buffer.push(' ');

    if fields[5] == "UTC" {
        buffer.push_str("+0000");
    } else {
        buffer.push_str(&fields[5])
    };

    DateTime::parse_from_rfc2822(&buffer)
        .map(|tz| tz.with_timezone(&Utc))
        .map_err(|why| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unable to parse date ({}) in release file: {}", buffer, why),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debian_rfc2822_quirks() {
        let time = "Wed, 28 Nov 2018 3:16:40 UTC";
        assert!(get_time(time).is_ok());
    }
}
