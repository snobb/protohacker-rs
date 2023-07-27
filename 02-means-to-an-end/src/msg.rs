#[derive(Debug, PartialEq)]
pub enum Request {
    Insert { time: i32, price: i32 },
    Query { mintime: i32, maxtime: i32 },
}

impl Request {
    pub const SIZE: usize = 9;
}

#[derive(Debug)]
pub enum RequestError {
    UnknownType(u8),
}

impl TryFrom<[u8; Request::SIZE]> for Request {
    type Error = RequestError;

    fn try_from(value: [u8; Request::SIZE]) -> Result<Self, Self::Error> {
        let kind = value[0];
        let value1 = i32::from_be_bytes([value[1], value[2], value[3], value[4]]);
        let value2 = i32::from_be_bytes([value[5], value[6], value[7], value[8]]);

        match kind {
            b'I' => Ok(Request::Insert {
                time: value1,
                price: value2,
            }),

            b'Q' => Ok(Request::Query {
                mintime: value1,
                maxtime: value2,
            }),

            _ => Err(RequestError::UnknownType(kind)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_from_insert() {
        let buf: [u8; Request::SIZE] = [
            0x49, // I for insert
            0x00, 0x00, 0x30, 0x39, // 12345
            0x00, 0x00, 0x00, 0x65, // 101
        ];

        let want = Request::try_from(buf).unwrap();
        assert_eq!(
            want,
            Request::Insert {
                time: 12345,
                price: 101
            }
        )
    }

    #[test]
    fn test_try_from_query() {
        let buf: [u8; Request::SIZE] = [
            0x51, // Query
            0x00, 0x00, 0x30, 0x00, // 12288
            0x00, 0x00, 0x40, 0x00, // 16384
        ];

        let want = Request::try_from(buf).unwrap();
        assert_eq!(
            want,
            Request::Query {
                mintime: 12288,
                maxtime: 16384
            }
        )
    }
}
