#![feature(specialization)]

trait ItemIndex<'a, T> {
    fn item_index(&self, item: &'a T) -> Option<usize>;
}
pub struct Rps<'a, T> {
    sp: &'a [T],
    n: usize,
    p: usize
}
impl<'a, T> ItemIndex<'a, T> for Rps<'a, T> {
    default fn item_index(&self, item: &'a T) -> Option<usize> {
        self.sp.iter().position(|x| x as *const _==item as *const _)
    }
}
impl<'a, T> ItemIndex<'a, T> for Rps<'a, T>
where T: PartialEq {
    fn item_index(&self, item: &'a T) -> Option<usize> {
        self.sp.iter().position(|x| x==item)
    }
}
impl<'a, T> Rps<'a, T> {
    pub fn new(sp: &'a [T]) -> Result<Self, &'static str> {
        let n = sp.len() as usize;
        if n%2!=0 {
            Ok(Self { sp, n, p: (n-1)/2 }) 
        } else {
            Err("Length of slice must be odd")
        }
    }

    pub fn winner(&self, p1: &'a T, p2: &'a T) -> Result<Option<&'a T>, &'static str> {
        match (self.item_index(p1), self.item_index(p2)) {
            (Some(x), Some(y)) => {
                for i in 1..=self.p {
                    if (x+i)%self.n==y {
                        return Ok(Some(p2))
                    } else if (y+i)%self.n==x {
                        return Ok(Some(p1))
                    }
                }
                return Ok(None)
            }
            _ => return Err("Item is not in slice")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Rps;

    #[derive(PartialEq, Debug)]
    enum WithPEQ {
        Rock, Paper, Scissors
    }

    enum WithoutPEQ {
        Rock, Paper, Scissors
    }

    #[derive(Debug, PartialEq)]
    enum RPSLSP {
        Rock, Paper, Scissors, Lizard, Spock
    }

    #[test]
    fn commutative() {
        use WithPEQ::{Rock, Paper, Scissors};
        let rps = Rps::new(&[Paper, Scissors, Rock]).unwrap();

        assert_eq!(rps.winner(&Rock, &Paper), rps.winner(&Paper, &Rock));
    }
    #[test]
    fn type_with_partial_eq() {
        use WithPEQ::{Rock, Paper, Scissors};
        let rps = Rps::new(&[Paper, Scissors, Rock]).unwrap();

        rps.winner(&WithPEQ::Rock, &WithPEQ::Scissors).unwrap().unwrap();
    }
    #[test]
    fn type_without_partial_eq_incorrect() {
        use WithoutPEQ::*;
        let rps = Rps::new(&[Paper, Scissors, Rock]).unwrap();

        assert!(rps.winner(&WithoutPEQ::Rock, &WithoutPEQ::Scissors).is_err());
    }
    #[test]
    fn type_without_partial_eq_correct() {
        use WithoutPEQ::*;
        let items = [Paper, Scissors, Rock];
        let rps = Rps::new(&items).unwrap();

        rps.winner(&items[0], &items[1]).unwrap().unwrap();
    }
    #[test]
    fn draw() {
        use WithPEQ::*;
        let rps = Rps::new(&[Paper, Scissors, Rock]).unwrap();

        assert_eq!(rps.winner(&Rock, &Rock).unwrap(), None);
    }
    #[test]
    fn rps_logic() {
        use WithPEQ::{Rock, Paper, Scissors};
        let rps = Rps::new(&[Paper, Scissors, Rock]).unwrap();

        assert_eq!(rps.winner(&Rock, &Paper).unwrap().unwrap(), &Paper);
        assert_eq!(rps.winner(&Rock, &Scissors).unwrap().unwrap(), &Rock);
        assert_eq!(rps.winner(&Paper, &Scissors).unwrap().unwrap(), &Scissors);
    }
    #[test]
    fn longer_logic() {
        use RPSLSP::*;
        let rps = Rps::new(&[Rock, Spock, Paper, Lizard, Scissors]).unwrap();

        assert_eq!(rps.winner(&Rock, &Scissors).unwrap().unwrap(), &Rock);
        assert_eq!(rps.winner(&Rock, &Lizard).unwrap().unwrap(), &Rock);
        assert_eq!(rps.winner(&Scissors, &Paper).unwrap().unwrap(), &Scissors);
        assert_eq!(rps.winner(&Scissors, &Lizard).unwrap().unwrap(), &Scissors);
        assert_eq!(rps.winner(&Paper, &Rock).unwrap().unwrap(), &Paper);
        assert_eq!(rps.winner(&Paper, &Spock).unwrap().unwrap(), &Paper);
        assert_eq!(rps.winner(&Lizard, &Paper).unwrap().unwrap(), &Lizard);
        assert_eq!(rps.winner(&Lizard, &Spock).unwrap().unwrap(), &Lizard);
        assert_eq!(rps.winner(&Spock, &Rock).unwrap().unwrap(), &Spock);
        assert_eq!(rps.winner(&Spock, &Scissors).unwrap().unwrap(), &Spock);
    }
}
