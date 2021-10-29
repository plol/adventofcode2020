pub struct Advent;

use std::convert::TryInto;

#[derive(Debug)]
struct HashLife {
    octree_by_hash: std::collections::HashMap<HashLifeOctree, usize>,
    octrees: Vec<HashLifeOctree>,
    result: Vec<Option<usize>>,
    root: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct HashLifeOctree {
    layer: usize,
    sub_octrees: [usize; 8],
}

const DNW: usize = 0;
const DNE: usize = 1;
const DSW: usize = 2;
const DSE: usize = 3;
const UNW: usize = 4;
const UNE: usize = 5;
const USW: usize = 6;
const USE: usize = 7;

fn cartesian_product(
    dx: usize,
    dy: usize,
    dz: usize,
) -> impl Iterator<Item = (usize, usize, usize)> {
    (0..dz).flat_map(move |z| (0..dy).flat_map(move |y| (0..dx).map(move |x| (x, y, z))))
}

impl HashLife {
    fn new(min_size: u32) -> HashLife {
        let top_layer: usize = (min_size as f64).log2().ceil() as usize;
        println!("min_size {}, top_layer {}", min_size, top_layer);
        let null_octree = HashLifeOctree {
            layer: 0,
            sub_octrees: [0, 0, 0, 0, 0, 0, 0, 0],
        };

        let empty_small_layer = HashLifeOctree {
            layer: 1,
            sub_octrees: [0, 0, 0, 0, 0, 0, 0, 0],
        };
        let (octrees, result) = (2usize..=top_layer).fold(
            (vec![null_octree, empty_small_layer], vec![Some(0), None]),
            |mut result, layer| {
                let prev = layer - 1;
                result.0.push(HashLifeOctree {
                    layer,
                    sub_octrees: [prev, prev, prev, prev, prev, prev, prev, prev],
                });
                result.1.push(Some(prev));
                result
            },
        );

        let octree_by_hash = octrees.iter().enumerate().fold(
            std::collections::HashMap::new(),
            |mut result, (i, x)| {
                result.insert(*x, i);
                result
            },
        );

        let root = vec![octrees.len() - 1];

        HashLife {
            octree_by_hash,
            octrees,
            result,
            root,
        }
    }

    fn foo(&mut self, zz: u32, yy: u32, xx: u32, octree: usize) -> usize {
        let current = self.get_value_at(xx, yy, zz, octree);
        let neighbors = cartesian_product(2, 2, 2)
            .map(|(x, y, z)| self.get_value_at(x as u32, y as u32, z as u32, octree));
        let neighbor_count: usize = neighbors.sum::<usize>() - current;
        if current != 0 && (neighbor_count == 2 || neighbor_count == 3) {
            1
        } else if current == 0 && neighbor_count == 3 {
            1
        } else {
            0
        }
    }

    fn intern(&mut self, tree: HashLifeOctree) -> usize {
        let next_index = self.octrees.len();
        let mut oughta_push = false;
        let result = *self.octree_by_hash.entry(tree).or_insert_with(|| {
            oughta_push = true;
            next_index
        });
        if oughta_push {
            self.octrees.push(tree);
            self.result.push(None);
        }
        result
    }

