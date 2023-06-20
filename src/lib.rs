#[derive(Debug)]
pub struct HandValues {
    two: u8,
    three: u8,
    four: u8,
    five: u8,
    six: u8, 
    seven: u8,
    eight: u8,
    nine: u8,
    ten: u8,
    jack: u8,
    queen: u8,
    king: u8, 
    ace: u8,
}

impl HandValues {
    fn new(values: [u8; 13]) -> Self {
        Self {
            two: values[0],
            three: values[1],
            four: values[2],
            five: values[3],
            six: values[4],
            seven: values[5],
            eight: values[6],
            nine: values[7],
            ten: values[8],
            jack: values[9],
            queen: values[10],
            king: values[11],
            ace: values[12],
        }
    }
}
#[derive(Debug)]
pub struct HandScore {
    point_array: HandValues,
    hand_type: HandType,

}

impl HandScore {
    pub fn new() -> Self {

        let values_to_assign: [u8; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

        let hand_values = HandValues::new(values_to_assign);

        let ht = HandType::new();
    
        Self {
            point_array: hand_values,
            hand_type: ht
        }
    }

    pub fn display_point_values(&self) {
        println!("{:?}", self.point_array);
        println!("{:?}", self.hand_type);
    }
}

#[derive(Debug)]
pub struct HandType {
    HighCard: u8,
    Pair: u8,
    TwoPair: u8,
    ThreeOfAKind: u8,
    Straight: u8,
    Flush: u8,
    FullHouse: u8, 
    Quads: u8,
    StraightFlush: u8,
    RoyalFlush: u8,
}
/*
IMPORTANT 
two-tiered scoring system

we can evaluate the point totals for situations where straight vs straight occurs,
the max sum of all evaluated hands will be best
at a specific level this can be pruned
if we know for example best hand is in a higher rank indicated by 
multiplying the base score, 

EXAMPLE

Pair of 4s
4pts each
8*2 = 16 point hand value

TwoPair
4s and 3s
(2 is the pair rank multiple) 
4*2 + 3*2 = 14 
        -> 14 * 3 (two pair rank multiple of 3) = 42

10s and Jacks
10 + 11 -> ans * 3 = 63
42 * 3 = 126

Jacks and Queens
(11 + 12) * 3 = 69

Aces and Kings (top range of two pairs)
14 + 13 
* 3
= 81

at minimum sets must be > 81
Set of 2s
n*rank_multiple squared
(2*5)^2
10 * 10 = 100

but this means for sets, (14*5)^2 now is the min value
of straight hands

//Straights

4-5-6-7-8

4+5+6+7+8 = 30*5 ^2

2+3+4+5+6 = 20*5 ^2
need to recognize A2345 as worse point value


 */
impl HandType {
    pub fn new() -> Self {
        Self {
            HighCard: 1,
            Pair: 2,
            TwoPair: 3,
            ThreeOfAKind: 4,
            Straight: 5,
            Flush: 6,
            FullHouse: 7,
            Quads: 8,
            StraightFlush: 9,
            RoyalFlush: 10,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum HandRank {
    HighCard(u8),
    Pair(u8),
    TwoPair(u8),
    ThreeOfAKind(u8),
    Straight(u8),
    Flush(u8),
    FullHouse(u8),
    FourOfAKind(u8),
    StraightFlush(u8),
    RoyalFlush(u8),
}
