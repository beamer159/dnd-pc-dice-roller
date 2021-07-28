use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};
use std::fmt::{Debug, Display, Formatter};

trait FromU8 {
    fn from_u8(v: u8) -> Self;
}

trait ToU8 {
    fn to_u8(&self) -> u8;
}

#[derive(Eq, Copy, Clone)]
struct Ru8(u8);

impl PartialEq for Ru8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Ru8 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ru8 {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl Debug for Ru8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromU8 for Ru8 {
    fn from_u8(v: u8) -> Self {
        Self(v)
    }
}

impl ToU8 for Ru8 {
    fn to_u8(&self) -> u8 {
        self.0
    }
}

impl FromU8 for u8 {
    fn from_u8(v: u8) -> Self {
        v
    }
}

impl ToU8 for u8 {
    fn to_u8(&self) -> u8 {
        *self
    }
}

pub struct Roll {
    d20: i8,
    dice: u8,
}

impl Roll {
    pub fn new(d20: i8, dice: u8) -> Self {
        Self { d20, dice }
    }
}

impl Display for Roll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.d20, self.dice)
    }
}

pub struct Character {
    dice: u8,
    soh: i8,
    d20_map: BTreeMap<u8, u8>,
}

impl Character {
    pub fn new(dice: u8, soh: i8) -> Self {
        Self {
            dice,
            soh,
            d20_map: Self::derive_d20_map(dice, soh),
        }
    }

    pub fn roll(&self) -> Roll {
        let d20 = rand::thread_rng().gen_range(1..=20);
        let dice = *self.d20_map.get(&d20).unwrap();
        Roll::new(d20 as i8 + self.soh, dice)
    }

    pub fn d20_map_string(&self) -> String {
        let mut string = String::new();
        for (&d20_roll, dice_roll) in &self.d20_map {
            string
                .push_str(format!("{:2} -> {:2}\n", d20_roll as i8 + self.soh, dice_roll).as_str())
        }
        string
    }

    pub fn set_dice(&mut self, dice: u8) {
        self.dice = dice;
        self.d20_map = Self::derive_d20_map(self.dice, self.soh);
    }

    pub fn set_soh(&mut self, soh: i8) {
        self.soh = soh;
        self.d20_map = Self::derive_d20_map(self.dice, self.soh);
    }

    fn derive_d20_map(dice: u8, soh: i8) -> BTreeMap<u8, u8> {
        if soh < 0 {
            Self::derive_d20_map_from_type::<Ru8>(dice, soh.unsigned_abs())
        } else {
            Self::derive_d20_map_from_type::<u8>(dice, soh.unsigned_abs())
        }
    }

    fn derive_d20_map_from_type<T: Copy + Ord + FromU8 + ToU8>(
        dice: u8,
        soh: u8,
    ) -> BTreeMap<u8, u8> {
        let mut dice_map = Self::derive_dice_map::<T>(dice);
        Self::do_increments(&mut dice_map, soh);
        Self::dice_map_to_d20_map(&dice_map)
    }

    fn derive_dice_map<T: Ord + FromU8>(dice: u8) -> BTreeMap<T, BinaryHeap<T>> {
        let threshold = 20. / f64::from(dice);
        (1_u8..=20)
            .map(|x| (x, (f64::from(x) / threshold).ceil() as u8))
            .fold(BTreeMap::new(), |mut m, (k, v)| {
                m.entry(T::from_u8(v))
                    .or_insert_with(BinaryHeap::new)
                    .push(T::from_u8(k));
                m
            })
    }

    fn do_increments<T: Copy + Ord>(dice_map: &mut BTreeMap<T, BinaryHeap<T>>, soh: u8) {
        for _ in 0..soh {
            if Self::increment(dice_map).is_none() {
                break;
            }
        }
    }

    fn increment<T: Copy + Ord>(dice_map: &mut BTreeMap<T, BinaryHeap<T>>) -> Option<()> {
        let pop_this = {
            let last_key = dice_map.keys().last().unwrap();
            let (valid_keys, weights): (Vec<&T>, Vec<_>) = dice_map
                .iter()
                .filter_map(|(k, v)| {
                    if k != last_key && v.len() > 1 {
                        Some((k, v.len()))
                    } else {
                        None
                    }
                })
                .unzip();
            let dist = WeightedIndex::new(weights).ok()?;
            let mut rng = rand::thread_rng();
            *valid_keys[dist.sample(&mut rng)]
        };
        let incrementing_value = dice_map.get_mut(&pop_this).unwrap().pop().unwrap();
        let (_, push_to_this) = dice_map.range_mut(pop_this..).nth(1).unwrap();
        push_to_this.push(incrementing_value);
        Some(())
    }

    fn dice_map_to_d20_map<T: Copy + Ord + ToU8>(
        dice_map: &BTreeMap<T, BinaryHeap<T>>,
    ) -> BTreeMap<u8, u8> {
        dice_map
            .iter()
            .flat_map(|(&k, v)| v.iter().map(move |&a| (a.to_u8(), k.to_u8())))
            .collect()
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Character with {:+} Sleight of Hand rolling a D{}",
            self.soh, self.dice
        )
    }
}