    fn get_result(&mut self, octree: usize) -> usize {
        if let Some(result) = self.result[octree] {
            return result;
        }
        let tree = self.octrees[octree];
        if tree.layer == 2 {
            let sub_octrees = [
                self.foo(1, 1, 2, octree),
                self.foo(1, 1, 1, octree),
                self.foo(1, 2, 2, octree),
                self.foo(1, 2, 1, octree),
                self.foo(2, 1, 2, octree),
                self.foo(2, 1, 1, octree),
                self.foo(2, 2, 2, octree),
                self.foo(2, 2, 1, octree),
            ];
            let result = self.intern(HashLifeOctree {
                layer: 1,
                sub_octrees,
            });
            self.result[octree] = Some(result);
            result
        } else {
            // sick
            let temp_box = (0..3)
                .flat_map(|zz| {
                    (0..3).flat_map(move |yy| {
                        (0..3).map(move |xx| {
                            let xis2: usize = (xx == 2).into();
                            let xaint0: usize = (xx != 0).into();
                            let yis2: usize = (yy == 2).into();
                            let yaint0: usize = (yy != 0).into();
                            let zis2: usize = (zz == 2).into();
                            let zaint0: usize = (zz != 0).into();
                            [
                                (
                                    zis2 * 4 + yis2 * 2 + xis2,
                                    (zz & 1) * 4 + (yy & 1) * 2 + xx & 1,
                                ),
                                (
                                    zis2 * 4 + yis2 * 2 + xaint0,
                                    (zz & 1) * 4 + (yy & 1) * 2 + (1 - xx & 1),
                                ),
                                (
                                    zis2 * 4 + yaint0 * 2 + xis2,
                                    (zz & 1) * 4 + (1 - yy & 1) * 2 + xx & 1,
                                ),
                                (
                                    zis2 * 4 + yaint0 * 2 + xaint0,
                                    (zz & 1) * 4 + (1 - yy & 1) * 2 + 1 - xx & 1,
                                ),
                                (
                                    zaint0 * 4 + yis2 * 2 + xis2,
                                    (1 - zz & 1) * 4 + (yy & 1) * 2 + xx & 1,
                                ),
                                (
                                    zaint0 * 4 + yis2 * 2 + xaint0,
                                    (1 - zz & 1) * 4 + (yy & 1) * 2 + (1 - xx & 1),
                                ),
                                (
                                    zaint0 * 4 + yaint0 * 2 + xis2,
                                    (1 - zz & 1) * 4 + (1 - yy & 1) * 2 + xx & 1,
                                ),
                                (
                                    zaint0 * 4 + yaint0 * 2 + xaint0,
                                    (1 - zz & 1) * 4 + (1 - yy & 1) * 2 + 1 - xx & 1,
                                ),
                            ]
                        })
                    })
                })
                .map(|deltas| {
                    self.intern(HashLifeOctree {
                        layer: tree.layer - 1,
                        sub_octrees: deltas
                            .iter()
                            .map(|&(a, b): &(usize, usize)| {
                                self.octrees[tree.sub_octrees[a]].sub_octrees[b]
                            })
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap(),
                    })
                })
                .collect::<Vec<_>>();

            let mut temp_box2 = vec![];
            for zz in 0..2 {
                for yy in 0..2 {
                    for xx in 0..2 {
                        temp_box2.push(self.intern(HashLifeOctree {
                            layer: tree.layer - 1,
                            sub_octrees: [
                                temp_box[zz * 9 + yy * 3 + xx],
                                temp_box[zz * 9 + yy * 3 + xx + 1],
                                temp_box[zz * 9 + (yy + 1) * 3 + xx],
                                temp_box[zz * 9 + (yy + 1) * 3 + xx + 1],
                                temp_box[(zz + 1) * 9 + yy * 3 + xx],
                                temp_box[(zz + 1) * 9 + yy * 3 + xx + 1],
                                temp_box[(zz + 1) * 9 + (yy + 1) * 3 + xx],
                                temp_box[(zz + 1) * 9 + (yy + 1) * 3 + xx + 1],
                            ],
                        }));
                    }
                }
            }

            let sub_octrees = [
                self.get_result(temp_box2[0]),
                self.get_result(temp_box2[1]),
                self.get_result(temp_box2[2]),
                self.get_result(temp_box2[3]),
                self.get_result(temp_box2[4]),
                self.get_result(temp_box2[5]),
                self.get_result(temp_box2[6]),
                self.get_result(temp_box2[7]),
            ];

            let octree = self.intern(HashLifeOctree {
                layer: tree.layer - 1,
                sub_octrees,
            });

            octree
        }
    }

    fn get_value_at(&mut self, x: u32, y: u32, z: u32, octree: usize) -> usize {
        let tree = self.octrees[octree];
        let sh = 2u32.pow(tree.layer as u32 - 1);
        if tree.layer == 1 {
            tree.sub_octrees[(z * 4 + y * 2 + x) as usize]
        } else if x < sh {
            if y < sh {
                if z < sh {
                    self.get_value_at(x, y, z, tree.sub_octrees[DNW])
                } else {
                    self.get_value_at(x, y, z - sh, tree.sub_octrees[UNW])
                }
            } else {
                if z < sh {
                    self.get_value_at(x, y - sh, z, tree.sub_octrees[DSW])
                } else {
                    self.get_value_at(x, y - sh, z - sh, tree.sub_octrees[USW])
                }
            }
        } else {
            if y < sh {
                if z < sh {
                    self.get_value_at(x - sh, y, z, tree.sub_octrees[DNE])
                } else {
                    self.get_value_at(x - sh, y, z - sh, tree.sub_octrees[UNE])
                }
            } else {
                if z < sh {
                    self.get_value_at(x - sh, y - sh, z, tree.sub_octrees[DSE])
                } else {
                    self.get_value_at(x - sh, y - sh, z - sh, tree.sub_octrees[USE])
                }
            }
        }
    }

    fn show_octree(&mut self, octree: usize, x: u32, y: u32, z: u32, w: u32, h: u32) -> String {
        let mut rows = vec![];
        for yy in y..y + h {
            let mut row = vec![];
            for xx in x..x + w {
                row.push(if self.get_value_at(xx, yy, z, octree) != 0 {
                    "#"
                } else {
                    "."
                });
            }
            rows.push(row.join(""));
        }
        rows.join("\n")
    }

