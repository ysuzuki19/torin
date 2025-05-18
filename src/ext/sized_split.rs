use crate::prelude::*;

pub trait SizedSplit {
    fn sized_split<const N: usize>(&self, sep: &'static str) -> Result<[&str; N]>;
}

impl SizedSplit for str {
    fn sized_split<const N: usize>(&self, sep: &'static str) -> Result<[&str; N]> {
        let strs = self.split(sep).collect::<Vec<&str>>();
        match strs.try_into() {
            Ok(arr) => Ok(arr),
            Err(_) => trace!("Invalid trigger type "),
        }
    }
}

pub trait LeastSizedSplit {
    fn least_sized_split<const N: usize>(
        &self,
        sep: &'static str,
    ) -> Result<([&str; N], Vec<&str>)>;
}

impl LeastSizedSplit for str {
    fn least_sized_split<const N: usize>(
        &self,
        sep: &'static str,
    ) -> Result<([&str; N], Vec<&str>)> {
        let strs = self.split(sep).collect::<Vec<&str>>();
        if strs.len() < N {
            return trace!(
                "unsufficient number of parts: expected at least {}, found {}",
                N,
                strs.len(),
            );
        }
        let (sized, rest) = strs.split_at(N);
        let sized = match sized.try_into() {
            Ok(arr) => Ok(arr),
            Err(_) => trace!("Invalid number of parts, expected {}", N), // unreachable
        }?;
        Ok((sized, rest.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payloads_1() {
        let strs = "t".sized_split::<1>(" ");
        assert!(strs.is_ok());
        assert_eq!(strs.unwrap(), ["t"]);

        let strs = "t t".sized_split::<1>(" ");
        assert!(strs.is_err());

        let strs = "t t t".sized_split::<1>(" ");
        assert!(strs.is_err());
    }

    #[test]
    fn payloads_2() {
        let strs = "t".sized_split::<2>(" ");
        assert!(strs.is_err());

        let strs = "t t".sized_split::<2>(" ");
        assert!(strs.is_ok());
        assert_eq!(strs.unwrap(), ["t", "t"]);

        let strs = "t t t".sized_split::<2>(" ");
        assert!(strs.is_err());
    }
}