    fn show(&mut self, generation: u32, x: u32, y: u32, z: u32, w: u32, h: u32) -> String {
        let size = 2u32.pow(self.octrees[self.root[self.root.len() - 1]].layer as u32);
        if x < generation
            || y < generation
            || z < generation
            || x + w >= size - generation
            || y + w >= size - generation
            || z + 1 >= size - generation
        {
            panic!();
        }
        let mut rows = vec![];
        for yy in y..y + h {
            let mut row = vec![];
            for xx in x..x + w {
                row.push(if self.get_value_at(xx, yy, z, octree) != 0 {
                    "#"
                } else {
                    "."
                });
            }
            rows.push(row.join(""));
        }
        rows.join("\n")
    }

    fn set_in(&mut self, x: usize, y: usize, z: usize, octree: usize) -> usize {
        println!(
            "set_in {} {} {}, layer={}",
            x, y, z, self.octrees[octree].layer
        );
        if self.octrees[octree].layer == 1 {
            let mut sub_octrees = self.octrees[octree].sub_octrees;
            sub_octrees[z * 4 + y * 2 + x] = 1;
            let result = self.intern(HashLifeOctree {
                layer: self.octrees[octree].layer,
                sub_octrees,
            });
            println!("prev 2x2x2:");
            println!("{}", self.show_octree(octree, 0, 0, 1, 2, 2));
            println!("{}", self.show_octree(octree, 0, 0, 0, 2, 2));

            println!("resulting 2x2x2:");
            println!("{}", self.show_octree(result, 0, 0, 1, 2, 2));
            println!("{}", self.show_octree(result, 0, 0, 0, 2, 2));
            result
        } else {
            let sh = 2usize.pow(self.octrees[octree].layer as u32 - 1);
            let sub_octrees = [
                if z < sh && y < sh && x < sh {
                    self.set_in(x, y, z, self.octrees[octree].sub_octrees[0])
                } else {
                    self.octrees[octree].sub_octrees[0]
                },
                if z < sh && y < sh && x >= sh {
                    self.set_in(x - sh, y, z, self.octrees[octree].sub_octrees[1])
                } else {
                    self.octrees[octree].sub_octrees[1]
                },
                if z < sh && y >= sh && x < sh {
                    self.set_in(x, y - sh, z, self.octrees[octree].sub_octrees[2])
                } else {
                    self.octrees[octree].sub_octrees[2]
                },
                if z < sh && y >= sh && x >= sh {
                    self.set_in(x - sh, y - sh, z, self.octrees[octree].sub_octrees[3])
                } else {
                    self.octrees[octree].sub_octrees[3]
                },
                if z >= sh && y < sh && x < sh {
                    self.set_in(x, y, z - sh, self.octrees[octree].sub_octrees[4])
                } else {
                    self.octrees[octree].sub_octrees[4]
                },
                if z >= sh && y < sh && x >= sh {
                    self.set_in(x - sh, y, z - sh, self.octrees[octree].sub_octrees[5])
                } else {
                    self.octrees[octree].sub_octrees[5]
                },
                if z >= sh && y >= sh && x < sh {
                    self.set_in(x, y - sh, z - sh, self.octrees[octree].sub_octrees[6])
                } else {
                    self.octrees[octree].sub_octrees[6]
                },
                if z >= sh && y >= sh && x >= sh {
                    self.set_in(x - sh, y - sh, z - sh, self.octrees[octree].sub_octrees[7])
                } else {
                    self.octrees[octree].sub_octrees[7]
                },
            ];
            self.intern(HashLifeOctree {
                layer: self.octrees[octree].layer,
                sub_octrees,
            })
        }
    }
    fn set(&mut self, x: usize, y: usize, z: usize) {
        let new_root = self.set_in(x, y, z, self.root[self.root.len() - 1]);
        self.root.push(new_root);
    }
}

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        17
    }

    fn main1(_input: &String) -> String {
        let input = ".#.
..#
###";

        let mut hash_life = HashLife::new(6 * 4 + 3);

        for (y, line) in input.lines().enumerate() {
            for (x, byte) in line.bytes().enumerate() {
                println!(
                    "===\n{}\n===",
                    hash_life.show_octree(
                        hash_life.root[hash_life.root.len() - 1],
                        16,
                        16,
                        0,
                        3,
                        3
                    )
                );
                match byte {
                    b'#' => hash_life.set(x + 16, y + 16, 0),
                    _ => {}
                }
            }
        }

        println!(
            "===\n{}\n===",
            hash_life.show_octree(hash_life.root[hash_life.root.len() - 1], 16, 16, 0, 3, 3)
        );

        format!("{}", "wrong")
    }

    fn main2(_input: &String) -> String {
        format!("{}", "wrong")
    }
}
